use crc_any::CRC;

/// CRC validator for frame integrity checking
#[derive(Debug)]
pub struct CrcValidator {
    crc: CRC,
}

impl Clone for CrcValidator {
    fn clone(&self) -> Self {
        Self::new()  // Create a fresh CRC instance
    }
}

impl Default for CrcValidator {
    fn default() -> Self {
        Self {
            crc: CRC::crc32(),
        }
    }
}

impl CrcValidator {
    /// Create a new CRC validator
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate CRC for data
    pub fn calculate(&mut self, data: &[u8]) -> u32 {
        self.crc.digest(data);
        self.crc.get_crc() as u32
    }

    /// Verify CRC matches expected value
    pub fn verify(&mut self, data: &[u8], expected: u32) -> bool {
        let calculated = self.calculate(data);
        calculated == expected
    }

    /// Reset the CRC calculator
    pub fn reset(&mut self) {
        self.crc.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc_calculation() {
        let mut validator = CrcValidator::new();
        let data = b"Hello, World!";
        let crc1 = validator.calculate(data);
        validator.reset();
        let crc2 = validator.calculate(data);
        assert_eq!(crc1, crc2);
    }

    #[test]
    fn test_crc_verification() {
        let mut validator = CrcValidator::new();
        let data = b"Hello, World!";
        let crc = validator.calculate(data);
        validator.reset();
        assert!(validator.verify(data, crc));
    }

    #[test]
    fn test_crc_mismatch() {
        let mut validator = CrcValidator::new();
        let data = b"Hello, World!";
        let crc = validator.calculate(data);
        validator.reset();
        assert!(!validator.verify(b"Hello, world!", crc));
    }
}
