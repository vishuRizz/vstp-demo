//! Main anomaly detection engine

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, warn, error};

use crate::core::types::{Frame, SessionId, VstpError};

use super::monitor::{TrafficMonitor, ErrorType};
use super::models::BaselineModel;
use super::patterns::{ThreatDetection, ThreatLevel};

/// Configuration for the anomaly detector
#[derive(Debug, Clone)]
pub struct DetectorConfig {
    /// Enable real-time detection
    pub enabled: bool,
    /// Minimum confidence threshold for reporting threats
    pub min_confidence: f64,
    /// Auto-block on critical threats
    pub auto_block_critical: bool,
    /// Learning mode (collects data without blocking)
    pub learning_mode: bool,
    /// Minimum samples before detection starts
    pub min_samples: u64,
}

impl Default for DetectorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            min_confidence: 0.5,
            auto_block_critical: false,
            learning_mode: false,
            min_samples: 10,
        }
    }
}

/// Main anomaly detection engine
pub struct AnomalyDetector {
    monitor: Arc<TrafficMonitor>,
    baseline: Arc<RwLock<BaselineModel>>,
    config: DetectorConfig,
    threat_history: Arc<RwLock<Vec<ThreatDetection>>>,
    blocked_sessions: Arc<RwLock<Vec<SessionId>>>,
}

impl AnomalyDetector {
    /// Create a new anomaly detector
    pub fn new(config: DetectorConfig) -> Self {
        Self {
            monitor: Arc::new(TrafficMonitor::default()),
            baseline: Arc::new(RwLock::new(BaselineModel::new())),
            config,
            threat_history: Arc::new(RwLock::new(Vec::new())),
            blocked_sessions: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(DetectorConfig::default())
    }

    /// Analyze a frame for anomalies
    pub async fn analyze_frame(
        &self,
        session_id: SessionId,
        peer_addr: std::net::SocketAddr,
        frame: &Frame,
        frame_size: usize,
    ) -> Result<Vec<ThreatDetection>, VstpError> {
        if !self.config.enabled {
            return Ok(Vec::new());
        }

        // Check if session is blocked
        {
            let blocked = self.blocked_sessions.read().await;
            if blocked.contains(&session_id) {
                return Err(VstpError::Protocol(
                    format!("Session {} is blocked due to security threat", session_id)
                ));
            }
        }

        // Record frame in monitor
        self.monitor.record_frame(session_id, peer_addr, frame, frame_size).await;

        // Get connection stats
        let stats = match self.monitor.get_connection_stats(session_id).await {
            Some(s) => s,
            None => {
                debug!("No stats available for session {}", session_id);
                return Ok(Vec::new());
            }
        };

        // Update baseline model
        {
            let mut baseline = self.baseline.write().await;
            baseline.update_baseline(&stats);
        }

        // Detect anomalies
        let baseline = self.baseline.read().await;
        let mut threats = baseline.detect_anomalies(&stats);

        // Filter by confidence threshold
        threats.retain(|t| {
            t.confidence >= self.config.min_confidence
        });

        // Add session ID to threats
        for threat in threats.iter_mut() {
            threat.session_id = Some(session_id);
        }

        // Log and store threats
        for threat in &threats {
            match threat.threat_level {
                ThreatLevel::Critical => {
                    error!(
                        "🚨 CRITICAL THREAT DETECTED: {:?} - {} (confidence: {:.2}%)",
                        threat.pattern,
                        threat.description,
                        threat.confidence * 100.0
                    );
                    
                    if self.config.auto_block_critical {
                        self.block_session(session_id).await;
                    }
                }
                ThreatLevel::High => {
                    warn!(
                        "⚠️  HIGH THREAT: {:?} - {} (confidence: {:.2}%)",
                        threat.pattern,
                        threat.description,
                        threat.confidence * 100.0
                    );
                }
                ThreatLevel::Medium => {
                    warn!(
                        "⚠️  MEDIUM THREAT: {:?} - {} (confidence: {:.2}%)",
                        threat.pattern,
                        threat.description,
                        threat.confidence * 100.0
                    );
                }
                _ => {
                    debug!(
                        "Threat detected: {:?} - {} (confidence: {:.2}%)",
                        threat.pattern,
                        threat.description,
                        threat.confidence * 100.0
                    );
                }
            }

            // Store in history
            let mut history = self.threat_history.write().await;
            history.push(threat.clone());
            if history.len() > 1000 {
                history.remove(0);
            }
        }

        Ok(threats)
    }

    /// Record an error for analysis
    pub async fn record_error(&self, session_id: SessionId, error_type: ErrorType) {
        self.monitor.record_error(session_id, error_type).await;
    }

    /// Block a session
    pub async fn block_session(&self, session_id: SessionId) {
        let mut blocked = self.blocked_sessions.write().await;
        if !blocked.contains(&session_id) {
            blocked.push(session_id);
            warn!("Session {} has been blocked due to security threat", session_id);
        }
    }

    /// Unblock a session
    pub async fn unblock_session(&self, session_id: SessionId) {
        let mut blocked = self.blocked_sessions.write().await;
        blocked.retain(|&id| id != session_id);
        debug!("Session {} has been unblocked", session_id);
    }

    /// Check if a session is blocked
    pub async fn is_blocked(&self, session_id: SessionId) -> bool {
        let blocked = self.blocked_sessions.read().await;
        blocked.contains(&session_id)
    }

    /// Get threat history
    pub async fn get_threat_history(&self, limit: usize) -> Vec<ThreatDetection> {
        let history = self.threat_history.read().await;
        history.iter().rev().take(limit).cloned().collect()
    }

    /// Get recent threats for a session
    pub async fn get_session_threats(&self, session_id: SessionId) -> Vec<ThreatDetection> {
        let history = self.threat_history.read().await;
        history.iter()
            .rev()
            .filter(|t| t.session_id == Some(session_id))
            .take(10)
            .cloned()
            .collect()
    }

    /// Get connection statistics
    pub async fn get_connection_stats(&self, session_id: SessionId) -> Option<super::monitor::ConnectionStats> {
        self.monitor.get_connection_stats(session_id).await
    }

    /// Cleanup old data
    pub async fn cleanup(&self) {
        self.monitor.cleanup_old_connections().await;
    }
}

impl Default for AnomalyDetector {
    fn default() -> Self {
        Self::default()
    }
}

