#!/bin/bash
# Quick script to find your local IP address for VSTP server

echo "🌐 Finding your local IP address..."
echo ""

# Try different methods based on OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    IP=$(ifconfig | grep "inet " | grep -v 127.0.0.1 | awk '{print $2}' | head -1)
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    IP=$(hostname -I | awk '{print $1}')
else
    # Fallback
    IP=$(ipconfig getifaddr en0 2>/dev/null || ipconfig getifaddr en1 2>/dev/null || echo "Not found")
fi

if [ -z "$IP" ] || [ "$IP" == "Not found" ]; then
    echo "❌ Could not find IP address automatically"
    echo ""
    echo "Please find it manually:"
    echo "  macOS: ifconfig | grep 'inet '"
    echo "  Linux: hostname -I"
    echo "  Windows: ipconfig"
else
    echo "✅ Your local IP address: $IP"
    echo ""
    echo "📋 Use this on the client machine:"
    echo "   $IP:8080"
    echo ""
    echo "🚀 Start server with:"
    echo "   cargo run --example simple_server_ngrok"
    echo ""
    echo "💻 Connect from client with:"
    echo "   cargo run --example simple_client_direct_ip"
    echo "   (Enter: $IP:8080)"
fi

