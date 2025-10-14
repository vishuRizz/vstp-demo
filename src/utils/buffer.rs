use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::ops::{Deref, DerefMut};

/// A smart buffer that can be used for both reading and writing
#[derive(Debug)]
pub struct Buffer {
    inner: BytesMut,
}

impl Buffer {
    /// Create a new empty buffer
    pub fn new() -> Self {
        Self {
            inner: BytesMut::new(),
        }
    }

    /// Create a buffer with the specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: BytesMut::with_capacity(capacity),
        }
    }

    /// Create a buffer from existing bytes
    pub fn from_bytes(bytes: impl Into<Bytes>) -> Self {
        Self {
            inner: BytesMut::from(&bytes.into()[..]),
        }
    }

    /// Write bytes to the buffer
    pub fn write(&mut self, bytes: &[u8]) {
        self.inner.put_slice(bytes);
    }

    /// Write a string to the buffer
    pub fn write_str(&mut self, s: &str) {
        self.write(s.as_bytes());
    }

    /// Read bytes from the buffer
    pub fn read(&mut self, len: usize) -> Option<Bytes> {
        if self.inner.len() >= len {
            Some(self.inner.split_to(len).freeze())
        } else {
            None
        }
    }

    /// Read all remaining bytes from the buffer
    pub fn read_all(&mut self) -> Bytes {
        self.inner.split().freeze()
    }

    /// Get the number of bytes in the buffer
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Get the buffer's capacity
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Convert the buffer into a Bytes object
    pub fn freeze(self) -> Bytes {
        self.inner.freeze()
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Buffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_write_read() {
        let mut buf = Buffer::new();
        
        // Write data
        buf.write(b"Hello");
        buf.write(b" World");
        
        // Read data
        assert_eq!(&buf.read(5).unwrap()[..], b"Hello");
        assert_eq!(&buf.read_all()[..], b" World");
    }

    #[test]
    fn test_buffer_from_bytes() {
        let data = Bytes::from(&b"Test Data"[..]);
        let buf = Buffer::from_bytes(data);
        
        assert_eq!(&buf[..], b"Test Data");
    }

    #[test]
    fn test_buffer_clear() {
        let mut buf = Buffer::with_capacity(100);
        buf.write(b"Test");
        
        assert_eq!(buf.len(), 4);
        buf.clear();
        assert_eq!(buf.len(), 0);
        assert!(buf.capacity() >= 100);
    }
}
