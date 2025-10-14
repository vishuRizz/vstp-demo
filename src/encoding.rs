use crate::types::VstpError;
use bytes::{BufMut, Bytes, BytesMut};

/// Maximum number of bytes needed to encode a 64-bit integer
const MAX_VARINT_LEN: usize = 10;

/// Encodes a u64 as a variable-length integer
/// The encoding format is similar to Protocol Buffers' varint:
/// - Each byte uses 7 bits for the number
/// - The MSB (8th bit) indicates if more bytes follow
/// - Numbers are encoded in little-endian order
pub fn encode_varint(value: u64) -> Bytes {
    let mut buf = BytesMut::with_capacity(MAX_VARINT_LEN);
    let mut val = value;

    loop {
        let mut byte = (val & 0x7f) as u8;
        val >>= 7;
        if val != 0 {
            byte |= 0x80;
        }
        buf.put_u8(byte);
        if val == 0 {
            break;
        }
    }

    buf.freeze()
}

/// Decodes a variable-length integer from a byte slice
/// Returns the decoded value and the number of bytes read
pub fn decode_varint(buf: &[u8]) -> Result<(u64, usize), VstpError> {
    let mut result: u64 = 0;
    let mut shift: u32 = 0;
    let mut bytes_read = 0;

    for (i, &byte) in buf.iter().enumerate() {
        if i >= MAX_VARINT_LEN {
            return Err(VstpError::Protocol("Variable integer too long".to_string()));
        }

        bytes_read += 1;
        let value = (byte & 0x7f) as u64;
        result |= value << shift;

        if byte & 0x80 == 0 {
            return Ok((result, bytes_read));
        }

        shift += 7;
        if shift > 63 {
            return Err(VstpError::Protocol("Variable integer overflow".to_string()));
        }
    }

    Err(VstpError::Protocol(
        "Incomplete variable integer".to_string(),
    ))
}

/// Calculates the number of bytes needed to encode a value
pub fn varint_len(value: u64) -> usize {
    match value {
        0 => 1,
        v => (64 - v.leading_zeros() as usize + 6) / 7,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint_small_numbers() {
        let test_cases = vec![0, 1, 127, 128, 255, 256];

        for value in test_cases {
            let encoded = encode_varint(value);
            let (decoded, bytes_read) = decode_varint(&encoded).unwrap();
            assert_eq!(value, decoded);
            assert_eq!(bytes_read, encoded.len());
            assert_eq!(bytes_read, varint_len(value));
        }
    }

    #[test]
    fn test_varint_large_numbers() {
        let test_cases = vec![u64::MAX, u64::MAX / 2, 1 << 32, (1 << 32) - 1];

        for value in test_cases {
            let encoded = encode_varint(value);
            let (decoded, bytes_read) = decode_varint(&encoded).unwrap();
            assert_eq!(value, decoded);
            assert_eq!(bytes_read, encoded.len());
            assert_eq!(bytes_read, varint_len(value));
        }
    }

    #[test]
    fn test_varint_error_cases() {
        // Test incomplete varint
        let incomplete = vec![0x80];
        assert!(decode_varint(&incomplete).is_err());

        // Test overflow
        let overflow = vec![0x80; MAX_VARINT_LEN + 1];
        assert!(decode_varint(&overflow).is_err());
    }

    #[test]
    fn test_varint_encoding_efficiency() {
        // Test that small numbers use fewer bytes
        assert_eq!(encode_varint(0).len(), 1);
        assert_eq!(encode_varint(127).len(), 1);
        assert_eq!(encode_varint(128).len(), 2);
        assert_eq!(encode_varint(16383).len(), 2);
        assert_eq!(encode_varint(16384).len(), 3);
    }
}
