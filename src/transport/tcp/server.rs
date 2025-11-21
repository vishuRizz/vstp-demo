use futures::SinkExt;
use std::future::Future;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio::sync::Mutex;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
use tracing::info;

use crate::core::types::{Frame, SessionId, VstpError};
use crate::codec::VstpFrameCodec as Codec;
use crate::security::ai::AnomalyDetector;

/// TCP connection handler
pub struct VstpTcpConnection {
    framed: Framed<TcpStream, Codec>,
    session_id: SessionId,
    peer_addr: std::net::SocketAddr,
}

impl VstpTcpConnection {
    /// Send a frame to the client
    pub async fn send(&mut self, frame: Frame) -> Result<(), VstpError> {
        self.framed.send(frame).await?;
        Ok(())
    }

    /// Receive a frame from the client
    pub async fn recv(&mut self) -> Result<Option<Frame>, VstpError> {
        let frame = self.framed.next().await.transpose()?;
        Ok(frame)
    }

    /// Get the peer address
    pub fn peer_addr(&self) -> std::net::SocketAddr {
        self.peer_addr
    }
}

/// TCP server for VSTP protocol
pub struct VstpTcpServer {
    listener: TcpListener,
    next_session_id: Arc<Mutex<u128>>,
}

impl VstpTcpServer {
    /// Bind to the specified address
    pub async fn bind(addr: impl ToSocketAddrs) -> Result<Self, VstpError> {
        let listener = TcpListener::bind(addr).await?;
        info!("VSTP TCP server bound to {}", listener.local_addr()?);

        Ok(Self {
            listener,
            next_session_id: Arc::new(Mutex::new(1)),
        })
    }

    /// Accept a new client connection
    pub async fn accept(&self) -> Result<VstpTcpConnection, VstpError> {
        let (socket, addr) = self.listener.accept().await?;
        let session_id = {
            let mut id_guard = self.next_session_id.lock().await;
            *id_guard += 1;
            *id_guard
        };

        info!("New connection from {} (session {})", addr, session_id);

        Ok(VstpTcpConnection {
            framed: Framed::new(socket, Codec::default()),
            session_id,
            peer_addr: addr,
        })
    }

    /// Get the local address this server is bound to
    pub fn local_addr(&self) -> Result<std::net::SocketAddr, VstpError> {
        self.listener.local_addr().map_err(|e| VstpError::Io(e))
    }

    /// Run the server with the provided handler function
    pub async fn run<F, Fut>(self, handler: F) -> Result<(), VstpError>
    where
        F: Fn(SessionId, Frame) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = ()> + Send,
    {
        self.run_with_detector(handler, None).await
    }

    /// Run the server with AI anomaly detection enabled
    pub async fn run_with_detector<F, Fut>(
        self,
        handler: F,
        detector: Option<Arc<AnomalyDetector>>,
    ) -> Result<(), VstpError>
    where
        F: Fn(SessionId, Frame) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = ()> + Send,
    {
        info!("VSTP TCP server starting...");

        loop {
            match self.accept().await {
                Ok(mut conn) => {
                    let handler = handler.clone();
                    let detector = detector.clone();
                    let session_id = conn.session_id;
                    let peer_addr = conn.peer_addr;

                    tokio::spawn(async move {
                        while let Ok(Some(frame)) = conn.recv().await {
                            // Run AI anomaly detection if enabled
                            if let Some(detector) = &detector {
                                let frame_size = std::mem::size_of_val(&frame) + frame.payload.len();
                                
                                match detector.analyze_frame(session_id, peer_addr, &frame, frame_size).await {
                                    Ok(threats) => {
                                        if !threats.is_empty() {
                                            tracing::warn!(
                                                "Detected {} threat(s) for session {}",
                                                threats.len(),
                                                session_id
                                            );
                                            
                                            // Check if session should be blocked
                                            if detector.is_blocked(session_id).await {
                                                tracing::error!("Session {} blocked due to security threat", session_id);
                                                break;
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        // Session blocked or error
                                        tracing::error!("Anomaly detection error for session {}: {}", session_id, e);
                                        break;
                                    }
                                }
                            }

                            // Process frame with handler
                            handler(session_id, frame).await;
                        }
                        info!("Session {} ended", session_id);
                        
                        // Cleanup connection from detector
                        if let Some(detector) = &detector {
                            detector.cleanup().await;
                        }
                    });
                }
                Err(e) => {
                    tracing::error!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}
