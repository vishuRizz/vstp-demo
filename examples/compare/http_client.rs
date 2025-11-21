//! HTTP Client for comparison with VSTP
//!
//! Run with: cargo run --example compare::http_client

use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

/// Format bytes as hex string for display
fn format_bytes_hex(bytes: &[u8], max_display: usize) -> String {
    let display_len = bytes.len().min(max_display);
    let hex: Vec<String> = bytes[..display_len]
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect();
    let mut result = hex.join(" ");
    if bytes.len() > max_display {
        result.push_str(&format!(" ... ({} more bytes)", bytes.len() - max_display));
    }
    result
}

/// Display HTTP request being sent
fn display_http_request(request: &str, show_bytes: bool) {
    if show_bytes {
        let request_bytes = request.as_bytes();
        
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║        📤 ENCODING MESSAGE → HTTP (Text Protocol)           ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        
        println!("\n📝 Original Message:");
        if let Some(body_start) = request.find("\r\n\r\n") {
            let body = &request[body_start + 4..];
            println!("   Text: \"{}\"", body);
            println!("   Size: {} bytes", body.len());
        }
        
        println!("\n🔢 HTTP Request Bytes ({} total bytes):", request_bytes.len());
        println!("   {}", format_bytes_hex(request_bytes, 64));
        
        println!("\n🔍 HTTP Request Components:");
        let lines: Vec<&str> = request.lines().collect();
        if !lines.is_empty() {
            println!("   Request Line: {}", lines[0]);
        }
        println!("   Headers: {} header(s)", lines.len().saturating_sub(2));
        if let Some(body_start) = request.find("\r\n\r\n") {
            let body = &request[body_start + 4..];
            println!("   Body: {} bytes", body.len());
        }
        
        println!("\n🔍 PROTOCOL COMPARISON:");
        let body_size = request.find("\r\n\r\n")
            .map(|i| request.len() - i - 4)
            .unwrap_or(0);
        let overhead = request_bytes.len() - body_size;
        println!("   [HTTP]  Text-based: {} bytes total", request_bytes.len());
        println!("   [HTTP]  Overhead: {} bytes (headers, formatting)", overhead);
        println!("   [HTTP]  Body: {} bytes", body_size);
        println!("   [EFFICIENCY] {}% overhead", 
                 (overhead as f64 / request_bytes.len() as f64 * 100.0) as u32);
        println!("   [SECURITY] Text parsing required = injection risk");
        println!("   [PARSING] Must parse line-by-line (slow)");
        println!("   [ANALYSIS] No built-in integrity checks");
        println!();
    }
}

fn main() -> std::io::Result<()> {
    println!("🔌 Connecting to HTTP Server at 127.0.0.1:8081...");
    
    let mut stream = TcpStream::connect("127.0.0.1:8081")?;
    println!("✅ Connected to server!\n");
    
    println!("💡 Commands:");
    println!("   - Type any message to send");
    println!("   - Type 'show-bytes' to toggle binary display");
    println!("   - Type 'quit' or 'exit' to disconnect\n");
    
    let mut show_bytes = true;
    
    loop {
        println!("\nEnter a message (or 'quit' to exit, 'show-bytes' to toggle):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "quit" || input == "exit" {
            break;
        }
        
        if input == "show-bytes" {
            show_bytes = !show_bytes;
            println!("   Binary display: {}", if show_bytes { "ON" } else { "OFF" });
            continue;
        }
        
        if !input.is_empty() {
            // Build HTTP POST request
            let request = format!(
                "POST /api/message HTTP/1.1\r\n\
                Host: 127.0.0.1:8081\r\n\
                Content-Type: text/plain\r\n\
                Content-Length: {}\r\n\
                Connection: keep-alive\r\n\
                User-Agent: VSTP-Compare-Client/1.0\r\n\
                \r\n\
                {}",
                input.len(),
                input
            );
            
            display_http_request(&request, show_bytes);
            
            stream.write_all(request.as_bytes())?;
            println!("📤 Sent: {}", input);
            
            // Read response
            let mut response = vec![0u8; 1024];
            match stream.read(&mut response) {
                Ok(size) => {
                    let response_str = String::from_utf8_lossy(&response[..size]);
                    println!("📥 Response: {}", response_str.lines().next().unwrap_or(""));
                }
                Err(_) => {}
            }
        }
    }
    
    println!("\n👋 Disconnected from server");
    Ok(())
}

