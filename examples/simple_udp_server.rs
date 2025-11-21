//! Simple VSTP UDP Server - Minimal example
//!
//! Run with: cargo run --example simple_udp_server

use std::net::SocketAddr;
use vstp::types::{Frame, FrameType};
use vstp::udp::VstpUdpServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚀 Starting VSTP UDP Server on 127.0.0.1:8080");
    println!("   Waiting for clients to send messages...\n");

    // Bind UDP server to localhost:8080
    let server = VstpUdpServer::bind("127.0.0.1:8080").await?;

    // Run server with handler
    server
        .run(|addr: SocketAddr, frame: Frame| async move {
            match frame.typ {
                FrameType::Hello => {
                    println!("✅ Client {} connected (sent HELLO)", addr);
                }
                FrameType::Data => {
                    let message = String::from_utf8_lossy(&frame.payload);
                    println!("📨 Message from {}: {}", addr, message);
                }
                FrameType::Bye => {
                    println!("👋 Client {} disconnected (sent BYE)", addr);
                }
                _ => {
                    println!("📦 Received {:?} from {}", frame.typ, addr);
                }
            }
        })
        .await?;

    Ok(())
}
