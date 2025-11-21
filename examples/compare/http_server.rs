//! HTTP Server for comparison with VSTP
//! 
//! Run with: cargo run --example compare::http_server

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

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

/// Display HTTP request structure
fn display_http_request(request_bytes: &[u8]) {
    let request_str = String::from_utf8_lossy(request_bytes);
    
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║        🌐 HTTP REQUEST STRUCTURE (Text Protocol)             ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    
    println!("\n📋 Request Information:");
    let lines: Vec<&str> = request_str.lines().collect();
    if !lines.is_empty() {
        println!("   Method: {}", lines[0].split_whitespace().next().unwrap_or("UNKNOWN"));
        println!("   Headers: {} header(s)", lines.len().saturating_sub(2));
    }
    
    // Find body
    let body_start = request_str.find("\r\n\r\n").map(|i| i + 4).unwrap_or(request_str.len());
    let body = &request_bytes[body_start..];
    println!("   Body Size: {} bytes", body.len());
    
    println!("\n🔢 Raw HTTP Bytes ({} total bytes):", request_bytes.len());
    println!("   {}", format_bytes_hex(request_bytes, 64));
    
    println!("\n📦 HTTP Request Breakdown:");
    println!("   [0..]   Request Line + Headers: {} bytes", body_start);
    println!("   [{}..]  Body: {} bytes", body_start, body.len());
    
    // Show text representation
    println!("\n💬 Text Representation:");
    let preview = request_str.chars().take(200).collect::<String>();
    println!("   {}", preview.replace('\n', "\\n").replace('\r', "\\r"));
    if request_str.len() > 200 {
        println!("   ... ({} more characters)", request_str.len() - 200);
    }
    
    // Extract body if present
    if !body.is_empty() {
        let body_text = String::from_utf8_lossy(body);
        println!("\n📝 Body Content:");
        println!("   Text: \"{}\"", body_text);
        println!("   Hex:  {}", format_bytes_hex(body, 32));
    }
    
    println!("\n🔒 PROTOCOL ANALYSIS:");
    println!("   [SECURITY] Text-based: vulnerable to injection attacks");
    println!("   [PERFORMANCE] Variable structure: O(n) parsing required");
    println!("   [INTEGRITY] No built-in checksum (relies on TCP)");
    println!("   [OVERHEAD] HTTP headers: ~{} bytes overhead", body_start);
    println!("   [PARSING] Must parse text line-by-line (slow)");
    println!("   [INJECTION] Text parsing creates attack vectors");
    println!("   [SIZE] Total: {} bytes ({}% overhead)", 
             request_bytes.len(),
             (body_start as f64 / request_bytes.len() as f64 * 100.0) as u32);
    println!();
}

fn handle_client(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();
    println!("✅ Client {} connected", peer_addr);
    
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut request = Vec::new();
    
    // Read request line by line
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {
                request.extend_from_slice(line.as_bytes());
                // Empty line indicates end of headers
                if line == "\r\n" || line == "\n" {
                    // Try to read body if Content-Length is present
                    let request_str = String::from_utf8_lossy(&request);
                    if let Some(content_length_line) = request_str
                        .lines()
                        .find(|l| l.to_lowercase().starts_with("content-length:"))
                    {
                        if let Some(len_str) = content_length_line.split(':').nth(1) {
                            if let Ok(len) = len_str.trim().parse::<usize>() {
                                if len > 0 {
                                    let mut body = vec![0u8; len];
                                    if reader.read_exact(&mut body).is_ok() {
                                        request.extend_from_slice(&body);
                                    }
                                }
                            }
                        }
                    }
                    break;
                }
            }
            Err(_) => break,
        }
    }
    
    if !request.is_empty() {
        display_http_request(&request);
        
        // Send HTTP response
        let response = "HTTP/1.1 200 OK\r\n\
                       Content-Type: text/plain\r\n\
                       Content-Length: 13\r\n\
                       Connection: close\r\n\
                       \r\n\
                       Hello, World!";
        
        let _ = stream.write_all(response.as_bytes());
        println!("📤 Sent HTTP response ({} bytes)", response.len());
    }
    
    println!("👋 Client {} disconnected\n", peer_addr);
}

fn main() -> std::io::Result<()> {
    println!("🚀 Starting HTTP Server on 127.0.0.1:8081");
    println!("   Waiting for clients to connect...");
    println!("   📊 Server will show HTTP request structure\n");
    
    let listener = TcpListener::bind("127.0.0.1:8081")?;
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    
    Ok(())
}

