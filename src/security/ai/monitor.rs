//! Traffic monitoring and pattern collection for anomaly detection

use std::collections::{HashMap, VecDeque};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, warn};

use crate::core::types::{Frame, FrameType, SessionId};

/// Traffic statistics for a single connection/session
#[derive(Debug, Clone)]
pub struct ConnectionStats {
    pub session_id: SessionId,
    pub peer_addr: SocketAddr,
    pub frame_count: u64,
    pub byte_count: u64,
    pub frame_types: HashMap<FrameType, u64>,
    pub first_seen: Instant,
    pub last_seen: Instant,
    pub avg_frame_size: f64,
    pub max_frame_size: usize,
    pub min_frame_size: usize,
    pub inter_arrival_times: VecDeque<Duration>,
    pub crc_errors: u64,
    pub protocol_errors: u64,
    pub suspicious_flags: u64,
}

impl ConnectionStats {
    pub fn new(session_id: SessionId, peer_addr: SocketAddr) -> Self {
        let now = Instant::now();
        Self {
            session_id,
            peer_addr,
            frame_count: 0,
            byte_count: 0,
            frame_types: HashMap::new(),
            first_seen: now,
            last_seen: now,
            avg_frame_size: 0.0,
            max_frame_size: 0,
            min_frame_size: usize::MAX,
            inter_arrival_times: VecDeque::with_capacity(100),
            crc_errors: 0,
            protocol_errors: 0,
            suspicious_flags: 0,
        }
    }

    pub fn record_frame(&mut self, frame: &Frame, frame_size: usize) {
        self.frame_count += 1;
        self.byte_count += frame_size as u64;
        self.last_seen = Instant::now();

        // Track frame types
        *self.frame_types.entry(frame.typ).or_insert(0) += 1;

        // Update size statistics
        if frame_size > self.max_frame_size {
            self.max_frame_size = frame_size;
        }
        if frame_size < self.min_frame_size {
            self.min_frame_size = frame_size;
        }

        // Calculate running average
        self.avg_frame_size = (self.avg_frame_size * (self.frame_count - 1) as f64
            + frame_size as f64)
            / self.frame_count as f64;

        // Record inter-arrival time (if we have a previous timestamp)
        if self.frame_count > 1 {
            let elapsed = self.last_seen.duration_since(self.first_seen);
            if self.frame_count > 1 {
                let avg_time = elapsed.as_millis() as f64 / self.frame_count as f64;
                self.inter_arrival_times
                    .push_back(Duration::from_millis(avg_time as u64));
                if self.inter_arrival_times.len() > 100 {
                    self.inter_arrival_times.pop_front();
                }
            }
        }
    }

    pub fn record_error(&mut self, error_type: ErrorType) {
        match error_type {
            ErrorType::Crc => self.crc_errors += 1,
            ErrorType::Protocol => self.protocol_errors += 1,
            ErrorType::SuspiciousFlag => self.suspicious_flags += 1,
        }
    }

    pub fn get_connection_duration(&self) -> Duration {
        self.last_seen.duration_since(self.first_seen)
    }

    pub fn get_frames_per_second(&self) -> f64 {
        let duration = self.get_connection_duration();
        if duration.as_secs() > 0 {
            self.frame_count as f64 / duration.as_secs() as f64
        } else {
            self.frame_count as f64 / duration.as_secs_f64()
        }
    }

    pub fn get_bytes_per_second(&self) -> f64 {
        let duration = self.get_connection_duration();
        if duration.as_secs() > 0 {
            self.byte_count as f64 / duration.as_secs() as f64
        } else {
            self.byte_count as f64 / duration.as_secs_f64()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorType {
    Crc,
    Protocol,
    SuspiciousFlag,
}

/// Global traffic monitor that tracks all connections
pub struct TrafficMonitor {
    connections: Arc<RwLock<HashMap<SessionId, ConnectionStats>>>,
    global_stats: Arc<RwLock<GlobalStats>>,
    window_size: Duration,
}

#[derive(Debug, Clone)]
struct GlobalStats {
    total_frames: u64,
    total_bytes: u64,
    total_connections: u64,
    active_connections: usize,
    avg_frames_per_connection: f64,
    avg_bytes_per_connection: f64,
    peak_frames_per_second: f64,
    peak_bytes_per_second: f64,
}

impl TrafficMonitor {
    /// Create a new traffic monitor
    pub fn new(window_size: Duration) -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            global_stats: Arc::new(RwLock::new(GlobalStats {
                total_frames: 0,
                total_bytes: 0,
                total_connections: 0,
                active_connections: 0,
                avg_frames_per_connection: 0.0,
                avg_bytes_per_connection: 0.0,
                peak_frames_per_second: 0.0,
                peak_bytes_per_second: 0.0,
            })),
            window_size,
        }
    }

    /// Record a frame for analysis
    pub async fn record_frame(
        &self,
        session_id: SessionId,
        peer_addr: SocketAddr,
        frame: &Frame,
        frame_size: usize,
    ) {
        let mut connections = self.connections.write().await;

        let stats = connections.entry(session_id).or_insert_with(|| {
            debug!(
                "New connection tracked: session {} from {}",
                session_id, peer_addr
            );
            ConnectionStats::new(session_id, peer_addr)
        });

        stats.record_frame(frame, frame_size);

        // Calculate current rates before releasing the lock
        let fps = stats.get_frames_per_second();
        let bps = stats.get_bytes_per_second();
        let active_conn_count = connections.len();

        // Release connections lock
        drop(connections);

        // Update global stats
        let mut global = self.global_stats.write().await;
        global.total_frames += 1;
        global.total_bytes += frame_size as u64;
        global.active_connections = active_conn_count;

        if global.total_connections > 0 {
            global.avg_frames_per_connection =
                global.total_frames as f64 / global.total_connections as f64;
            global.avg_bytes_per_connection =
                global.total_bytes as f64 / global.total_connections as f64;
        }

        if fps > global.peak_frames_per_second {
            global.peak_frames_per_second = fps;
        }
        if bps > global.peak_bytes_per_second {
            global.peak_bytes_per_second = bps;
        }
    }

    /// Record an error for a connection
    pub async fn record_error(&self, session_id: SessionId, error_type: ErrorType) {
        let mut connections = self.connections.write().await;
        if let Some(stats) = connections.get_mut(&session_id) {
            stats.record_error(error_type);
            warn!(
                "Error recorded for session {}: {:?}",
                session_id, error_type
            );
        }
    }

    /// Get statistics for a specific connection
    pub async fn get_connection_stats(&self, session_id: SessionId) -> Option<ConnectionStats> {
        let connections = self.connections.read().await;
        connections.get(&session_id).cloned()
    }

    /// Get all active connections
    pub async fn get_all_connections(&self) -> Vec<ConnectionStats> {
        let connections = self.connections.read().await;
        connections.values().cloned().collect()
    }

    /// Get global statistics
    pub async fn get_global_stats(&self) -> GlobalStats {
        self.global_stats.read().await.clone()
    }

    /// Remove a connection (when it closes)
    pub async fn remove_connection(&self, session_id: SessionId) {
        let mut connections = self.connections.write().await;
        if connections.remove(&session_id).is_some() {
            debug!("Connection removed: session {}", session_id);

            let mut global = self.global_stats.write().await;
            global.total_connections += 1;
            global.active_connections = connections.len();
        }
    }

    /// Clean up old connections (older than window_size)
    pub async fn cleanup_old_connections(&self) {
        let now = Instant::now();
        let mut connections = self.connections.write().await;

        let mut to_remove = Vec::new();
        for (session_id, stats) in connections.iter() {
            if now.duration_since(stats.last_seen) > self.window_size {
                to_remove.push(*session_id);
            }
        }

        for session_id in to_remove {
            connections.remove(&session_id);
            debug!("Cleaned up old connection: session {}", session_id);
        }

        let mut global = self.global_stats.write().await;
        global.active_connections = connections.len();
    }
}

impl Default for TrafficMonitor {
    fn default() -> Self {
        Self::new(Duration::from_secs(300)) // 5 minute window
    }
}
