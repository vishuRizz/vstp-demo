//! Simple VSTP UDP Client - Minimal example
//! 
//! Run with: cargo run --example simple_udp_client

use std::io;
use std::net::SocketAddr;
use vstp::udp::VstpUdpClient;
use vstp::types::{Frame, FrameType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr: SocketAddr = "127.0.0.1:8080".parse()?;
    
    println!("🔌 Connecting to VSTP UDP Server at {}...", server_addr);
    
    // Bind UDP client (can use any port)
    let client = VstpUdpClient::bind("127.0.0.1:0").await?;
    println!("✅ UDP Client ready!\n");
    
    // Send HELLO
    let hello = Frame::new(FrameType::Hello);
    client.send(hello, server_addr).await?;
    println!("👋 Sent HELLO to server");
    
    // Send messages
    loop {
        println!("\nEnter a message (or 'quit' to exit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "quit" || input == "exit" {
            // Send BYE
            let bye = Frame::new(FrameType::Bye);
            client.send(bye, server_addr).await?;
            break;
        }
        
        if !input.is_empty() {
            let data = Frame::new(FrameType::Data)
                .with_payload(input.as_bytes().to_vec());
            client.send(data, server_addr).await?;
            println!("📤 Sent: {}", input);
        }
    }
    
    println!("\n👋 Disconnected from server");
    
    Ok(())
}

