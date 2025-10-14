//! # VSTP - Vishu's Secure Transfer Protocol
//!
//! A general-purpose, binary, extensible application-layer protocol designed to be:
//!
//! * **Secure by default** on TCP (TLS 1.3)
//! * **Fast** on UDP (no TLS initially)
//! * **Minimal but extensible** with binary headers
//! * **Easy to implement** across languages
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use vstp::easy::{VstpClient, VstpServer};
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Message {
//!     content: String,
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), vstp::VstpError> {
//!     // Start a TCP server
//!     let server = VstpServer::bind_tcp("127.0.0.1:8080").await?;
//!     tokio::spawn(async move {
//!         server.serve(|msg: Message| async move {
//!             println!("Received: {}", msg.content);
//!             Ok(msg) // Echo the message back
//!         }).await
//!     });
//!
//!     // Connect a client
//!     let mut client = VstpClient::connect_tcp("127.0.0.1:8080").await?;
//!
//!     // Send a message
//!     let msg = Message { content: "Hello, VSTP!".to_string() };
//!     client.send(msg).await?;
//!
//!     // Receive the response
//!     let response: Message = client.receive().await?;
//!     println!("Got response: {}", response.content);
//!     Ok(())
//! }
//! ```

pub mod codec;
pub mod core;
pub mod transport;
pub mod security;
pub mod protocol;
pub mod utils;
pub mod net;

// Re-export commonly used types
pub use core::types::{Frame, FrameType, Header, Flags, VstpError};
pub use core::encoding::{encode_varint, decode_varint, varint_len};

// Re-export transport modules
pub use transport::tcp::{VstpTcpClient, VstpTcpServer};
pub use transport::udp::{VstpUdpClient, VstpUdpServer};

// Re-export easy-to-use API
pub mod easy;