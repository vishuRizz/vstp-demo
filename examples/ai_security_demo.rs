//! Example demonstrating AI-powered anomaly detection in VSTP
//!
//! This example shows how to use the AI anomaly detection system to detect:
//! - Packet theft attempts
//! - Man-in-the-Middle attacks
//! - Traffic anomalies
//! - Connection flooding
//! - And other cyber threats

use std::sync::Arc;
use vstp::{
    security::ai::{AnomalyDetector, ThreatLevel},
    security::ai::detector::DetectorConfig,
    tcp::{VstpTcpClient, VstpTcpServer},
    types::{Frame, SessionId},
};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚀 VSTP AI Security Demo");
    println!("========================");
    println!();

    // Create AI anomaly detector with custom configuration
    let config = DetectorConfig {
        enabled: true,
        min_confidence: 0.5, // Report threats with 50%+ confidence
        auto_block_critical: true, // Auto-block critical threats
        learning_mode: false,
        min_samples: 5, // Start detecting after 5 samples
    };

    let detector = Arc::new(AnomalyDetector::new(config));

    // Start TCP server with AI detection enabled
    let server = VstpTcpServer::bind("127.0.0.1:0").await?;
    let server_addr = server.local_addr()?;
    println!("📡 Server listening on: {}", server_addr);

    // Spawn server with AI detection
    let server_detector = detector.clone();
    let server_handle = tokio::spawn(async move {
        server
            .run_with_detector(
                |session_id: SessionId, frame: Frame| async move {
                    println!("📦 Server: Session {} received {:?} frame ({} bytes)", 
                        session_id, frame.typ, frame.payload.len());
                },
                Some(server_detector),
            )
            .await
            .unwrap();
    });

    // Give server time to start
    sleep(Duration::from_millis(100)).await;

    // Connect client
    println!("🔌 Connecting client...");
    let mut client = VstpTcpClient::connect(&format!("127.0.0.1:{}", server_addr.port())).await?;
    println!("✅ Client connected");

    // Send normal traffic
    println!("\n📤 Sending normal traffic...");
    client.send_hello().await?;
    sleep(Duration::from_millis(50)).await;

    for i in 0..5 {
        let payload = format!("Normal message {}", i).into_bytes();
        client.send_data(payload).await?;
        sleep(Duration::from_millis(100)).await;
    }

    // Simulate suspicious activity (rapid fire - potential flood)
    println!("\n⚠️  Simulating suspicious activity (rapid fire)...");
    for i in 0..20 {
        let payload = format!("Rapid message {}", i).into_bytes();
        client.send_data(payload).await?;
        // Very short delay - might trigger flood detection
        sleep(Duration::from_millis(10)).await;
    }

    // Check threat history
    println!("\n🔍 Checking threat history...");
    let threats = detector.get_threat_history(10).await;
    if threats.is_empty() {
        println!("✅ No threats detected");
    } else {
        println!("🚨 Detected {} threat(s):", threats.len());
        for threat in threats {
            let level_emoji = match threat.threat_level {
                ThreatLevel::Critical => "🔴",
                ThreatLevel::High => "🟠",
                ThreatLevel::Medium => "🟡",
                ThreatLevel::Low => "🟢",
                ThreatLevel::None => "⚪",
            };
            println!(
                "  {} {} - {} (confidence: {:.1}%)",
                level_emoji,
                threat.pattern.description(),
                threat.description,
                threat.confidence * 100.0
            );
            if !threat.indicators.is_empty() {
                for indicator in &threat.indicators {
                    println!("    └─ Indicator: {}", indicator);
                }
            }
        }
    }

    // Get connection statistics
    println!("\n📊 Connection Statistics:");
    if let Some(stats) = detector.get_connection_stats(2).await {
        println!("  Frames: {}", stats.frame_count);
        println!("  Bytes: {}", stats.byte_count);
        println!("  FPS: {:.2}", stats.get_frames_per_second());
        println!("  BPS: {:.2}", stats.get_bytes_per_second());
        println!("  Avg frame size: {:.2} bytes", stats.avg_frame_size);
        println!("  Errors: {} CRC, {} Protocol", stats.crc_errors, stats.protocol_errors);
    }

    // Close connection
    println!("\n🔚 Closing connection...");
    client.close().await?;
    sleep(Duration::from_millis(200)).await;

    // Cleanup
    server_handle.abort();
    detector.cleanup().await;

    println!("\n✅ Demo completed!");
    Ok(())
}

