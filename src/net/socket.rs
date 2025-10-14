use std::net::SocketAddr;
use tokio::net::{TcpSocket, UdpSocket};

/// A unified socket interface for TCP and UDP
#[derive(Debug)]
pub enum Socket {
    Tcp(TcpSocket),
    Udp(UdpSocket),
}

impl Socket {
    /// Create a new TCP socket
    pub fn tcp() -> std::io::Result<Self> {
        Ok(Socket::Tcp(TcpSocket::new_v4()?))
    }

    /// Create a new UDP socket
    pub async fn udp() -> std::io::Result<Self> {
        Ok(Socket::Udp(UdpSocket::bind("0.0.0.0:0").await?))
    }

    /// Bind the socket to an address
    pub async fn bind(&self, addr: SocketAddr) -> std::io::Result<()> {
        match self {
            Socket::Tcp(socket) => socket.bind(addr),
            Socket::Udp(_) => Ok(()), // UDP socket is already bound
        }
    }

    /// Set the socket's send buffer size
    pub fn set_send_buffer_size(&self, size: u32) -> std::io::Result<()> {
        match self {
            Socket::Tcp(socket) => socket.set_send_buffer_size(size),
            Socket::Udp(_) => Ok(()), // Not supported for UDP
        }
    }

    /// Set the socket's receive buffer size
    pub fn set_recv_buffer_size(&self, size: u32) -> std::io::Result<()> {
        match self {
            Socket::Tcp(socket) => socket.set_recv_buffer_size(size),
            Socket::Udp(_) => Ok(()), // Not supported for UDP
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_socket_creation() {
        let tcp = Socket::tcp().unwrap();
        let udp = Socket::udp().await.unwrap();

        match tcp {
            Socket::Tcp(_) => (),
            _ => panic!("Expected TCP socket"),
        }

        match udp {
            Socket::Udp(_) => (),
            _ => panic!("Expected UDP socket"),
        }
    }

    #[tokio::test]
    async fn test_socket_bind() {
        let tcp = Socket::tcp().unwrap();
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        tcp.bind(addr).await.unwrap();
    }
}
