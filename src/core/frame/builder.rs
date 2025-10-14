use crate::core::types::{Flags, Frame, FrameType, Header, VSTP_VERSION};

/// Builder for creating VSTP frames
pub struct FrameBuilder {
    frame: Frame,
}

impl FrameBuilder {
    /// Create a new frame builder with the specified type
    pub fn new(typ: FrameType) -> Self {
        Self {
            frame: Frame {
                version: VSTP_VERSION,
                typ,
                flags: Flags::empty(),
                headers: Vec::new(),
                payload: Vec::new(),
            },
        }
    }

    /// Add a header to the frame
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.frame.headers.push(Header::from_str(key, value));
        self
    }

    /// Add a binary header to the frame
    pub fn binary_header(mut self, key: Vec<u8>, value: Vec<u8>) -> Self {
        self.frame.headers.push(Header::new(key, value));
        self
    }

    /// Set the payload
    pub fn payload(mut self, payload: Vec<u8>) -> Self {
        self.frame.payload = payload;
        self
    }

    /// Add a flag
    pub fn flag(mut self, flag: Flags) -> Self {
        self.frame.flags |= flag;
        self
    }

    /// Build the frame
    pub fn build(self) -> Frame {
        self.frame
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let frame = FrameBuilder::new(FrameType::Data)
            .header("content-type", "text/plain")
            .payload(b"Hello".to_vec())
            .build();

        assert_eq!(frame.typ, FrameType::Data);
        assert_eq!(frame.headers.len(), 1);
        assert_eq!(frame.payload, b"Hello");
    }

    #[test]
    fn test_builder_with_flags() {
        let frame = FrameBuilder::new(FrameType::Data)
            .flag(Flags::REQ_ACK)
            .flag(Flags::CRC)
            .build();

        assert!(frame.flags.contains(Flags::REQ_ACK));
        assert!(frame.flags.contains(Flags::CRC));
    }

    #[test]
    fn test_builder_binary_headers() {
        let frame = FrameBuilder::new(FrameType::Data)
            .binary_header(vec![1, 2, 3], vec![4, 5, 6])
            .build();

        assert_eq!(frame.headers[0].key, vec![1, 2, 3]);
        assert_eq!(frame.headers[0].value, vec![4, 5, 6]);
    }
}
