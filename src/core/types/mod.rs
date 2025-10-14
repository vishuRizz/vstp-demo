mod error;
mod flags;

pub use error::VstpError;
pub use flags::Flags;

/// VSTP protocol constants
pub const VSTP_MAGIC: [u8; 2] = [0x56, 0x54]; // "VT"
pub const VSTP_VERSION: u8 = 0x01;

/// Session identifier for tracking connections
pub type SessionId = u128;

/// Header key-value pair
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

impl Header {
    pub fn new(key: Vec<u8>, value: Vec<u8>) -> Self {
        Self { key, value }
    }

    pub fn from_str(key: &str, value: &str) -> Self {
        Self {
            key: key.as_bytes().to_vec(),
            value: value.as_bytes().to_vec(),
        }
    }
}

/// VSTP frame types
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    Hello = 0x01,
    Welcome = 0x02,
    Data = 0x03,
    Ping = 0x04,
    Pong = 0x05,
    Bye = 0x06,
    Ack = 0x07,
    Err = 0x08,
}

impl FrameType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(FrameType::Hello),
            0x02 => Some(FrameType::Welcome),
            0x03 => Some(FrameType::Data),
            0x04 => Some(FrameType::Ping),
            0x05 => Some(FrameType::Pong),
            0x06 => Some(FrameType::Bye),
            0x07 => Some(FrameType::Ack),
            0x08 => Some(FrameType::Err),
            _ => None,
        }
    }
}

/// Complete VSTP frame
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub version: u8,
    pub typ: FrameType,
    pub flags: Flags,
    pub headers: Vec<Header>,
    pub payload: Vec<u8>,
}

impl Frame {
    pub fn new(typ: FrameType) -> Self {
        Self {
            version: VSTP_VERSION,
            typ,
            flags: Flags::empty(),
            headers: Vec::new(),
            payload: Vec::new(),
        }
    }

    pub fn with_payload(mut self, payload: Vec<u8>) -> Self {
        self.payload = payload;
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.push(Header::from_str(key, value));
        self
    }

    pub fn with_flag(mut self, flag: Flags) -> Self {
        self.flags |= flag;
        self
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn frame_type(&self) -> FrameType {
        self.typ
    }

    pub fn get_header(&self, key: &str) -> Option<&str> {
        let key_bytes = key.as_bytes();
        self.headers.iter()
            .find(|h| h.key == key_bytes)
            .and_then(|h| std::str::from_utf8(&h.value).ok())
    }
}
