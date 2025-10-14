use async_trait::async_trait;
use crate::core::types::{Frame, VstpError};

/// Trait for implementing protocol extensions
#[async_trait]
pub trait ExtensionHandler: Send + Sync {
    /// Check if this handler should process the frame
    fn should_handle(&self, frame: &Frame) -> bool;

    /// Process a frame
    async fn handle_frame(&self, frame: Frame) -> Result<Frame, VstpError>;
}
