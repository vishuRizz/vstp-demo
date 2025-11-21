# 🌐 ngrok Setup for VSTP

## ⚠️ Important: Use TCP Forwarding, Not HTTP!

VSTP is a **raw TCP protocol**, so you need **TCP forwarding**, not HTTP forwarding.

## 📋 Setup Steps

### Step 1: Start ngrok with TCP Forwarding

**Stop your current ngrok** (if running HTTP forwarding) and run:

```bash
ngrok tcp 8080
```

You should see output like:

```
Forwarding    tcp://0.tcp.ngrok.io:12345 -> localhost:8080
```

**Copy the TCP address** (e.g., `0.tcp.ngrok.io:12345`)

### Step 2: Start VSTP Server

**Terminal 1:**

```bash
cargo run --example simple_server_ngrok
```

This binds to `0.0.0.0:8080` to accept connections from ngrok.

### Step 3: Connect from Remote Client

**On another laptop/device:**

```bash
cargo run --example simple_client_ngrok
```

When prompted, enter the ngrok TCP address:

```
0.tcp.ngrok.io:12345
```

Or set environment variable:

```bash
export NGROK_ADDR="0.tcp.ngrok.io:12345"
cargo run --example simple_client_ngrok
```

## 🔍 Troubleshooting

### Problem: "Connection refused"

- Make sure ngrok is running with `ngrok tcp 8080` (not `ngrok http 8080`)
- Check that the server is running on port 8080
- Verify the ngrok TCP address is correct

### Problem: "Cannot connect"

- ngrok free tier has connection limits
- Try restarting ngrok to get a new address
- Make sure firewall allows connections

### Problem: "Wrong protocol"

- You're using HTTP forwarding instead of TCP
- Stop ngrok and restart with: `ngrok tcp 8080`

## 📝 Quick Reference

**Server (your machine):**

```bash
# Terminal 1: Start ngrok
ngrok tcp 8080

# Terminal 2: Start VSTP server
cargo run --example simple_server_ngrok
```

**Client (remote machine):**

```bash
# Get ngrok TCP address from server's ngrok output
# Then run:
cargo run --example simple_client_ngrok
# Enter: 0.tcp.ngrok.io:12345 (or whatever ngrok shows)
```

## 🎯 Example ngrok Output

```
Session Status                online
Account                       vishu pratap (Plan: Free)
Version                       3.18.4
Region                        India (in)
Forwarding                    tcp://0.tcp.ngrok.io:12345 -> localhost:8080
```

Use: `0.tcp.ngrok.io:12345` (without the `tcp://` prefix)
