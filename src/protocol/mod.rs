pub mod extensions;
pub mod compression;

// Re-export commonly used types
pub use extensions::registry::ExtensionRegistry;
pub use compression::CompressionConfig;
