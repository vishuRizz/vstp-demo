# 🚀 VSTP Quick Demo Guide

The **shortest way** to demonstrate VSTP working!

## 📋 Prerequisites

Make sure you have Rust installed:
```bash
rustc --version
```

## 🎯 TCP Demo (Recommended - Easiest)

### Terminal 1 - Start the Server:
```bash
cargo run --example simple_server
```

You should see:
```
🚀 Starting VSTP Server on 127.0.0.1:8080
   Waiting for clients to connect...
```

### Terminal 2 - Connect the Client:
```bash
cargo run --example simple_client
```

You should see:
```
🔌 Connecting to VSTP Server at 127.0.0.1:8080...
✅ Connected to server!

👋 Sent HELLO to server

Enter a message (or 'quit' to exit):
```

**Type messages and press Enter!** The server will show them in Terminal 1.

Type `quit` to disconnect.

---

## 🎯 UDP Demo

### Terminal 1 - Start the UDP Server:
```bash
cargo run --example simple_udp_server
```

### Terminal 2 - Connect the UDP Client:
```bash
cargo run --example simple_udp_client
```

Works the same way as TCP!

---

## 🎬 What to Show Someone

1. **Start the server** - Show it's waiting for connections
2. **Start the client** - Show it connects successfully
3. **Send messages** - Type messages in client, show them appear on server
4. **Show the protocol working** - Real-time communication!

---

## 💡 Quick Commands Cheat Sheet

```bash
# TCP Server
cargo run --example simple_server

# TCP Client (in another terminal)
cargo run --example simple_client

# UDP Server
cargo run --example simple_udp_server

# UDP Client (in another terminal)
cargo run --example simple_udp_client
```

---

## 🔍 What's Happening Behind the Scenes

- **HELLO frame**: Client announces itself
- **DATA frames**: Your messages are sent as binary frames
- **BYE frame**: Graceful disconnection

All using the **VSTP protocol** with binary encoding, framing, and error handling!

