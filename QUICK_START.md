# ⚡ VSTP Quick Start Guide

> Get up and running with VSTP in 5 minutes

## Installation

```toml
[dependencies]
vstp = "0.2.1"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

```bash
cargo add vstp
```

---

## Choose Your Level

### 🎯 Level 1: Easy API (Recommended for Beginners)

**Perfect for**: Quick prototypes, type-safe messaging, JSON data

```rust
use vstp::easy::{VstpClient, VstpServer};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    user: String,
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), vstp::VstpError> {
    // Server
    let server = VstpServer::bind_tcp("127.0.0.1:8080").await?;
    tokio::spawn(async move {
        server.serve(|msg: ChatMessage| async move {
            println!("{}: {}", msg.user, msg.content);
            Ok(msg) // Echo back
        }).await
    });

    // Client
    let client = VstpClient::connect_tcp("127.0.0.1:8080").await?;
    let msg = ChatMessage {
        user: "Alice".into(),
        content: "Hello VSTP!".into(),
    };
    client.send(msg).await?;
    let response: ChatMessage = client.receive().await?;
    println!("Got: {}", response.content);

    Ok(())
}
```

### 🚀 Level 2: Transport API

**Perfect for**: Custom protocols, binary data, frame control

#### TCP Example

```rust
use vstp::{VstpTcpClient, VstpTcpServer, Frame, FrameType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Server
    let server = VstpTcpServer::bind("127.0.0.1:8080").await?;
    tokio::spawn(async move {
        server.run(|session_id, frame| async move {
            println!("Session {}: {:?} frame", session_id, frame.typ);
        }).await.unwrap();
    });

    // Client
    let mut client = VstpTcpClient::connect("127.0.0.1:8080").await?;

    let frame = Frame::new(FrameType::Data)
        .with_header("content-type", "application/octet-stream")
        .with_payload(vec![1, 2, 3, 4, 5]);

    client.send(frame).await?;
    Ok(())
}
```

#### UDP Example

```rust
use vstp::{VstpUdpClient, VstpUdpServer, Frame, FrameType, Flags};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Server
    let server = VstpUdpServer::bind("127.0.0.1:6969").await?;
    tokio::spawn(async move {
        server.run(|addr, frame| async move {
            println!("{} sent: {:?}", addr, frame.typ);
        }).await.unwrap();
    });

    // Client
    let mut client = VstpUdpClient::bind("0.0.0.0:0").await?;
    let dest = "127.0.0.1:6969".parse()?;

    // Regular send
    let frame = Frame::new(FrameType::Data)
        .with_payload(b"Fast UDP message".to_vec());
    client.send(frame, dest).await?;

    // Reliable send with ACK
    let reliable_frame = Frame::new(FrameType::Data)
        .with_payload(b"Important message".to_vec());
    client.send_with_ack(reliable_frame, dest).await?;

    Ok(())
}
```

### 🔧 Level 3: Frame API (Advanced)

**Perfect for**: Custom transports, protocol extensions, maximum control

```rust
use vstp::{Frame, FrameType, encode_frame, try_decode_frame, Flags};
use bytes::BytesMut;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build frame
    let frame = Frame::new(FrameType::Data)
        .with_header("custom-header", "custom-value")
        .with_payload(b"Custom data".to_vec())
        .with_flag(Flags::CRC);

    // Encode to bytes
    let bytes = encode_frame(&frame)?;

    // Send over your custom transport...
    send_over_custom_transport(&bytes).await?;

    // Receive from transport...
    let received = receive_from_custom_transport().await?;

    // Decode back to frame
    let mut buffer = BytesMut::from(&received[..]);
    let decoded_frame = try_decode_frame(&mut buffer, 65536)?;

    if let Some(frame) = decoded_frame {
        println!("Received: {:?}", frame.typ);
    }

    Ok(())
}
```

---

## Common Patterns

### Pattern 1: Request-Response (TCP)

```rust
// Server
let server = VstpTcpServer::bind("127.0.0.1:8080").await?;
tokio::spawn(async move {
    loop {
        let mut conn = server.accept().await?;
        tokio::spawn(async move {
            while let Ok(Some(request)) = conn.recv().await {
                // Process request
                let response = process(request);
                conn.send(response).await?;
            }
            Ok::<_, VstpError>(())
        });
    }
});

// Client
let mut client = VstpTcpClient::connect("127.0.0.1:8080").await?;
client.send(request_frame).await?;
let response = client.recv().await?.unwrap();
```

### Pattern 2: Pub-Sub (UDP)

```rust
// Publisher
let publisher = VstpUdpClient::bind("0.0.0.0:0").await?;
let subscribers = vec!["127.0.0.1:7001", "127.0.0.1:7002"];

let message = Frame::new(FrameType::Data)
    .with_header("topic", "updates")
    .with_payload(data);

for addr in subscribers {
    publisher.send(message.clone(), addr.parse()?).await?;
}
```

### Pattern 3: Fire-and-Forget (UDP)

```rust
// Fast, no acknowledgment needed
let client = VstpUdpClient::bind("0.0.0.0:0").await?;
let metrics = Frame::new(FrameType::Data)
    .with_header("metric", "cpu_usage")
    .with_header("value", "75")
    .with_payload(b"".to_vec());

client.send(metrics, metrics_server).await?;
// Done! No waiting for response
```

### Pattern 4: Reliable UDP

```rust
// UDP speed with TCP reliability
let mut client = VstpUdpClient::bind("0.0.0.0:0").await?;
let critical_data = Frame::new(FrameType::Data)
    .with_header("transaction-id", "tx-123")
    .with_payload(transaction_data);

// Automatically retries until ACK received
client.send_with_ack(critical_data, server).await?;
```

### Pattern 5: Large File Transfer (UDP)

```rust
// Automatic fragmentation for large payloads
let file_data = std::fs::read("large_file.bin")?; // 50MB

let frame = Frame::new(FrameType::Data)
    .with_header("file-name", "large_file.bin")
    .with_header("file-size", &file_data.len().to_string())
    .with_payload(file_data);

// VSTP automatically fragments into ~41,000 chunks
// and reassembles on the server
client.send_with_ack(frame, server).await?;
```

---

## Feature Cheat Sheet

### Frame Types

```rust
FrameType::Hello    // Connection start
FrameType::Welcome  // Connection accept
FrameType::Data     // Application data
FrameType::Ping     // Keepalive request
FrameType::Pong     // Keepalive response
FrameType::Bye      // Graceful close
FrameType::Ack      // Acknowledgment
FrameType::Err      // Error notification
```

### Flags

```rust
Flags::REQ_ACK  // Request acknowledgment
Flags::CRC      // CRC checksum present
Flags::FRAG     // Fragmented frame
Flags::COMP     // Compressed payload
```

### Priority Levels

```rust
frame.priority()  // Returns 0-255
// Error: 255, ACK: 200, Control: 150, Keepalive: 100, Data: 50
```

### Encoding

```rust
encode_varint(123)           // Variable-length integer
encode_string("hello")       // Length-prefixed string
encode_frame(&frame)         // Complete frame
```

---

## Troubleshooting

### Issue: "Connection refused"

```bash
# Server not running
cargo run --bin server  # Start server first
```

### Issue: "Frame too large"

```rust
// Increase max frame size
let codec = VstpFrameCodec::new(16 * 1024 * 1024); // 16MB
```

### Issue: "ACK timeout"

```rust
// Increase timeout or retries
let config = UdpConfig::default()
    .ack_timeout(Duration::from_secs(5))
    .max_retries(5);

let client = VstpUdpClient::bind_with_config("0.0.0.0:0", config).await?;
```

### Issue: "CRC mismatch"

```bash
# Data corruption detected
# Check network quality
# Enable retransmission
```

---

## Next Steps

1. **Read** `ARCHITECTURE_GUIDE.md` for deep dive
2. **Explore** tests in `tests/` directory
3. **Benchmark** with `cargo bench`
4. **Extend** with custom extensions
5. **Deploy** to production

---

## Getting Help

- 📖 Read `ARCHITECTURE_GUIDE.md` for complete documentation
- 🧪 Check `TESTING_GUIDE.md` for testing help
- 🗺️ See `IMPLEMENTATION_ROADMAP.md` for future features
- 💬 Open GitHub issue for bugs
- 🌟 Star the repo if you like VSTP!

---

**Happy coding with VSTP! 🚀**
