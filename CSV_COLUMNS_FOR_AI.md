# 🤖 CSV Data Schema for AI Model Training

## Quick Reference - All CSV Columns Needed

Copy this list for your AI to generate training data:

### Required Columns (40+ features):

```
session_id
frame_count
byte_count
connection_duration_seconds
frames_per_second
bytes_per_second
avg_frame_size
max_frame_size
min_frame_size
frame_size_std
frame_size_variance
hello_frames
data_frames
ack_frames
ping_frames
pong_frames
bye_frames
error_frames
data_frame_ratio
control_frame_ratio
ack_frame_ratio
avg_inter_arrival_time_ms
min_inter_arrival_time_ms
max_inter_arrival_time_ms
inter_arrival_std_ms
inter_arrival_variance_ms
timing_consistency_score
crc_errors
protocol_errors
suspicious_flags
total_errors
error_rate
crc_error_rate
protocol_error_rate
peer_ip
peer_port
is_localhost
frame_size_z_score
fps_z_score
bps_z_score
inter_arrival_z_score
burst_score
steady_score
irregular_score
connection_established
connection_closed_gracefully
has_ping_pong
uses_ack
is_anomaly
threat_type
```

### Target Variables (Labels):
- `is_anomaly` - 0 (normal) or 1 (anomaly)
- `threat_type` - One of: normal, packet_theft, mitm, replay_attack, connection_flood, data_exfiltration, protocol_violation, timing_attack, suspicious_activity, anomalous_traffic

### Data Types:
- **Integers**: session_id, frame_count, byte_count, max_frame_size, min_frame_size, all *_frames counts, crc_errors, protocol_errors, suspicious_flags, total_errors, peer_port
- **Floats**: All *_ratio, *_per_second, *_score, *_z_score, *_ms, *_std, *_variance, error_rate, connection_duration_seconds
- **Booleans (0/1)**: is_localhost, connection_established, connection_closed_gracefully, has_ping_pong, uses_ack, is_anomaly
- **Strings**: peer_ip, threat_type

### Example Row:
```csv
12345,150,45000,5.2,28.85,8653.85,300.0,1024,64,125.5,0.0,1,145,4,0,0,0,0.967,0.033,0.027,34.67,30.0,40.0,2.1,4.41,0.95,0,0,0,0,0.0,0.0,0.0,127.0.0.1,54321,1,0.5,1.2,0.8,0.3,0.2,0.7,0.3,1,1,0,1,1,connection_flood
```

This is what your AI needs to generate! 🚀

