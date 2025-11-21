# 🌐 Direct IP Connection Setup (No ngrok Needed!)

## ✅ Use This When:
- Both devices are on the **same network** (WiFi/LAN)
- You have a **public IP** with port forwarding
- You're using a **VPN** (both devices connected)

## 📋 Setup Steps

### Step 1: Find Your Server's IP Address

**On the server machine (your laptop):**

**macOS/Linux:**
```bash
# Find local IP address
ifconfig | grep "inet " | grep -v 127.0.0.1
# Or
ip addr show | grep "inet " | grep -v 127.0.0.1
```

**Windows:**
```cmd
ipconfig
# Look for "IPv4 Address" under your network adapter
```

You'll see something like: `192.168.1.100` or `10.0.0.5`

### Step 2: Start VSTP Server

**On server machine:**
```bash
cargo run --example simple_server_ngrok
```

This binds to `0.0.0.0:8080` which accepts connections from any IP.

### Step 3: Connect from Remote Client

**On the other laptop:**

```bash
cargo run --example simple_client_direct_ip
```

When prompted, enter the server's IP address:
```
192.168.1.100:8080
```

Or just the IP (port 8080 is default):
```
192.168.1.100
```

## 🔍 Network Requirements

### ✅ Same Network (Easiest)
- Both devices on same WiFi
- Both devices on same LAN
- **No firewall/router configuration needed**

### 🌍 Public IP (Advanced)
- Server has public IP address
- Router port forwarding: `8080 -> server's local IP:8080`
- Firewall allows port 8080

### 🔒 VPN Connection
- Both devices connected to same VPN
- Use VPN-assigned IP addresses

## 🛠️ Troubleshooting

### Problem: "Connection refused"
**Solutions:**
1. Check firewall - allow port 8080
2. Verify server is running: `cargo run --example simple_server_ngrok`
3. Check IP address is correct
4. Make sure both devices are on same network

### Problem: "Cannot connect"
**Solutions:**
1. **macOS Firewall**: System Settings → Firewall → Allow incoming connections
2. **Windows Firewall**: Allow port 8080 in Windows Defender Firewall
3. **Router**: Check if router blocks local connections
4. **Try ping**: `ping 192.168.1.100` (replace with server IP)

### Problem: "Connection timeout"
**Solutions:**
1. Server might be on different network
2. Firewall blocking the connection
3. Router blocking local connections
4. Try disabling firewall temporarily to test

## 📝 Quick Commands

**Find IP Address:**
```bash
# macOS/Linux
hostname -I | awk '{print $1}'

# Or
ifconfig | grep "inet " | grep -v 127.0.0.1 | awk '{print $2}'
```

**Test Connection:**
```bash
# From client machine, test if port is open
telnet 192.168.1.100 8080
# Or
nc -zv 192.168.1.100 8080
```

## 🎯 Example Workflow

**Server Machine (192.168.1.100):**
```bash
# Terminal 1: Start server
cargo run --example simple_server_ngrok
```

**Client Machine (192.168.1.101):**
```bash
# Terminal 1: Connect to server
cargo run --example simple_client_direct_ip
# Enter: 192.168.1.100:8080
```

## 💡 Pro Tips

1. **Use local IP for same network** - Much faster than ngrok!
2. **Check firewall first** - Most connection issues are firewall-related
3. **Test with ping** - If ping works, network is fine
4. **Use `0.0.0.0` binding** - Server must bind to `0.0.0.0`, not `127.0.0.1`

## 🔒 Security Note

Direct IP connections are **unencrypted** by default. For production:
- Use TLS/SSL layer
- Use VPN for secure connections
- Consider firewall rules to restrict access

