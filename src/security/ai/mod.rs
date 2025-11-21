//! AI-Powered Anomaly Detection System for VSTP
//!
//! This module provides intelligent threat detection capabilities including:
//! - Packet theft detection
//! - Man-in-the-Middle (MITM) attack detection
//! - Traffic pattern analysis
//! - Behavioral anomaly detection
//! - Real-time threat scoring

pub mod detector;
pub mod models;
pub mod monitor;
pub mod patterns;

pub use detector::AnomalyDetector;
pub use monitor::TrafficMonitor;
pub use patterns::{AttackPattern, ThreatLevel};
