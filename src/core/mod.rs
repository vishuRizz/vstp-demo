pub mod encoding;
pub mod frame;
pub mod types;

// Re-export commonly used types
pub use encoding::varint::{decode_varint, encode_varint, varint_len};
pub use types::{Flags, Frame, FrameType, Header, VstpError};
