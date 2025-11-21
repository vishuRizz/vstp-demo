# 📊 VSTP ML Training Data Schema

## 🎯 For LSTM Autoencoder Training

**IMPORTANT**: LSTM models need **sequential/temporal data**. You have two options:

### Option 1: Sequence-Based Format (RECOMMENDED for LSTM) ⭐

- **Multiple rows per session** (one row per time step/frame)
- Each row represents a single frame or a time window
- Better for LSTM because it captures temporal patterns
- See "Sequence-Based Schema" section below

### Option 2: Aggregated Format (Current Schema)

- **One row per session** (all features aggregated)
- Can work with LSTM but less ideal
- Better for traditional ML models
- See "Connection-Level Features" section below

---

## 📈 Sequence-Based Schema (BEST for LSTM Autoencoder)

### Format: Multiple rows per session, one per time step

Each row represents a **single frame** or a **time window** (e.g., every 10 frames or every 1 second).

### Required Columns:

#### Session Identification

- **`session_id`** (integer) - Same session_id for all rows in a sequence
- **`sequence_index`** (integer) - Position in sequence (0, 1, 2, ...)
- **`timestamp_ms`** (integer) - Timestamp in milliseconds since session start

#### Frame-Level Features (Per Row)

- **`frame_type`** (integer) - Frame type: 1=Hello, 2=Welcome, 3=Data, 4=Ping, 5=Pong, 6=Bye, 7=Ack, 8=Err
- **`frame_size`** (integer) - Size of this frame in bytes
- **`payload_size`** (integer) - Payload size in bytes
- **`has_flags`** (boolean: 0 or 1) - Whether frame has flags set
- **`header_count`** (integer) - Number of headers in frame

#### Timing Features (Per Row)

- **`time_since_last_frame_ms`** (float) - Time since previous frame (0 for first frame)
- **`time_since_session_start_ms`** (float) - Time since session started
- **`cumulative_frame_count`** (integer) - Total frames so far in session
- **`cumulative_byte_count`** (integer) - Total bytes so far in session

#### Running Statistics (Per Row - Calculated from history)

- **`avg_frame_size_so_far`** (float) - Average frame size up to this point
- **`frames_per_second_so_far`** (float) - FPS calculated up to this point
- **`bytes_per_second_so_far`** (float) - BPS calculated up to this point
- **`data_frame_ratio_so_far`** (float) - Ratio of data frames up to this point

#### Error Indicators (Per Row)

- **`is_crc_error`** (boolean: 0 or 1) - This frame had CRC error
- **`is_protocol_error`** (boolean: 0 or 1) - This frame had protocol error
- **`cumulative_errors`** (integer) - Total errors so far
- **`error_rate_so_far`** (float) - Error rate up to this point

#### Label (Per Row or Per Session)

- **`is_anomaly`** (boolean: 0 or 1) - Whether this session is anomalous
- **`threat_type`** (string) - Type of threat (same for all rows in session)

### Example Sequence-Based CSV:

```csv
session_id,sequence_index,timestamp_ms,frame_type,frame_size,payload_size,time_since_last_frame_ms,time_since_session_start_ms,cumulative_frame_count,avg_frame_size_so_far,frames_per_second_so_far,data_frame_ratio_so_far,is_anomaly,threat_type
1001,0,0,1,64,0,0.0,0.0,1,64.0,0.0,0.0,0,normal
1001,1,50,3,300,256,50.0,50.0,2,182.0,20.0,0.5,0,normal
1001,2,100,3,320,276,50.0,100.0,3,228.0,20.0,0.67,0,normal
1001,3,150,3,310,266,50.0,150.0,4,248.5,20.0,0.75,0,normal
1002,0,0,1,64,0,0.0,0.0,1,64.0,0.0,0.0,1,connection_flood
1002,1,5,3,200,156,5.0,5.0,2,132.0,200.0,0.5,1,connection_flood
1002,2,10,3,200,156,5.0,10.0,3,154.7,200.0,0.67,1,connection_flood
1002,3,15,3,200,156,5.0,15.0,4,166.0,200.0,0.75,1,connection_flood
```

### How to Structure for LSTM:

1. **Group by session_id** - All rows with same session_id form one sequence
2. **Sort by sequence_index** - Ensure chronological order
3. **Fixed sequence length** - Pad or truncate to same length (e.g., 100 time steps)
4. **Feature selection** - Use 10-20 most important features per time step

### Recommended Features Per Time Step (20 features):

```
1. frame_type
2. frame_size
3. payload_size
4. time_since_last_frame_ms
5. cumulative_frame_count
6. cumulative_byte_count
7. avg_frame_size_so_far
8. frames_per_second_so_far
9. bytes_per_second_so_far
10. data_frame_ratio_so_far
11. is_crc_error
12. is_protocol_error
13. cumulative_errors
14. error_rate_so_far
15. has_flags
16. header_count
17. time_since_session_start_ms (normalized)
18. frame_type_one_hot_0 (if using one-hot encoding)
19. frame_type_one_hot_1
20. frame_type_one_hot_2
```

---

## CSV Columns for Anomaly Detection Model Training (Aggregated Format)

### 🎯 Target Variable (Label)

- **`is_anomaly`** (boolean: 0 or 1) - Whether this connection/session is anomalous
- **`threat_type`** (string) - Type of threat detected:
  - `normal` - No threat
  - `packet_theft` - Packet theft/sniffing
  - `mitm` - Man-in-the-Middle attack
  - `replay_attack` - Replay attack
  - `connection_flood` - DDoS/flooding attack
  - `data_exfiltration` - Data exfiltration
  - `protocol_violation` - Protocol violation/fuzzing
  - `timing_attack` - Timing-based attack
  - `suspicious_activity` - General suspicious activity
  - `anomalous_traffic` - Unusual traffic pattern

### 📈 Connection-Level Features

#### Basic Statistics

- **`session_id`** (integer) - Unique session identifier
- **`frame_count`** (integer) - Total number of frames in session
- **`byte_count`** (integer) - Total bytes transferred
- **`connection_duration_seconds`** (float) - Duration of connection in seconds
- **`frames_per_second`** (float) - Average frames per second
- **`bytes_per_second`** (float) - Average bytes per second

#### Frame Size Statistics

- **`avg_frame_size`** (float) - Average frame size in bytes
- **`max_frame_size`** (integer) - Maximum frame size
- **`min_frame_size`** (integer) - Minimum frame size
- **`frame_size_std`** (float) - Standard deviation of frame sizes
- **`frame_size_variance`** (float) - Variance of frame sizes

#### Frame Type Distribution (Counts)

- **`hello_frames`** (integer) - Number of HELLO frames
- **`data_frames`** (integer) - Number of DATA frames
- **`ack_frames`** (integer) - Number of ACK frames
- **`ping_frames`** (integer) - Number of PING frames
- **`pong_frames`** (integer) - Number of PONG frames
- **`bye_frames`** (integer) - Number of BYE frames
- **`error_frames`** (integer) - Number of ERROR frames

#### Frame Type Ratios (Percentages)

- **`data_frame_ratio`** (float) - Ratio of DATA frames to total (0.0 to 1.0)
- **`control_frame_ratio`** (float) - Ratio of control frames (HELLO, BYE, etc.)
- **`ack_frame_ratio`** (float) - Ratio of ACK frames

#### Timing Features

- **`avg_inter_arrival_time_ms`** (float) - Average time between frames in milliseconds
- **`min_inter_arrival_time_ms`** (float) - Minimum inter-arrival time
- **`max_inter_arrival_time_ms`** (float) - Maximum inter-arrival time
- **`inter_arrival_std_ms`** (float) - Standard deviation of inter-arrival times
- **`inter_arrival_variance_ms`** (float) - Variance of inter-arrival times
- **`timing_consistency_score`** (float) - How consistent timing is (0.0 to 1.0, higher = more consistent, suspicious for packet theft)

#### Error Statistics

- **`crc_errors`** (integer) - Number of CRC errors
- **`protocol_errors`** (integer) - Number of protocol errors
- **`suspicious_flags`** (integer) - Number of frames with suspicious flags
- **`total_errors`** (integer) - Total errors (CRC + protocol + suspicious)
- **`error_rate`** (float) - Error rate (errors / total frames, 0.0 to 1.0)
- **`crc_error_rate`** (float) - CRC error rate
- **`protocol_error_rate`** (float) - Protocol error rate

#### Network Features

- **`peer_ip`** (string) - Client IP address
- **`peer_port`** (integer) - Client port
- **`is_localhost`** (boolean: 0 or 1) - Whether connection is from localhost

### 🔍 Derived Features (Statistical Indicators)

#### Z-Scores (for anomaly detection)

- **`frame_size_z_score`** (float) - Z-score of frame size vs baseline
- **`fps_z_score`** (float) - Z-score of frames per second vs baseline
- **`bps_z_score`** (float) - Z-score of bytes per second vs baseline
- **`inter_arrival_z_score`** (float) - Z-score of inter-arrival time vs baseline

#### Pattern Indicators

- **`burst_score`** (float) - Indicates burst traffic patterns (0.0 to 1.0)
- **`steady_score`** (float) - Indicates steady traffic patterns (0.0 to 1.0)
- **`irregular_score`** (float) - Indicates irregular patterns (0.0 to 1.0)

#### Behavioral Features

- **`connection_established`** (boolean: 0 or 1) - Whether HELLO was sent
- **`connection_closed_gracefully`** (boolean: 0 or 1) - Whether BYE was sent
- **`has_ping_pong`** (boolean: 0 or 1) - Whether ping/pong was used
- **`uses_ack`** (boolean: 0 or 1) - Whether ACK frames were used

### 📋 Example CSV Row

```csv
session_id,frame_count,byte_count,connection_duration_seconds,frames_per_second,bytes_per_second,avg_frame_size,max_frame_size,min_frame_size,frame_size_std,hello_frames,data_frames,ack_frames,data_frame_ratio,avg_inter_arrival_time_ms,inter_arrival_std_ms,timing_consistency_score,crc_errors,protocol_errors,error_rate,total_errors,frame_size_z_score,fps_z_score,bps_z_score,is_anomaly,threat_type
12345,150,45000,5.2,28.85,8653.85,300.0,1024,64,125.5,1,145,4,0.967,34.67,2.1,0.95,0,0,0.0,0,0.5,1.2,0.8,1,connection_flood
```

### 🎯 Key Features for Each Attack Type

#### Packet Theft Detection

- `timing_consistency_score` (high = suspicious)
- `inter_arrival_std_ms` (low = suspicious)
- `avg_inter_arrival_time_ms` (very consistent = suspicious)

#### MITM Detection

- `error_rate` (high = suspicious)
- `crc_errors` (high = suspicious)
- `protocol_errors` (high = suspicious)

#### Connection Flood Detection

- `frames_per_second` (very high = suspicious)
- `bytes_per_second` (very high = suspicious)
- `fps_z_score` (high = suspicious)
- `burst_score` (high = suspicious)

#### Data Exfiltration Detection

- `data_frame_ratio` (very high > 0.95 = suspicious)
- `byte_count` (very high = suspicious)
- `data_frames` (high count = suspicious)

#### Protocol Violation Detection

- `protocol_errors` (any = suspicious)
- `error_frames` (any = suspicious)
- `suspicious_flags` (any = suspicious)

### 📝 Notes for Data Collection

1. **Sample Rate**: Collect data every N frames (e.g., every 10 frames) or every N seconds
2. **Window Size**: Use sliding windows (e.g., last 100 frames or last 30 seconds)
3. **Normalization**: Some features may need normalization (0-1 scale) before training
4. **Missing Values**: Handle missing values (e.g., use 0 for counts, -1 for undefined)
5. **Class Balance**: Ensure balanced dataset (equal normal vs anomaly samples)

### 🔢 Minimum Required Features (Core Set)

If you need to start with fewer features, these are the most important:

1. `frame_count`
2. `byte_count`
3. `frames_per_second`
4. `bytes_per_second`
5. `avg_frame_size`
6. `data_frame_ratio`
7. `error_rate`
8. `avg_inter_arrival_time_ms`
9. `timing_consistency_score`
10. `is_anomaly` (label)

---

## 🎯 LSTM-Specific Data Preparation

### For LSTM Autoencoder, use **Sequence-Based Format**:

1. **Data Collection**: Collect frame-by-frame data (not aggregated)
2. **Sequence Length**: Use fixed length (e.g., 100 frames per sequence)
3. **Padding**: Pad shorter sequences with zeros
4. **Normalization**: Normalize all features to 0-1 range
5. **Feature Selection**: Use 15-20 features per time step

### Data Shape for LSTM:

```
Input Shape: (batch_size, sequence_length, num_features)
Example: (32, 100, 20)
- 32 samples per batch
- 100 time steps per sequence
- 20 features per time step
```

### Training Data Split:

- **Normal Traffic**: 70-80% of data (for training autoencoder)
- **Anomalous Traffic**: 20-30% of data (for validation/testing)
- **Sequence Length**: 50-200 time steps (recommended: 100)

### Preprocessing Steps:

1. **Normalize features**: Use MinMaxScaler (0 to 1)
2. **Handle missing values**: Fill with 0 or forward-fill
3. **Create sequences**: Group by session_id, sort by timestamp
4. **Pad sequences**: Pad to fixed length with zeros
5. **Split data**: Train/validation/test split (70/15/15)

---

## 📝 Summary: Which Format to Use?

### Use **Sequence-Based Format** if:

- ✅ Training LSTM Autoencoder (RECOMMENDED)
- ✅ Want to capture temporal patterns
- ✅ Have frame-level data available
- ✅ Need to detect timing-based attacks

### Use **Aggregated Format** if:

- ✅ Training traditional ML models (SVM, Random Forest, etc.)
- ✅ Only have session-level statistics
- ✅ Quick prototyping
- ✅ Don't need temporal patterns

**For LSTM: Use Sequence-Based Format!** ⭐
