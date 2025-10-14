use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Flags: u8 {
        const REQ_ACK = 0b0000_0001;  // Request acknowledgment
        const CRC     = 0b0000_0010;  // CRC checksum present
        const FRAG    = 0b0001_0000;  // Fragmented frame
        const COMP    = 0b0010_0000;  // Compressed payload
    }
}
