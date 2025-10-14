use crate::core::types::VstpError;

/// Configuration for frame compression
#[derive(Debug, Clone)]
pub struct CompressionConfig {
    /// Minimum size for compression (bytes)
    pub min_size: usize,
    /// Compression level (0-9)
    pub level: u32,
    /// Whether to compress headers
    pub compress_headers: bool,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            min_size: 1024,  // Only compress payloads >= 1KB
            level: 6,        // Default compression level
            compress_headers: false,
        }
    }
}

impl CompressionConfig {
    /// Create a new compression configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set minimum size for compression
    pub fn min_size(mut self, size: usize) -> Self {
        self.min_size = size;
        self
    }

    /// Set compression level
    pub fn level(mut self, level: u32) -> Self {
        self.level = level.min(9);
        self
    }

    /// Enable or disable header compression
    pub fn compress_headers(mut self, enable: bool) -> Self {
        self.compress_headers = enable;
        self
    }
}

/// Compress data using the specified configuration
pub fn compress(data: &[u8], config: &CompressionConfig) -> Result<Vec<u8>, VstpError> {
    if data.len() < config.min_size {
        return Ok(data.to_vec());
    }

    let mut encoder = flate2::write::GzEncoder::new(
        Vec::new(),
        flate2::Compression::new(config.level),
    );
    std::io::Write::write_all(&mut encoder, data)
        .map_err(|e| VstpError::Protocol(format!("Compression error: {}", e)))?;
    encoder.finish()
        .map_err(|e| VstpError::Protocol(format!("Compression finish error: {}", e)))
}

/// Decompress data
pub fn decompress(data: &[u8]) -> Result<Vec<u8>, VstpError> {
    let mut decoder = flate2::write::GzDecoder::new(Vec::new());
    std::io::Write::write_all(&mut decoder, data)
        .map_err(|e| VstpError::Protocol(format!("Decompression error: {}", e)))?;
    decoder.finish()
        .map_err(|e| VstpError::Protocol(format!("Decompression finish error: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_config() {
        let config = CompressionConfig::new()
            .min_size(2048)
            .level(9)
            .compress_headers(true);

        assert_eq!(config.min_size, 2048);
        assert_eq!(config.level, 9);
        assert!(config.compress_headers);
    }

    #[test]
    fn test_compression_roundtrip() {
        let config = CompressionConfig::new();
        let data = vec![0u8; 2048];
        let compressed = compress(&data, &config).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, decompressed);
    }

    #[test]
    fn test_small_data_no_compression() {
        let config = CompressionConfig::new();
        let data = vec![0u8; 512];  // Smaller than min_size
        let result = compress(&data, &config).unwrap();
        assert_eq!(data, result);  // Should return original data
    }
}
