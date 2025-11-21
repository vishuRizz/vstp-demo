//! Attack pattern definitions and threat classification

use serde::{Deserialize, Serialize};

/// Types of attack patterns that can be detected
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttackPattern {
    /// Packet theft / Sniffing attempt
    PacketTheft,
    /// Man-in-the-Middle attack
    ManInTheMiddle,
    /// Replay attack
    ReplayAttack,
    /// Protocol violation / Fuzzing
    ProtocolViolation,
    /// Unusual traffic pattern
    AnomalousTraffic,
    /// Connection flooding / DDoS
    ConnectionFlood,
    /// Unauthorized access attempt
    UnauthorizedAccess,
    /// Data exfiltration pattern
    DataExfiltration,
    /// Timing attack
    TimingAttack,
    /// Unknown suspicious activity
    SuspiciousActivity,
}

impl AttackPattern {
    pub fn description(&self) -> &'static str {
        match self {
            AttackPattern::PacketTheft => "Possible packet theft or network sniffing detected",
            AttackPattern::ManInTheMiddle => "Potential Man-in-the-Middle attack detected",
            AttackPattern::ReplayAttack => "Replay attack pattern detected",
            AttackPattern::ProtocolViolation => "Protocol violation or fuzzing attempt",
            AttackPattern::AnomalousTraffic => "Unusual traffic pattern detected",
            AttackPattern::ConnectionFlood => "Connection flooding or DDoS attempt",
            AttackPattern::UnauthorizedAccess => "Unauthorized access attempt",
            AttackPattern::DataExfiltration => "Possible data exfiltration pattern",
            AttackPattern::TimingAttack => "Timing-based attack detected",
            AttackPattern::SuspiciousActivity => "Suspicious activity detected",
        }
    }
}

/// Threat level classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ThreatLevel {
    /// No threat detected
    None,
    /// Low-level suspicious activity
    Low,
    /// Medium threat level
    Medium,
    /// High threat level - immediate action recommended
    High,
    /// Critical threat - immediate action required
    Critical,
}

impl ThreatLevel {
    pub fn from_score(score: f64) -> Self {
        match score {
            s if s >= 0.9 => ThreatLevel::Critical,
            s if s >= 0.7 => ThreatLevel::High,
            s if s >= 0.5 => ThreatLevel::Medium,
            s if s >= 0.3 => ThreatLevel::Low,
            _ => ThreatLevel::None,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ThreatLevel::None => "No threat detected",
            ThreatLevel::Low => "Low-level suspicious activity",
            ThreatLevel::Medium => "Medium threat - monitor closely",
            ThreatLevel::High => "High threat - consider blocking",
            ThreatLevel::Critical => "Critical threat - immediate action required",
        }
    }
}

/// Detected threat information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetection {
    pub pattern: AttackPattern,
    pub threat_level: ThreatLevel,
    pub confidence: f64, // 0.0 to 1.0
    pub description: String,
    pub session_id: Option<u128>,
    pub timestamp: u64,
    pub indicators: Vec<String>,
}

impl ThreatDetection {
    pub fn new(
        pattern: AttackPattern,
        threat_level: ThreatLevel,
        confidence: f64,
        description: String,
    ) -> Self {
        Self {
            pattern,
            threat_level,
            confidence: confidence.min(1.0).max(0.0),
            description,
            session_id: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            indicators: Vec::new(),
        }
    }

    pub fn with_session_id(mut self, session_id: u128) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub fn with_indicator(mut self, indicator: String) -> Self {
        self.indicators.push(indicator);
        self
    }
}
