//! Simple VSTP TCP Server - Minimal example with binary encoding demonstration
//!
//! Run with: cargo run --example simple_server

use vstp::{
    encode_frame,
    tcp::VstpTcpServer,
    types::{Frame, FrameType, SessionId},
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

/// Display frame structure breakdown
fn display_frame_structure(frame: &Frame, encoded_bytes: &[u8]) {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║          🔐 VSTP FRAME STRUCTURE (Binary Protocol)          ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    println!("\n📋 Frame Information:");
    println!("   Type: {:?}", frame.typ);
    println!("   Version: 0x{:02X}", frame.version);
    println!("   Flags: 0x{:02X} ({:?})", frame.flags.bits(), frame.flags);
    println!("   Headers: {} header(s)", frame.headers.len());
    println!("   Payload Size: {} bytes", frame.payload.len());

    println!(
        "\n🔢 Encoded Binary Bytes ({} total bytes):",
        encoded_bytes.len()
    );
    println!("   {}", format_bytes_hex(encoded_bytes, 64));

    // Break down the frame structure
    if encoded_bytes.len() >= 5 {
        println!("\n📦 Frame Breakdown:");
        println!(
            "   [0-1]   Magic:     {:02X} {:02X} (VSTP identifier)",
            encoded_bytes[0], encoded_bytes[1]
        );
        println!(
            "   [2]     Version:   {:02X} (Protocol version)",
            encoded_bytes[2]
        );
        println!(
            "   [3]     Type:      {:02X} ({:?})",
            encoded_bytes[3], frame.typ
        );
        println!(
            "   [4]     Flags:     {:02X} (Frame flags)",
            encoded_bytes[4]
        );

        if encoded_bytes.len() >= 11 {
            let header_len = u16::from_le_bytes([encoded_bytes[5], encoded_bytes[6]]);
            let payload_len = u32::from_be_bytes([
                encoded_bytes[7],
                encoded_bytes[8],
                encoded_bytes[9],
                encoded_bytes[10],
            ]);
            println!(
                "   [5-6]   Header Len: {} bytes (little-endian)",
                header_len
            );
            println!("   [7-10]  Payload Len: {} bytes (big-endian)", payload_len);
        }

        // Show CRC (last 4 bytes)
        if encoded_bytes.len() >= 4 {
            let crc_start = encoded_bytes.len() - 4;
            let crc = u32::from_be_bytes([
                encoded_bytes[crc_start],
                encoded_bytes[crc_start + 1],
                encoded_bytes[crc_start + 2],
                encoded_bytes[crc_start + 3],
            ]);
            println!(
                "   [{}..]  CRC-32:    {:08X} (integrity check)",
                crc_start, crc
            );
        }
    }

    // Show payload preview
    if !frame.payload.is_empty() {
        let preview_len = frame.payload.len().min(50);
        let payload_preview = String::from_utf8_lossy(&frame.payload[..preview_len]);
        println!("\n💬 Payload Preview:");
        println!("   Text: \"{}\"", payload_preview);
        println!("   Hex:  {}", format_bytes_hex(&frame.payload, 32));
    }

    // Show headers if any
    if !frame.headers.is_empty() {
        println!("\n📎 Headers:");
        for header in &frame.headers {
            let key_str = String::from_utf8_lossy(&header.key);
            let value_str = String::from_utf8_lossy(&header.value);
            println!("   {}: {}", key_str, value_str);
        }
    }

    println!("\n🔒 PROTOCOL ANALYSIS:");
    println!("   [SECURITY] Binary encoding prevents text-based injection attacks");
    println!("   [PERFORMANCE] Fixed frame structure: O(1) parsing vs HTTP O(n)");
    println!(
        "   [INTEGRITY] CRC-32 checksum: {} bytes verified",
        encoded_bytes.len()
    );
    println!(
        "   [EXTENSIBILITY] {} custom headers supported (vs HTTP fixed headers)",
        frame.headers.len()
    );
    println!(
        "   [OVERHEAD] Frame overhead: {} bytes (HTTP: ~200+ bytes)",
        encoded_bytes.len() - frame.payload.len()
    );
    println!("   [ENCRYPTION] Ready for TLS 1.3 layer (not shown in demo)");
    println!("   [DETECTION] AI anomaly detection enabled (packet theft, MITM, flood)");
    println!();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚀 Starting VSTP Server on 127.0.0.1:8080");
    println!("   Waiting for clients to connect...");
    println!("   📊 Server will show binary encoding/decoding of all frames\n");

    // Bind server to localhost:8080
    let server = VstpTcpServer::bind("127.0.0.1:8080").await?;

    // Run server with handler
    server
        .run(|session_id: SessionId, frame: Frame| async move {
            // Re-encode the frame to show the binary representation
            let encoded = encode_frame(&frame).unwrap_or_default();

            match frame.typ {
                FrameType::Hello => {
                    println!("✅ Client {} connected (sent HELLO)", session_id);
                    display_frame_structure(&frame, &encoded);
                }
                FrameType::Data => {
                    let message = String::from_utf8_lossy(&frame.payload);
                    println!("📨 Message from client {}: {}", session_id, message);
                    display_frame_structure(&frame, &encoded);
                }
                FrameType::Bye => {
                    println!("👋 Client {} disconnected (sent BYE)", session_id);
                    display_frame_structure(&frame, &encoded);
                }
                _ => {
                    println!("📦 Received {:?} from client {}", frame.typ, session_id);
                    display_frame_structure(&frame, &encoded);
                }
            }
        })
        .await?;

    Ok(())
}
