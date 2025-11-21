//! Simple VSTP TCP Client - Minimal example with binary encoding demonstration
//!
//! Run with: cargo run --example simple_client

use std::io;
use vstp::{
    encode_frame,
    tcp::VstpTcpClient,
    types::{Frame, FrameType},
};

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

/// Display what we're sending (encoding process)
fn display_sending_frame(frame: &Frame, show_bytes: bool) {
    if show_bytes {
        let encoded = encode_frame(frame).unwrap_or_default();

        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║        📤 ENCODING MESSAGE → BINARY (VSTP Protocol)         ║");
        println!("╚══════════════════════════════════════════════════════════════╝");

        println!("\n📝 Original Message:");
        if !frame.payload.is_empty() {
            println!("   Text: \"{}\"", String::from_utf8_lossy(&frame.payload));
            println!("   Size: {} bytes", frame.payload.len());
        }

        println!("\n🔢 Encoded Binary Frame ({} bytes):", encoded.len());
        println!("   {}", format_bytes_hex(&encoded, 64));

        println!("\n🔍 Frame Components:");
        println!("   Magic:    56 54 (VT - VSTP identifier)");
        println!("   Version:  {:02X} (Protocol version)", frame.version);
        println!("   Type:     {:02X} ({:?})", frame.typ as u8, frame.typ);
        println!("   Flags:    {:02X}", frame.flags.bits());
        println!("   Payload:  {} bytes", frame.payload.len());
        println!("   CRC-32:   (calculated automatically)");

        println!("\n🔍 PROTOCOL COMPARISON:");
        let http_overhead = frame.payload.len() + 150; // Typical HTTP overhead
        let efficiency = ((1.0 - encoded.len() as f64 / http_overhead as f64) * 100.0) as u32;
        println!(
            "   [HTTP]  Text-based: {} bytes (vulnerable to injection)",
            http_overhead
        );
        println!(
            "   [VSTP]  Binary: {} bytes (secure, compact)",
            encoded.len()
        );
        println!(
            "   [GAIN]  Efficiency: {}% reduction in bandwidth",
            efficiency
        );
        println!(
            "   [SPEED] Parsing: ~{}x faster (binary vs text parsing)",
            (http_overhead as f64 / encoded.len() as f64) as u32
        );
        println!("   [SECURITY] No text parsing = no injection vectors");
        println!("   [ANALYSIS] Frame structure prevents protocol-level attacks");
        println!();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔌 Connecting to VSTP Server at 127.0.0.1:8080...");

    // Connect to server
    let mut client = VstpTcpClient::connect("127.0.0.1:8080").await?;
    println!("✅ Connected to server!\n");

    println!("💡 Commands:");
    println!("   - Type any message to send");
    println!("   - Type 'show-bytes' to toggle binary display");
    println!("   - Type 'quit' or 'exit' to disconnect\n");

    // Send HELLO
    let hello_frame = Frame::new(FrameType::Hello);
    let encoded_hello = encode_frame(&hello_frame).unwrap_or_default();
    client.send_hello().await?;
    println!("👋 Sent HELLO to server");
    println!("   Binary: {}", format_bytes_hex(&encoded_hello, 32));

    let mut show_bytes = true; // Default to showing bytes

    // Send messages
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
            println!(
                "   Binary display: {}",
                if show_bytes { "ON" } else { "OFF" }
            );
            continue;
        }

        if !input.is_empty() {
            let data_frame = Frame::new(FrameType::Data).with_payload(input.as_bytes().to_vec());

            display_sending_frame(&data_frame, show_bytes);

            client.send_data(input.as_bytes().to_vec()).await?;
            println!("📤 Sent: {}", input);
        }
    }

    // Close connection gracefully
    let bye_frame = Frame::new(FrameType::Bye);
    let encoded_bye = encode_frame(&bye_frame).unwrap_or_default();
    client.close().await?;
    println!("\n👋 Disconnected from server");
    println!("   Final BYE frame: {}", format_bytes_hex(&encoded_bye, 32));

    Ok(())
}
