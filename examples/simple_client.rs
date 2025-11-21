//! Simple VSTP TCP Client - Minimal example
//! 
//! Run with: cargo run --example simple_client

use std::io;
use vstp::tcp::VstpTcpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔌 Connecting to VSTP Server at 127.0.0.1:8080...");
    
    // Connect to server
    let mut client = VstpTcpClient::connect("127.0.0.1:8080").await?;
    println!("✅ Connected to server!\n");
    
    // Send HELLO
    client.send_hello().await?;
    println!("👋 Sent HELLO to server");
    
    // Send messages
    loop {
        println!("\nEnter a message (or 'quit' to exit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "quit" || input == "exit" {
            break;
        }
        
        if !input.is_empty() {
            client.send_data(input.as_bytes().to_vec()).await?;
            println!("📤 Sent: {}", input);
        }
    }
    
    // Close connection gracefully
    client.close().await?;
    println!("\n👋 Disconnected from server");
    
    Ok(())
}

