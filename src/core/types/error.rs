use thiserror::Error;
use crate::core::types::VSTP_MAGIC;

/// VSTP error types
#[derive(Error, Debug)]
pub enum VstpError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Invalid version: expected {expected}, got {got}")]
    InvalidVersion { expected: u8, got: u8 },

    #[error("Invalid frame type: {0}")]
    InvalidFrameType(u8),

    #[error("Invalid magic bytes: expected {:?}, got {:?}", VSTP_MAGIC, .0)]
    InvalidMagic([u8; 2]),

    #[error("CRC mismatch: expected {expected}, got {got}")]
    CrcMismatch { expected: u32, got: u32 },

    #[error("Incomplete frame: need {needed} more bytes")]
    Incomplete { needed: usize },

    #[error("Frame too large: {size} bytes exceeds limit of {limit}")]
    FrameTooLarge { size: usize, limit: usize },

    #[error("Operation timed out")]
    Timeout,

    #[error("Invalid address")]
    InvalidAddress,

    #[error("Serialization error")]
    SerializationError,

    #[error("Deserialization error")]
    DeserializationError,

    #[error("Unexpected frame type")]
    UnexpectedFrameType,

    #[error("Connection closed")]
    ConnectionClosed,

    #[error("Server error: {0}")]
    ServerError(String),
}
