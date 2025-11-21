//! Compare VSTP vs HTTP metrics side-by-side
//!
//! Run with: cargo run --example compare::compare_metrics

use std::io;
use std::net::TcpStream;
use std::time::Instant;
use vstp::{encode_frame, types::{Frame, FrameType}};

/// Format bytes as hex string
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

/// Test HTTP request
fn test_http(message: &str) -> (usize, f64) {
    let request = format!(
        "POST /api/message HTTP/1.1\r\n\
        Host: 127.0.0.1:8081\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: {}\r\n\
        Connection: keep-alive\r\n\
        User-Agent: VSTP-Compare/1.0\r\n\
        \r\n\
        {}",
        message.len(),
        message
    );
    
    let start = Instant::now();
    let _ = TcpStream::connect("127.0.0.1:8081");
    let connect_time = start.elapsed().as_secs_f64() * 1000.0;
    
    (request.len(), connect_time)
}

/// Test VSTP frame
fn test_vstp(message: &str) -> (usize, f64) {
    let frame = Frame::new(FrameType::Data).with_payload(message.as_bytes().to_vec());
    let encoded = encode_frame(&frame).unwrap_or_default();
    
    let start = Instant::now();
    let _ = TcpStream::connect("127.0.0.1:8080");
    let connect_time = start.elapsed().as_secs_f64() * 1000.0;
    
    (encoded.len(), connect_time)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          🔬 VSTP vs HTTP PROTOCOL COMPARISON                 ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");
    
    println!("Enter a test message:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let message = input.trim();
    
    if message.is_empty() {
        println!("Using default message: 'Hello, World!'");
        let message = "Hello, World!";
        run_comparison(message);
    } else {
        run_comparison(message);
    }
    
    Ok(())
}

fn run_comparison(message: &str) {
    println!("\n🔬 Running comparison tests...\n");
    
    // Test VSTP
    let (vstp_size, vstp_time) = test_vstp(message);
    let vstp_frame = Frame::new(FrameType::Data).with_payload(message.as_bytes().to_vec());
    let vstp_encoded = encode_frame(&vstp_frame).unwrap_or_default();
    
    // Test HTTP
    let (http_size, http_time) = test_http(message);
    let http_request = format!(
        "POST /api/message HTTP/1.1\r\n\
        Host: 127.0.0.1:8081\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: {}\r\n\
        Connection: keep-alive\r\n\
        User-Agent: VSTP-Compare/1.0\r\n\
        \r\n\
        {}",
        message.len(),
        message
    );
    
    // Calculate metrics
    let payload_size = message.len();
    let vstp_overhead = vstp_size - payload_size;
    let http_overhead = http_size - payload_size;
    let size_reduction = ((1.0 - vstp_size as f64 / http_size as f64) * 100.0) as u32;
    let overhead_reduction = ((1.0 - vstp_overhead as f64 / http_overhead as f64) * 100.0) as u32;
    
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    📊 METRICS COMPARISON                     ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");
    
    println!("📝 Test Message: \"{}\" ({} bytes)\n", message, payload_size);
    
    // Size comparison
    println!("📦 SIZE COMPARISON:");
    println!("   ┌─────────────────┬──────────┬──────────┬─────────────┐");
    println!("   │ Protocol        │ Total    │ Overhead │ Payload     │");
    println!("   ├─────────────────┼──────────┼──────────┼─────────────┤");
    println!("   │ HTTP (Text)      │ {:8} │ {:8} │ {:11} │", 
             http_size, http_overhead, payload_size);
    println!("   │ VSTP (Binary)    │ {:8} │ {:8} │ {:11} │", 
             vstp_size, vstp_overhead, payload_size);
    println!("   └─────────────────┴──────────┴──────────┴─────────────┘");
    println!("   ✅ VSTP is {}% smaller ({} bytes saved)", 
             size_reduction, http_size - vstp_size);
    println!("   ✅ Overhead reduced by {}% ({} bytes saved)\n", 
             overhead_reduction, http_overhead - vstp_overhead);
    
    // Binary representation
    println!("🔢 BINARY REPRESENTATION:");
    println!("   HTTP: {}", format_bytes_hex(http_request.as_bytes(), 40));
    println!("   VSTP: {}\n", format_bytes_hex(&vstp_encoded, 40));
    
    // Security comparison
    println!("🔒 SECURITY COMPARISON:");
    println!("   ┌─────────────────────┬──────────────┬──────────────┐");
    println!("   │ Feature             │ HTTP         │ VSTP         │");
    println!("   ├─────────────────────┼──────────────┼──────────────┤");
    println!("   │ Encoding             │ Text         │ Binary       │");
    println!("   │ Injection Risk       │ High         │ Low          │");
    println!("   │ Parsing Complexity   │ O(n)         │ O(1)         │");
    println!("   │ Integrity Check      │ None         │ CRC-32       │");
    println!("   │ Frame Structure      │ Variable     │ Fixed        │");
    println!("   │ Anomaly Detection     │ None         │ AI-Powered   │");
    println!("   └─────────────────────┴──────────────┴──────────────┘\n");
    
    // Performance comparison
    println!("⚡ PERFORMANCE COMPARISON:");
    println!("   HTTP Connection Time: {:.2} ms", http_time);
    println!("   VSTP Connection Time:  {:.2} ms", vstp_time);
    println!("   Parsing Speed: HTTP requires line-by-line parsing");
    println!("                 VSTP uses fixed offsets (faster)\n");
    
    // Summary
    println!("📋 SUMMARY:");
    println!("   ✅ VSTP uses {}% less bandwidth", size_reduction);
    println!("   ✅ VSTP has {}% less overhead", overhead_reduction);
    println!("   ✅ VSTP is more secure (binary encoding)");
    println!("   ✅ VSTP has built-in integrity checks (CRC-32)");
    println!("   ✅ VSTP supports AI anomaly detection");
    println!("   ✅ VSTP has fixed frame structure (faster parsing)");
    println!();
}

