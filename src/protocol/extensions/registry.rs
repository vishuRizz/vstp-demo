use std::collections::HashMap;
use std::sync::Arc;

use crate::core::types::{Frame, VstpError};
use super::handler::ExtensionHandler;

/// Registry for protocol extensions
#[derive(Default)]
pub struct ExtensionRegistry {
    handlers: HashMap<String, Arc<dyn ExtensionHandler>>,
}

impl ExtensionRegistry {
    /// Create a new extension registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new extension handler
    pub fn register<H>(&mut self, name: impl Into<String>, handler: H)
    where
        H: ExtensionHandler + 'static,
    {
        self.handlers.insert(name.into(), Arc::new(handler));
    }

    /// Process a frame through registered extensions
    pub async fn process_frame(&self, frame: Frame) -> Result<Frame, VstpError> {
        let mut current_frame = frame;

        // Process through each handler
        for handler in self.handlers.values() {
            if handler.should_handle(&current_frame) {
                current_frame = handler.handle_frame(current_frame).await?;
            }
        }

        Ok(current_frame)
    }

    /// Get a registered handler by name
    pub fn get_handler(&self, name: &str) -> Option<Arc<dyn ExtensionHandler>> {
        self.handlers.get(name).cloned()
    }

    /// Remove a registered handler
    pub fn unregister(&mut self, name: &str) -> Option<Arc<dyn ExtensionHandler>> {
        self.handlers.remove(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::FrameType;
    use async_trait::async_trait;

    struct TestHandler;

    #[async_trait]
    impl ExtensionHandler for TestHandler {
        fn should_handle(&self, frame: &Frame) -> bool {
            frame.typ == FrameType::Data
        }

        async fn handle_frame(&self, mut frame: Frame) -> Result<Frame, VstpError> {
            frame.headers.push(crate::core::types::Header::from_str(
                "test-extension",
                "processed",
            ));
            Ok(frame)
        }
    }

    #[tokio::test]
    async fn test_extension_registry() {
        let mut registry = ExtensionRegistry::new();
        registry.register("test", TestHandler);

        let frame = Frame::new(FrameType::Data);
        let processed = registry.process_frame(frame).await.unwrap();

        assert_eq!(
            processed.get_header("test-extension").unwrap(),
            "processed"
        );
    }
}
