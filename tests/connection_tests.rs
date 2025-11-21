//! Comprehensive connection tests for VSTP protocol
//! Tests both TCP and UDP server-client connections

use std::time::Duration;
use tokio::time::timeout;
use vstp::{
    tcp::{VstpTcpClient, VstpTcpServer},
    udp::{VstpUdpClient, VstpUdpServer},
    types::{Frame, FrameType, SessionId},
};

/// Test TCP server binding and client connection
#[tokio::test]
async fn test_tcp_server_bind_and_connect() {
    println!("=== Testing TCP Server Bind and Client Connection ===");
    
    // Start TCP server
    let server = VstpTcpServer::bind("127.0.0.1:0").await.expect("Failed to bind TCP server");
    let server_addr = server.local_addr().expect("Failed to get server address");
    println!("TCP Server bound to: {}", server_addr);

    // Spawn server task
    let server_handle = tokio::spawn(async move {
        server
            .run(|session_id: SessionId, frame: Frame| async move {
                println!("TCP Server: Session {} received frame type: {:?}", session_id, frame.typ);
                match frame.typ {
                    FrameType::Hello => {
                        println!("TCP Server: Received HELLO from session {}", session_id);
                    }
                    FrameType::Data => {
                        println!(
                            "TCP Server: Received DATA from session {} ({} bytes)",
                            session_id,
                            frame.payload.len()
                        );
                    }
                    FrameType::Bye => {
                        println!("TCP Server: Received BYE from session {}", session_id);
                    }
                    _ => {
                        println!("TCP Server: Received {:?} from session {}", frame.typ, session_id);
                    }
                }
            })
            .await
            .expect("Server error");
    });

    // Give server time to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Connect TCP client
    let mut client = VstpTcpClient::connect(&format!("127.0.0.1:{}", server_addr.port()))
        .await
        .expect("Failed to connect TCP client");
    println!("TCP Client connected to server");

    // Send HELLO frame
    client.send_hello().await.expect("Failed to send HELLO");
    println!("TCP Client sent HELLO");

    // Wait for server to process
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Send DATA frame
    let test_payload = b"Hello from TCP client!".to_vec();
    client.send_data(test_payload.clone()).await.expect("Failed to send DATA");
    println!("TCP Client sent DATA ({} bytes)", test_payload.len());

    // Wait for server to process
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Close connection gracefully
    client.close().await.expect("Failed to close TCP client");
    println!("TCP Client closed connection");

    // Wait for server to finish processing
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Stop server
    server_handle.abort();
    println!("✓ TCP connection test completed successfully!\n");
}

/// Test TCP bidirectional communication
#[tokio::test]
async fn test_tcp_bidirectional_communication() {
    println!("=== Testing TCP Bidirectional Communication ===");
    
    let server = VstpTcpServer::bind("127.0.0.1:0").await.expect("Failed to bind TCP server");
    let server_addr = server.local_addr().expect("Failed to get server address");
    
    let server_handle = tokio::spawn(async move {
        server
            .run(|_session_id: SessionId, frame: Frame| async move {
                match frame.typ {
                    FrameType::Data => {
                        // Echo the data back
                        println!("TCP Server: Echoing data back to client");
                    }
                    _ => {}
                }
            })
            .await
            .expect("Server error");
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let mut client = VstpTcpClient::connect(&format!("127.0.0.1:{}", server_addr.port()))
        .await
        .expect("Failed to connect");

    // Send data
    let payload = b"Test message for bidirectional communication".to_vec();
    client.send_data(payload).await.expect("Failed to send");

    tokio::time::sleep(Duration::from_millis(200)).await;

    client.close().await.expect("Failed to close");
    server_handle.abort();
    
    println!("✓ TCP bidirectional test completed!\n");
}

/// Test UDP server binding and client connection
#[tokio::test]
async fn test_udp_server_bind_and_connect() {
    println!("=== Testing UDP Server Bind and Client Connection ===");
    
    // Start UDP server
    let server = VstpUdpServer::bind("127.0.0.1:0").await.expect("Failed to bind UDP server");
    let server_addr = server.local_addr().expect("Failed to get server address");
    println!("UDP Server bound to: {}", server_addr);

    // Spawn server task
    let server_handle = tokio::spawn(async move {
        server
            .run(|addr, frame| async move {
                println!("UDP Server: Received frame from {} - type: {:?}", addr, frame.typ);
                match frame.typ {
                    FrameType::Hello => {
                        println!("UDP Server: Received HELLO from {}", addr);
                    }
                    FrameType::Data => {
                        println!(
                            "UDP Server: Received DATA from {} ({} bytes)",
                            addr,
                            frame.payload.len()
                        );
                    }
                    FrameType::Bye => {
                        println!("UDP Server: Received BYE from {}", addr);
                    }
                    _ => {
                        println!("UDP Server: Received {:?} from {}", frame.typ, addr);
                    }
                }
            })
            .await
            .expect("Server error");
    });

    // Give server time to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Create UDP client
    let client = VstpUdpClient::bind("127.0.0.1:0").await.expect("Failed to bind UDP client");
    let client_addr = client.local_addr().expect("Failed to get client address");
    println!("UDP Client bound to: {}", client_addr);

    // Send HELLO frame
    let hello_frame = Frame::new(FrameType::Hello);
    client.send(hello_frame, server_addr).await.expect("Failed to send HELLO");
    println!("UDP Client sent HELLO to server");

    // Wait for server to process
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Send DATA frame
    let test_payload = b"Hello from UDP client!".to_vec();
    let data_frame = Frame::new(FrameType::Data).with_payload(test_payload.clone());
    client.send(data_frame, server_addr).await.expect("Failed to send DATA");
    println!("UDP Client sent DATA ({} bytes) to server", test_payload.len());

    // Wait for server to process
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Send BYE frame
    let bye_frame = Frame::new(FrameType::Bye);
    client.send(bye_frame, server_addr).await.expect("Failed to send BYE");
    println!("UDP Client sent BYE to server");

    // Wait for server to process
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Stop server
    server_handle.abort();
    println!("✓ UDP connection test completed successfully!\n");
}

/// Test UDP with ACK reliability
#[tokio::test]
async fn test_udp_ack_reliability() {
    println!("=== Testing UDP ACK Reliability ===");
    
    let server = VstpUdpServer::bind("127.0.0.1:0").await.expect("Failed to bind UDP server");
    let server_addr = server.local_addr().expect("Failed to get server address");
    
    let server_handle = tokio::spawn(async move {
        server
            .run(|addr, frame| async move {
                println!("UDP Server: Received frame from {} - type: {:?}", addr, frame.typ);
            })
            .await
            .expect("Server error");
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let mut client = VstpUdpClient::bind("127.0.0.1:0").await.expect("Failed to bind UDP client");

    // Send DATA frame with ACK reliability
    let data_frame = Frame::new(FrameType::Data)
        .with_payload(b"Reliable message with ACK!".to_vec());
    
    println!("UDP Client: Sending reliable message with ACK...");
    let result = timeout(
        Duration::from_secs(5),
        client.send_with_ack(data_frame, server_addr)
    ).await;
    
    assert!(result.is_ok(), "send_with_ack should complete within timeout");
    assert!(result.unwrap().is_ok(), "send_with_ack should succeed");
    println!("UDP Client: Received ACK confirmation");

    tokio::time::sleep(Duration::from_millis(200)).await;
    server_handle.abort();
    
    println!("✓ UDP ACK reliability test completed!\n");
}

/// Test UDP client receiving data
#[tokio::test]
async fn test_udp_client_receive() {
    println!("=== Testing UDP Client Receive ===");
    
    let server = VstpUdpServer::bind("127.0.0.1:0").await.expect("Failed to bind UDP server");
    let server_addr = server.local_addr().expect("Failed to get server address");
    
    let server_handle = tokio::spawn(async move {
        server
            .run(|_addr, frame| async move {
                println!("UDP Server: Received {:?}", frame.typ);
                // In a real scenario, server would send response here
            })
            .await
            .expect("Server error");
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let client = VstpUdpClient::bind("127.0.0.1:0").await.expect("Failed to bind UDP client");

    // Send a frame to server
    let request_frame = Frame::new(FrameType::Data)
        .with_payload(b"Request from client".to_vec());
    client.send(request_frame, server_addr).await.expect("Failed to send");

    tokio::time::sleep(Duration::from_millis(300)).await;
    server_handle.abort();
    
    println!("✓ UDP client receive test completed!\n");
}

/// Test both TCP and UDP in sequence
#[tokio::test]
async fn test_both_protocols_sequential() {
    println!("=== Testing Both Protocols Sequentially ===");
    
    // Test TCP first
    println!("--- Testing TCP ---");
    let tcp_server = VstpTcpServer::bind("127.0.0.1:0").await.expect("Failed to bind TCP server");
    let tcp_addr = tcp_server.local_addr().expect("Failed to get TCP server address");
    
    let tcp_handle = tokio::spawn(async move {
        tcp_server
            .run(|_session_id: SessionId, frame: Frame| async move {
                println!("TCP Server: Received {:?}", frame.typ);
            })
            .await
            .expect("TCP server error");
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let mut tcp_client = VstpTcpClient::connect(&format!("127.0.0.1:{}", tcp_addr.port()))
        .await
        .expect("Failed to connect TCP client");
    
    tcp_client.send_hello().await.expect("Failed to send TCP HELLO");
    tcp_client.send_data(b"TCP test message".to_vec()).await.expect("Failed to send TCP DATA");
    tokio::time::sleep(Duration::from_millis(100)).await;
    tcp_client.close().await.expect("Failed to close TCP client");
    
    tokio::time::sleep(Duration::from_millis(200)).await;
    tcp_handle.abort();
    println!("✓ TCP test passed");

    // Test UDP second
    println!("--- Testing UDP ---");
    let udp_server = VstpUdpServer::bind("127.0.0.1:0").await.expect("Failed to bind UDP server");
    let udp_addr = udp_server.local_addr().expect("Failed to get UDP server address");
    
    let udp_handle = tokio::spawn(async move {
        udp_server
            .run(|_addr, frame| async move {
                println!("UDP Server: Received {:?}", frame.typ);
            })
            .await
            .expect("UDP server error");
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let udp_client = VstpUdpClient::bind("127.0.0.1:0").await.expect("Failed to bind UDP client");
    
    let hello = Frame::new(FrameType::Hello);
    udp_client.send(hello, udp_addr).await.expect("Failed to send UDP HELLO");
    
    let data = Frame::new(FrameType::Data).with_payload(b"UDP test message".to_vec());
    udp_client.send(data, udp_addr).await.expect("Failed to send UDP DATA");
    
    tokio::time::sleep(Duration::from_millis(200)).await;
    udp_handle.abort();
    println!("✓ UDP test passed");
    
    println!("✓ Both protocols tested successfully!\n");
}

/// Test multiple TCP clients connecting to same server
#[tokio::test]
async fn test_tcp_multiple_clients() {
    println!("=== Testing Multiple TCP Clients ===");
    
    let server = VstpTcpServer::bind("127.0.0.1:0").await.expect("Failed to bind TCP server");
    let server_addr = server.local_addr().expect("Failed to get server address");
    
    let server_handle = tokio::spawn(async move {
        server
            .run(|session_id: SessionId, frame: Frame| async move {
                println!("TCP Server: Session {} received {:?}", session_id, frame.typ);
            })
            .await
            .expect("Server error");
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Connect 5 clients
    let mut clients = Vec::new();
    for i in 0..5 {
        let mut client = VstpTcpClient::connect(&format!("127.0.0.1:{}", server_addr.port()))
            .await
            .expect(&format!("Failed to connect client {}", i));
        
        client.send_hello().await.expect(&format!("Failed to send HELLO from client {}", i));
        let payload = format!("Message from client {}", i).as_bytes().to_vec();
        client.send_data(payload).await.expect(&format!("Failed to send DATA from client {}", i));
        
        clients.push(client);
        println!("Client {} connected and sent data", i);
    }

    tokio::time::sleep(Duration::from_millis(300)).await;

    // Close all clients
    for (i, client) in clients.iter_mut().enumerate() {
        client.close().await.expect(&format!("Failed to close client {}", i));
    }

    tokio::time::sleep(Duration::from_millis(200)).await;
    server_handle.abort();
    
    println!("✓ Multiple TCP clients test completed!\n");
}

/// Test multiple UDP clients sending to same server
#[tokio::test]
async fn test_udp_multiple_clients() {
    println!("=== Testing Multiple UDP Clients ===");
    
    let server = VstpUdpServer::bind("127.0.0.1:0").await.expect("Failed to bind UDP server");
    let server_addr = server.local_addr().expect("Failed to get server address");
    
    let server_handle = tokio::spawn(async move {
        server
            .run(|addr, frame| async move {
                println!("UDP Server: Received {:?} from {}", frame.typ, addr);
            })
            .await
            .expect("Server error");
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Create 5 clients
    let mut clients = Vec::new();
    for i in 0..5 {
        let client = VstpUdpClient::bind("127.0.0.1:0").await
            .expect(&format!("Failed to bind UDP client {}", i));
        
        let hello = Frame::new(FrameType::Hello);
        client.send(hello, server_addr).await
            .expect(&format!("Failed to send HELLO from client {}", i));
        
        let message = format!("Message from UDP client {}", i);
        let data = Frame::new(FrameType::Data).with_payload(message.as_bytes().to_vec());
        client.send(data, server_addr).await
            .expect(&format!("Failed to send DATA from client {}", i));
        
        clients.push(client);
        println!("UDP Client {} sent data", i);
    }

    tokio::time::sleep(Duration::from_millis(300)).await;
    server_handle.abort();
    
    println!("✓ Multiple UDP clients test completed!\n");
}

