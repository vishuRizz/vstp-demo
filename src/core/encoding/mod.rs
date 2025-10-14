pub mod binary;
pub mod varint;

pub use varint::{decode_varint, encode_varint, varint_len};
