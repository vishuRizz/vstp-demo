use bytes::{BufMut, Bytes, BytesMut};
use crate::core::types::VstpError;

/// Encodes a string as a length-prefixed binary string
pub fn encode_string(value: &str) -> Bytes {
    let mut buf = BytesMut::new();
    let bytes = value.as_bytes();
    buf.put_u16_le(bytes.len() as u16);
    buf.put_slice(bytes);
    buf.freeze()
}

/// Decodes a length-prefixed binary string
pub fn decode_string(buf: &[u8]) -> Result<(&str, usize), VstpError> {
    if buf.len() < 2 {
        return Err(VstpError::Protocol("Incomplete string length".to_string()));
    }

    let len = u16::from_le_bytes([buf[0], buf[1]]) as usize;
    let total_len = len + 2;

    if buf.len() < total_len {
        return Err(VstpError::Protocol("Incomplete string data".to_string()));
    }

    let string_data = &buf[2..total_len];
    match std::str::from_utf8(string_data) {
        Ok(s) => Ok((s, total_len)),
        Err(_) => Err(VstpError::Protocol("Invalid UTF-8 string".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_roundtrip() {
        let test_cases = vec![
            "",
            "Hello",
            "Hello, World!",
            "🦀 Rust",
            "Unicode: 你好, こんにちは",
        ];

        for test_str in test_cases {
            let encoded = encode_string(test_str);
            let (decoded, bytes_read) = decode_string(&encoded).unwrap();
            assert_eq!(test_str, decoded);
            assert_eq!(bytes_read, encoded.len());
        }
    }

    #[test]
    fn test_string_errors() {
        // Test incomplete length
        let incomplete = vec![0x80];
        assert!(decode_string(&incomplete).is_err());

        // Test incomplete data
        let incomplete = vec![0x05, 0x00, b'H', b'e', b'l'];
        assert!(decode_string(&incomplete).is_err());

        // Test invalid UTF-8
        let invalid = vec![0x01, 0x00, 0xFF];
        assert!(decode_string(&invalid).is_err());
    }
}
