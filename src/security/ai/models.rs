//! ML models and statistical analysis for anomaly detection

use std::collections::VecDeque;
use std::time::Duration;

use super::monitor::ConnectionStats;
use super::patterns::{AttackPattern, ThreatDetection, ThreatLevel};

/// Statistical model for baseline behavior
#[derive(Debug, Clone)]
pub struct BaselineModel {
    // Normal traffic characteristics
    avg_frame_size: f64,
    avg_frames_per_second: f64,
    avg_bytes_per_second: f64,
    avg_inter_arrival_time: Duration,

    // Standard deviations for anomaly detection
    frame_size_std: f64,
    frames_per_second_std: f64,
    bytes_per_second_std: f64,

    // Sample count for learning
    sample_count: u64,

    // Historical data for pattern recognition
    historical_patterns: VecDeque<ConnectionStats>,
    max_history: usize,
}

impl BaselineModel {
    pub fn new() -> Self {
        Self {
            avg_frame_size: 0.0,
            avg_frames_per_second: 0.0,
            avg_bytes_per_second: 0.0,
            avg_inter_arrival_time: Duration::from_millis(100),
            frame_size_std: 0.0,
            frames_per_second_std: 0.0,
            bytes_per_second_std: 0.0,
            sample_count: 0,
            historical_patterns: VecDeque::with_capacity(1000),
            max_history: 1000,
        }
    }

    /// Update the baseline with new connection statistics
    pub fn update_baseline(&mut self, stats: &ConnectionStats) {
        self.sample_count += 1;

        // Update historical patterns
        if self.historical_patterns.len() >= self.max_history {
            self.historical_patterns.pop_front();
        }
        self.historical_patterns.push_back(stats.clone());

        // Calculate running averages
        let n = self.sample_count as f64;

        // Frame size
        let old_avg = self.avg_frame_size;
        self.avg_frame_size = (old_avg * (n - 1.0) + stats.avg_frame_size) / n;

        // Frames per second
        let fps = stats.get_frames_per_second();
        let old_fps_avg = self.avg_frames_per_second;
        self.avg_frames_per_second = (old_fps_avg * (n - 1.0) + fps) / n;

        // Bytes per second
        let bps = stats.get_bytes_per_second();
        let old_bps_avg = self.avg_bytes_per_second;
        self.avg_bytes_per_second = (old_bps_avg * (n - 1.0) + bps) / n;

        // Calculate standard deviations (simplified)
        if n > 1.0 {
            let frame_size_diff = (stats.avg_frame_size - self.avg_frame_size).abs();
            self.frame_size_std = (self.frame_size_std * (n - 1.0) + frame_size_diff) / n;

            let fps_diff = (fps - self.avg_frames_per_second).abs();
            self.frames_per_second_std = (self.frames_per_second_std * (n - 1.0) + fps_diff) / n;

            let bps_diff = (bps - self.avg_bytes_per_second).abs();
            self.bytes_per_second_std = (self.bytes_per_second_std * (n - 1.0) + bps_diff) / n;
        }
    }

    /// Detect anomalies in connection statistics
    pub fn detect_anomalies(&self, stats: &ConnectionStats) -> Vec<ThreatDetection> {
        let mut threats = Vec::new();

        // Check for anomalous frame sizes
        if self.sample_count > 10 {
            let frame_size_z_score = if self.frame_size_std > 0.0 {
                (stats.avg_frame_size - self.avg_frame_size).abs() / self.frame_size_std
            } else {
                0.0
            };

            if frame_size_z_score > 3.0 {
                threats.push(ThreatDetection::new(
                    AttackPattern::AnomalousTraffic,
                    ThreatLevel::Medium,
                    0.6,
                    format!(
                        "Unusual frame size detected: {:.2} (baseline: {:.2}, z-score: {:.2})",
                        stats.avg_frame_size, self.avg_frame_size, frame_size_z_score
                    ),
                ));
            }
        }

        // Check for traffic flooding
        let fps = stats.get_frames_per_second();
        if self.sample_count > 10 && self.frames_per_second_std > 0.0 {
            let fps_z_score = (fps - self.avg_frames_per_second).abs() / self.frames_per_second_std;

            if fps_z_score > 4.0 && fps > self.avg_frames_per_second {
                threats.push(
                    ThreatDetection::new(
                        AttackPattern::ConnectionFlood,
                        ThreatLevel::High,
                        0.8,
                        format!(
                            "Traffic flood detected: {:.2} fps (baseline: {:.2}, z-score: {:.2})",
                            fps, self.avg_frames_per_second, fps_z_score
                        ),
                    )
                    .with_indicator(format!("Frames per second: {:.2}", fps)),
                );
            }
        }

        // Check for packet theft indicators (unusual inter-arrival patterns)
        if stats.inter_arrival_times.len() > 10 {
            let mut suspicious_timing = false;
            let mut consistent_intervals = true;
            let first_interval = stats.inter_arrival_times[0];

            for interval in stats.inter_arrival_times.iter().skip(1) {
                let diff = if first_interval > *interval {
                    first_interval.as_millis() as f64 - interval.as_millis() as f64
                } else {
                    interval.as_millis() as f64 - first_interval.as_millis() as f64
                };

                if diff > first_interval.as_millis() as f64 * 0.5 {
                    consistent_intervals = false;
                }

                // Very consistent intervals might indicate packet capture/replay
                if diff < first_interval.as_millis() as f64 * 0.1 {
                    suspicious_timing = true;
                }
            }

            if suspicious_timing && consistent_intervals {
                threats.push(
                    ThreatDetection::new(
                        AttackPattern::PacketTheft,
                        ThreatLevel::High,
                        0.75,
                        "Suspicious timing pattern detected - possible packet capture/replay"
                            .to_string(),
                    )
                    .with_indicator("Highly consistent inter-arrival times".to_string()),
                );
            }
        }

        // Check for high error rates (possible MITM or protocol manipulation)
        let error_rate = if stats.frame_count > 0 {
            (stats.crc_errors + stats.protocol_errors) as f64 / stats.frame_count as f64
        } else {
            0.0
        };

        if error_rate > 0.1 {
            threats.push(
                ThreatDetection::new(
                    AttackPattern::ManInTheMiddle,
                    ThreatLevel::High,
                    0.7,
                    format!(
                        "High error rate detected: {:.2}% (possible MITM)",
                        error_rate * 100.0
                    ),
                )
                .with_indicator(format!("Error rate: {:.2}%", error_rate * 100.0)),
            );
        }

        // Check for unusual frame type distributions
        if stats.frame_types.len() > 0 {
            let data_frames = stats
                .frame_types
                .get(&crate::core::types::FrameType::Data)
                .copied()
                .unwrap_or(0);
            let data_ratio = if stats.frame_count > 0 {
                data_frames as f64 / stats.frame_count as f64
            } else {
                0.0
            };

            // Very high data frame ratio might indicate data exfiltration
            if data_ratio > 0.95 && stats.frame_count > 100 {
                threats.push(
                    ThreatDetection::new(
                        AttackPattern::DataExfiltration,
                        ThreatLevel::Medium,
                        0.65,
                        format!("Unusual data frame ratio: {:.2}%", data_ratio * 100.0),
                    )
                    .with_indicator("High data frame percentage".to_string()),
                );
            }
        }

        threats
    }

    /// Get baseline statistics
    pub fn get_baseline(&self) -> (f64, f64, f64) {
        (
            self.avg_frame_size,
            self.avg_frames_per_second,
            self.avg_bytes_per_second,
        )
    }
}

impl Default for BaselineModel {
    fn default() -> Self {
        Self::new()
    }
}
