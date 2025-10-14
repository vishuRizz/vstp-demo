use std::time::Duration;

/// TLS configuration for secure connections
#[derive(Debug, Clone)]
pub struct TlsConfig {
    /// Path to certificate file
    pub cert_path: Option<String>,
    /// Path to private key file
    pub key_path: Option<String>,
    /// Whether to verify client certificates
    pub verify_client: bool,
    /// TLS handshake timeout
    pub handshake_timeout: Duration,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            cert_path: None,
            key_path: None,
            verify_client: false,
            handshake_timeout: Duration::from_secs(30),
        }
    }
}

impl TlsConfig {
    /// Create a new TLS configuration with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the certificate path
    pub fn with_cert(mut self, path: impl Into<String>) -> Self {
        self.cert_path = Some(path.into());
        self
    }

    /// Set the private key path
    pub fn with_key(mut self, path: impl Into<String>) -> Self {
        self.key_path = Some(path.into());
        self
    }

    /// Enable or disable client certificate verification
    pub fn verify_client(mut self, verify: bool) -> Self {
        self.verify_client = verify;
        self
    }

    /// Set the handshake timeout
    pub fn handshake_timeout(mut self, timeout: Duration) -> Self {
        self.handshake_timeout = timeout;
        self
    }
}
