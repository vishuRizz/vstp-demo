# ⚡ VSTP Quick Demo - Show It Working in 30 Seconds!

## 🎯 The Shortest Way to Demo VSTP

### Step 1: Open TWO terminals

### Step 2: Terminal 1 - Start Server
```bash
cargo run --example simple_server
```

**You'll see:**
```
🚀 Starting VSTP Server on 127.0.0.1:8080
   Waiting for clients to connect...
```

### Step 3: Terminal 2 - Start Client
```bash
cargo run --example simple_client
```

**You'll see:**
```
🔌 Connecting to VSTP Server at 127.0.0.1:8080...
✅ Connected to server!

👋 Sent HELLO to server

Enter a message (or 'quit' to exit):
```

### Step 4: Type Messages!

**In Terminal 2 (Client):**
- Type: `Hello VSTP!`
- Press Enter

**In Terminal 1 (Server):**
- You'll see: `📨 Message from client 2: Hello VSTP!`

**Type more messages!** They appear instantly on the server.

**Type `quit`** to disconnect.

---

## 🎬 That's It!

You just demonstrated:
- ✅ VSTP protocol working
- ✅ TCP connection established
- ✅ Real-time message transfer
- ✅ Binary protocol encoding
- ✅ Frame-based communication

---

## 📝 Code Summary

**Server (3 lines of code!):**
```rust
let server = VstpTcpServer::bind("127.0.0.1:8080").await?;
server.run(|session_id, frame| async move {
    // Handle frames
}).await?;
```

**Client (2 lines of code!):**
```rust
let mut client = VstpTcpClient::connect("127.0.0.1:8080").await?;
client.send_data(message).await?;
```

**That's how simple VSTP is!** 🚀

