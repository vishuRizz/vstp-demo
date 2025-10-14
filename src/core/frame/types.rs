use crate::core::types::{Frame, FrameType};

/// Extension trait for frame type-specific functionality
pub trait FrameTypeExt {
    /// Check if the frame is a control frame
    fn is_control(&self) -> bool;
    
    /// Check if the frame requires acknowledgment
    fn requires_ack(&self) -> bool;
    
    /// Get the frame's priority level (0-255, higher is more important)
    fn priority(&self) -> u8;
}

impl FrameTypeExt for FrameType {
    fn is_control(&self) -> bool {
        matches!(
            self,
            FrameType::Hello
                | FrameType::Welcome
                | FrameType::Ping
                | FrameType::Pong
                | FrameType::Bye
                | FrameType::Ack
                | FrameType::Err
        )
    }

    fn requires_ack(&self) -> bool {
        matches!(
            self,
            FrameType::Hello | FrameType::Bye | FrameType::Data
        )
    }

    fn priority(&self) -> u8 {
        match self {
            FrameType::Err => 255,     // Highest priority
            FrameType::Ack => 200,
            FrameType::Hello => 150,
            FrameType::Welcome => 150,
            FrameType::Bye => 150,
            FrameType::Ping => 100,
            FrameType::Pong => 100,
            FrameType::Data => 50,      // Lowest priority
        }
    }
}

/// Extension trait for frame functionality
pub trait FrameExt {
    /// Get the frame's type-specific priority
    fn priority(&self) -> u8;
    
    /// Check if this is a control frame
    fn is_control(&self) -> bool;
    
    /// Check if this frame requires acknowledgment
    fn requires_ack(&self) -> bool;
}

impl FrameExt for Frame {
    fn priority(&self) -> u8 {
        self.typ.priority()
    }

    fn is_control(&self) -> bool {
        self.typ.is_control()
    }

    fn requires_ack(&self) -> bool {
        self.typ.requires_ack()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_type_control() {
        assert!(FrameType::Hello.is_control());
        assert!(FrameType::Ping.is_control());
        assert!(!FrameType::Data.is_control());
    }

    #[test]
    fn test_frame_type_ack() {
        assert!(FrameType::Hello.requires_ack());
        assert!(FrameType::Data.requires_ack());
        assert!(!FrameType::Ping.requires_ack());
    }

    #[test]
    fn test_frame_type_priority() {
        assert!(FrameType::Err.priority() > FrameType::Data.priority());
        assert!(FrameType::Ack.priority() > FrameType::Ping.priority());
        assert_eq!(FrameType::Hello.priority(), FrameType::Welcome.priority());
    }

    #[test]
    fn test_frame_extensions() {
        let frame = Frame::new(FrameType::Data);
        assert!(!frame.is_control());
        assert!(frame.requires_ack());
        assert_eq!(frame.priority(), 50);

        let frame = Frame::new(FrameType::Err);
        assert!(frame.is_control());
        assert!(!frame.requires_ack());
        assert_eq!(frame.priority(), 255);
    }
}
