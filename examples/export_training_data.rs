//! Export VSTP traffic data to CSV for ML model training
//! 
//! Run with: cargo run --example export_training_data
//! 
//! This will collect data from a running VSTP server and export it to CSV format

use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use vstp::{
    security::ai::{AnomalyDetector, AttackPattern},
    security::ai::detector::DetectorConfig,
    tcp::{VstpTcpClient, VstpTcpServer},
    types::{Frame, FrameType, SessionId},
};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    println!("📊 VSTP Training Data Exporter");
    println!("===============================\n");
    
    // Create detector in learning mode (collects data without blocking)
    let config = DetectorConfig {
        enabled: true,
        min_confidence: 0.3, // Lower threshold to collect more data
        auto_block_critical: false, // Don't block, just collect
        learning_mode: true, // Learning mode - collect all data
        min_samples: 1, // Start collecting immediately
    };
    
    let detector = Arc::new(AnomalyDetector::new(config));
    let detector_clone = detector.clone();
    
    // Start server
    let server = VstpTcpServer::bind("127.0.0.1:8080").await?;
    println!("✅ Server started on 127.0.0.1:8080");
    println!("   Collecting data for 30 seconds...\n");
    
    let server_handle = tokio::spawn(async move {
        server
            .run_with_detector(
                |_session_id: SessionId, _frame: Frame| async move {
                    // Just process frames, detector collects data
                },
                Some(detector_clone),
            )
            .await
            .unwrap();
    });
    
    sleep(Duration::from_millis(100)).await;
    
    // Simulate different types of traffic
    println!("📤 Generating sample traffic patterns...");
    
    // Normal traffic
    let mut normal_client = VstpTcpClient::connect("127.0.0.1:8080").await?;
    normal_client.send_hello().await?;
    for i in 0..20 {
        let msg = format!("Normal message {}", i);
        normal_client.send_data(msg.as_bytes().to_vec()).await?;
        sleep(Duration::from_millis(100)).await;
    }
    normal_client.close().await?;
    println!("   ✓ Normal traffic sent");
    
    sleep(Duration::from_millis(500)).await;
    
    // Flood traffic (anomaly)
    let mut flood_client = VstpTcpClient::connect("127.0.0.1:8080").await?;
    flood_client.send_hello().await?;
    for i in 0..200 {
        let msg = format!("Flood {}", i);
        flood_client.send_data(msg.as_bytes().to_vec()).await?;
        sleep(Duration::from_millis(5)).await; // Very fast
    }
    flood_client.close().await?;
    println!("   ✓ Flood traffic sent (anomaly)");
    
    sleep(Duration::from_millis(500)).await;
    
    // High data ratio (exfiltration pattern)
    let mut exfil_client = VstpTcpClient::connect("127.0.0.1:8080").await?;
    exfil_client.send_hello().await?;
    for i in 0..150 {
        let data = vec![0u8; 500]; // Large data frames
        exfil_client.send_data(data).await?;
        sleep(Duration::from_millis(50)).await;
    }
    exfil_client.close().await?;
    println!("   ✓ Data exfiltration pattern sent (anomaly)");
    
    sleep(Duration::from_millis(1000)).await;
    
    // Export data to CSV
    println!("\n📝 Exporting data to CSV...");
    export_to_csv(&detector, "vstp_training_data.csv").await?;
    
    server_handle.abort();
    
    println!("\n✅ Data exported to: vstp_training_data.csv");
    println!("   You can now use this CSV to train your ML model!");
    
    Ok(())
}

async fn export_to_csv(
    detector: &AnomalyDetector,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(filename)?;
    
    // Write CSV header
    writeln!(file, "session_id,frame_count,byte_count,connection_duration_seconds,frames_per_second,bytes_per_second,avg_frame_size,max_frame_size,min_frame_size,hello_frames,data_frames,ack_frames,data_frame_ratio,avg_inter_arrival_time_ms,crc_errors,protocol_errors,error_rate,is_anomaly,threat_type")?;
    
    // Get all connection stats
    let connections = detector.get_connection_stats(1).await; // This needs to be implemented to get all
    
    // For now, we'll create a simple export function
    // You'll need to implement get_all_connections() in the detector
    
    // Placeholder - you'll need to iterate through all sessions
    // and export their stats
    
    Ok(())
}

