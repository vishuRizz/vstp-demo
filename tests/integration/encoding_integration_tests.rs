use vstp::{
    encoding::{encode_varint, decode_varint},
    types::{Frame, FrameType, VstpError},
};

#[test]
fn test_frame_with_varint() {
    // Test encoding frame lengths using varint
    let test_cases = vec![
        // Small payload
        vec![0u8; 10],
        // Medium payload
        vec![0u8; 1000],
        // Large payload
        vec![0u8; 100_000],
    ];

    for payload in test_cases {
        // Create a frame with the test payload
        let frame = Frame::new(FrameType::Data)
            .with_header("test", "header")
            .with_payload(payload.clone());

        // Encode the payload length
        let encoded_len = encode_varint(payload.len() as u64);
        
        // Decode and verify
        let (decoded_len, bytes_read) = decode_varint(&encoded_len).unwrap();
        assert_eq!(decoded_len as usize, payload.len());
        
        // Verify the encoding is efficient
        match payload.len() {
            0..=127 => assert_eq!(bytes_read, 1),
            128..=16383 => assert_eq!(bytes_read, 2),
            16384..=2097151 => assert_eq!(bytes_read, 3),
            _ => assert!(bytes_read <= 10),
        }
    }
}

#[test]
fn test_frame_headers_with_varint() {
    // Test encoding header counts and lengths using varint
    let mut frame = Frame::new(FrameType::Data);
    
    // Add increasing numbers of headers
    for i in 0..10 {
        frame = frame.with_header(
            &format!("key{}", i),
            &"a".repeat(i * 100), // Increasing value sizes
        );
    }

    // Encode header count
    let encoded_count = encode_varint(frame.headers.len() as u64);
    let (decoded_count, _) = decode_varint(&encoded_count).unwrap();
    assert_eq!(decoded_count as usize, frame.headers.len());

    // Test each header's length encoding
    for header in &frame.headers {
        let encoded_key_len = encode_varint(header.key.len() as u64);
        let encoded_val_len = encode_varint(header.value.len() as u64);

        let (decoded_key_len, _) = decode_varint(&encoded_key_len).unwrap();
        let (decoded_val_len, _) = decode_varint(&encoded_val_len).unwrap();

        assert_eq!(decoded_key_len as usize, header.key.len());
        assert_eq!(decoded_val_len as usize, header.value.len());
    }
}

#[test]
fn test_error_conditions() {
    // Test incomplete varint
    let incomplete = vec![0x80];
    assert!(matches!(
        decode_varint(&incomplete),
        Err(VstpError::Protocol(_))
    ));

    // Test overflow
    let overflow = vec![0x80; 11]; // More than MAX_VARINT_LEN
    assert!(matches!(
        decode_varint(&overflow),
        Err(VstpError::Protocol(_))
    ));

    // Test empty input
    let empty: Vec<u8> = vec![];
    assert!(matches!(
        decode_varint(&empty),
        Err(VstpError::Protocol(_))
    ));
}

#[test]
fn test_large_frame_encoding() {
    // Test with a frame that has large headers and payload
    let large_frame = Frame::new(FrameType::Data)
        .with_header("large_key", &"a".repeat(16384))
        .with_payload(vec![0u8; 1_000_000]);

    // Encode sizes
    let encoded_header_len = encode_varint(16384);
    let encoded_payload_len = encode_varint(1_000_000);

    // Decode and verify
    let (decoded_header_len, _) = decode_varint(&encoded_header_len).unwrap();
    let (decoded_payload_len, _) = decode_varint(&encoded_payload_len).unwrap();

    assert_eq!(decoded_header_len, 16384);
    assert_eq!(decoded_payload_len, 1_000_000);
}

#[test]
fn test_encoding_efficiency() {
    // Test that the encoding is efficient for various sizes
    let test_cases = vec![
        (0, 1),              // Single byte
        (127, 1),            // Max single byte
        (128, 2),            // Min two bytes
        (16383, 2),          // Max two bytes
        (16384, 3),          // Min three bytes
        (2097151, 3),        // Max three bytes
        (2097152, 4),        // Min four bytes
    ];

    for (value, expected_bytes) in test_cases {
        let encoded = encode_varint(value);
        assert_eq!(encoded.len(), expected_bytes, 
            "Value {} should encode to {} bytes", value, expected_bytes);
    }
}
