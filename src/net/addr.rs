use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

/// Enhanced address type with additional functionality
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    inner: SocketAddr,
    hostname: Option<String>,
}

impl Address {
    /// Create a new address from a socket address
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            inner: addr,
            hostname: None,
        }
    }

    /// Create a new address with hostname
    pub fn with_hostname(addr: SocketAddr, hostname: impl Into<String>) -> Self {
        Self {
            inner: addr,
            hostname: Some(hostname.into()),
        }
    }

    /// Get the socket address
    pub fn socket_addr(&self) -> SocketAddr {
        self.inner
    }

    /// Get the hostname if available
    pub fn hostname(&self) -> Option<&str> {
        self.hostname.as_deref()
    }

    /// Get the IP address
    pub fn ip(&self) -> IpAddr {
        self.inner.ip()
    }

    /// Get the port number
    pub fn port(&self) -> u16 {
        self.inner.port()
    }

    /// Check if this is an IPv4 address
    pub fn is_ipv4(&self) -> bool {
        self.inner.is_ipv4()
    }

    /// Check if this is an IPv6 address
    pub fn is_ipv6(&self) -> bool {
        self.inner.is_ipv6()
    }
}

impl FromStr for Address {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let addr = SocketAddr::from_str(s)?;
        Ok(Self::new(addr))
    }
}

impl From<SocketAddr> for Address {
    fn from(addr: SocketAddr) -> Self {
        Self::new(addr)
    }
}

impl From<Address> for SocketAddr {
    fn from(addr: Address) -> Self {
        addr.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_creation() {
        let addr: Address = "127.0.0.1:8080".parse().unwrap();
        assert_eq!(addr.port(), 8080);
        assert!(addr.is_ipv4());
        assert_eq!(addr.hostname(), None);
    }

    #[test]
    fn test_address_with_hostname() {
        let socket_addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let addr = Address::with_hostname(socket_addr, "localhost");
        assert_eq!(addr.hostname(), Some("localhost"));
    }

    #[test]
    fn test_address_conversion() {
        let socket_addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let addr = Address::from(socket_addr);
        let back: SocketAddr = addr.into();
        assert_eq!(socket_addr, back);
    }
}
