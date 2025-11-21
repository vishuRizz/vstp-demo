//! Simple VSTP TCP Server - Minimal example
//!
//! Run with: cargo run --example simple_server

use vstp::tcp::VstpTcpServer;
use vstp::types::{Frame, FrameType, SessionId};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚀 Starting VSTP Server on 127.0.0.1:8080");
    println!("   Waiting for clients to connect...\n");

    // Bind server to localhost:8080
    let server = VstpTcpServer::bind("127.0.0.1:8080").await?;

    // Run server with handler
    server
        .run(|session_id: SessionId, frame: Frame| async move {
            match frame.typ {
                FrameType::Hello => {
                    println!("✅ Client {} connected (sent HELLO)", session_id);
                }
                FrameType::Data => {
                    let message = String::from_utf8_lossy(&frame.payload);
                    println!("📨 Message from client {}: {}", session_id, message);
                }
                FrameType::Bye => {
                    println!("👋 Client {} disconnected (sent BYE)", session_id);
                }
                _ => {
                    println!("📦 Received {:?} from client {}", frame.typ, session_id);
                }
            }
        })
        .await?;

    Ok(())
}
