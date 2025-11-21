# 🔬 VSTP vs HTTP Comparison Tools

Compare VSTP and HTTP protocols side-by-side with detailed metrics.

## 📋 Available Examples

### 1. HTTP Server (`http_server_compare`)
Simple HTTP server that shows request structure and bytes.

**Run:**
```bash
cargo run --example http_server_compare
```

**Port:** `127.0.0.1:8081`

### 2. HTTP Client (`http_client_compare`)
HTTP client that sends messages and shows encoding.

**Run:**
```bash
cargo run --example http_client_compare
```

### 3. Metrics Comparison (`compare_metrics`)
Side-by-side comparison of VSTP vs HTTP with detailed metrics.

**Run:**
```bash
cargo run --example compare_metrics
```

## 🎯 How to Use

### Step 1: Start Both Servers

**Terminal 1 - VSTP Server:**
```bash
cargo run --example simple_server
```

**Terminal 2 - HTTP Server:**
```bash
cargo run --example http_server_compare
```

### Step 2: Test with Clients

**Terminal 3 - VSTP Client:**
```bash
cargo run --example simple_client
```

**Terminal 4 - HTTP Client:**
```bash
cargo run --example http_client_compare
```

### Step 3: Compare Metrics

**Terminal 5 - Metrics Tool:**
```bash
cargo run --example compare_metrics
```

## 📊 What Gets Compared

- **Size**: Total bytes, overhead, payload size
- **Security**: Injection risks, parsing complexity
- **Performance**: Parsing speed, connection time
- **Features**: Integrity checks, anomaly detection
- **Binary Representation**: Hex dumps of both protocols

## 🔍 Example Output

The comparison tool shows:
- Side-by-side size comparison table
- Security feature matrix
- Performance metrics
- Binary representation of both protocols
- Efficiency percentages

