use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tracing::{debug, info};

use crate::core::frame::{encode_frame, try_decode_frame};
use crate::core::types::{Flags, Frame, FrameType, Header, VstpError, VSTP_VERSION};
use crate::security::ai::AnomalyDetector;
use crate::transport::udp::reassembly::{
    extract_fragment_info, ReassemblyManager, MAX_DATAGRAM_SIZE,
};

/// Configuration for UDP server
#[derive(Debug, Clone)]
pub struct UdpServerConfig {
    /// Whether to use CRC validation
    pub use_crc: bool,
    /// Whether to allow fragmentation
    pub allow_frag: bool,
    /// Maximum number of concurrent reassembly sessions
    pub max_reassembly_sessions: usize,
}

impl Default for UdpServerConfig {
    fn default() -> Self {
        Self {
            use_crc: true,
            allow_frag: true,
            max_reassembly_sessions: 1000,
        }
    }
}

/// VSTP UDP Server
pub struct VstpUdpServer {
    socket: UdpSocket,
    config: UdpServerConfig,
    reassembly: ReassemblyManager,
    next_session_id: Arc<Mutex<u128>>,
}

impl VstpUdpServer {
    /// Run the server with a handler function
    pub async fn run<F, Fut>(&self, handler: F) -> Result<(), VstpError>
    where
        F: Fn(SocketAddr, Frame) -> Fut + Send + Sync + Clone + 'static,
        Fut: std::future::Future<Output = ()> + Send,
    {
        self.run_with_detector(handler, None).await
    }

    /// Run the server with AI anomaly detection enabled
    pub async fn run_with_detector<F, Fut>(
        &self,
        handler: F,
        detector: Option<Arc<AnomalyDetector>>,
    ) -> Result<(), VstpError>
    where
        F: Fn(SocketAddr, Frame) -> Fut + Send + Sync + Clone + 'static,
        Fut: std::future::Future<Output = ()> + Send,
    {
        info!("Starting UDP server...");

        loop {
            match self.recv().await {
                Ok((frame, addr)) => {
                    let handler = handler.clone();
                    let detector = detector.clone();

                    // Create a session ID for UDP (based on address)
                    let session_id = {
                        let mut hasher = DefaultHasher::new();
                        addr.hash(&mut hasher);
                        hasher.finish() as u128
                    };

                    let frame_size = std::mem::size_of_val(&frame) + frame.payload.len();

                    tokio::spawn(async move {
                        // Run AI anomaly detection if enabled
                        if let Some(detector) = &detector {
                            match detector
                                .analyze_frame(session_id, addr, &frame, frame_size)
                                .await
                            {
                                Ok(threats) => {
                                    if !threats.is_empty() {
                                        tracing::warn!(
                                            "Detected {} threat(s) from {}",
                                            threats.len(),
                                            addr
                                        );
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("Anomaly detection error from {}: {}", addr, e);
                                    return; // Skip processing if blocked
                                }
                            }
                        }

                        // Process frame with handler
                        handler(addr, frame).await;
                    });
                }
                Err(e) => {
                    debug!("Error receiving frame: {}", e);
                    continue;
                }
            }
        }
    }

    /// Create a new UDP server bound to the specified address
    pub async fn bind(addr: &str) -> Result<Self, VstpError> {
        let socket = UdpSocket::bind(addr).await?;
        info!("VSTP UDP server bound to {}", addr);

        Ok(Self {
            socket,
            config: UdpServerConfig::default(),
            reassembly: ReassemblyManager::new(),
            next_session_id: Arc::new(Mutex::new(1)),
        })
    }

    /// Create a new UDP server with custom configuration
    pub async fn bind_with_config(addr: &str, config: UdpServerConfig) -> Result<Self, VstpError> {
        let socket = UdpSocket::bind(addr).await?;
        info!("VSTP UDP server bound to {} with custom config", addr);

        Ok(Self {
            socket,
            config,
            reassembly: ReassemblyManager::new(),
            next_session_id: Arc::new(Mutex::new(1)),
        })
    }

    /// Get the local address this server is bound to
    pub fn local_addr(&self) -> Result<SocketAddr, VstpError> {
        self.socket.local_addr().map_err(|e| VstpError::Io(e))
    }

    /// Send a frame to a specific address
    pub async fn send(&self, frame: Frame, dest: SocketAddr) -> Result<(), VstpError> {
        let encoded = encode_frame(&frame)?;
        self.socket.send_to(&encoded, dest).await?;
        Ok(())
    }

    /// Receive a frame from any client
    pub async fn recv(&self) -> Result<(Frame, SocketAddr), VstpError> {
        let mut buf = vec![0u8; MAX_DATAGRAM_SIZE * 2]; // Extra space for headers

        loop {
            let (len, from_addr) = self.socket.recv_from(&mut buf).await?;
            let data = &buf[..len];
            debug!("Received {} bytes from {}", len, from_addr);

            // Try to decode the frame
            let mut buf = bytes::BytesMut::from(data);
            match try_decode_frame(&mut buf, 65536) {
                Ok(Some(frame)) => {
                    // Check if this is a fragmented frame
                    if let Some(fragment) = extract_fragment_info(&frame) {
                        // Handle fragmentation
                        if let Some(assembled_data) =
                            self.reassembly.add_fragment(from_addr, fragment).await?
                        {
                            // Reassemble the complete frame
                            let mut complete_frame = frame;
                            complete_frame.payload = assembled_data;
                            // Remove fragment headers
                            complete_frame.headers.retain(|h| {
                                h.key != b"frag-id"
                                    && h.key != b"frag-index"
                                    && h.key != b"frag-total"
                            });

                            // Send ACK if requested
                            if complete_frame.flags.contains(Flags::REQ_ACK) {
                                if let Some(msg_id) = self.extract_msg_id(&complete_frame) {
                                    let _ = self.send_ack(msg_id, from_addr).await;
                                }
                            }

                            return Ok((complete_frame, from_addr));
                        }
                        // Fragment received, continue waiting for more
                        continue;
                    } else {
                        // Send ACK if requested
                        if frame.flags.contains(Flags::REQ_ACK) {
                            if let Some(msg_id) = self.extract_msg_id(&frame) {
                                let _ = self.send_ack(msg_id, from_addr).await;
                            }
                        }

                        return Ok((frame, from_addr));
                    }
                }
                Ok(None) => continue, // Incomplete frame
                Err(_) => continue,   // Invalid frame
            }
        }
    }

    /// Extract message ID from frame headers
    fn extract_msg_id(&self, frame: &Frame) -> Option<u64> {
        for header in &frame.headers {
            if header.key == b"msg-id" {
                if let Ok(msg_id) = std::str::from_utf8(&header.value).ok()?.parse::<u64>() {
                    return Some(msg_id);
                }
            }
        }
        None
    }

    /// Send an ACK for a received message
    async fn send_ack(&self, msg_id: u64, dest: SocketAddr) -> Result<(), VstpError> {
        let ack_frame = Frame {
            version: VSTP_VERSION,
            typ: FrameType::Ack,
            flags: Flags::empty(),
            headers: vec![Header {
                key: b"msg-id".to_vec(),
                value: msg_id.to_string().into_bytes(),
            }],
            payload: Vec::new(),
        };

        self.send(ack_frame, dest).await
    }

    /// Get the number of active reassembly sessions
    pub async fn reassembly_session_count(&self) -> usize {
        self.reassembly.session_count().await
    }
}
