use std::convert::TryFrom;

const ART_NET_ID: &[u8; 8] = b"Art-Net\0";
const ART_POLL_OPCODE: u16 = 0x2000; // Replace with the actual OpCode for ArtPoll

#[derive(Debug, Clone)]
pub struct ArtPoll {
    pub id: [u8; 8], // Art-Net packet ID, always "Art-Net" followed by null
    pub op_code: u16, // OpCode for ArtPoll packet
    pub protocol_version: u16, // High byte first
    pub talk_to_me: u8, // Behavior flags
    pub priority: u8, // Diagnostic message priority
}

impl TryFrom<Vec<u8>> for ArtPoll {
    type Error = &'static str;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() < 10 || &bytes[0..8] != ART_NET_ID {
            return Err("Invalid Art-Net ID");
        }

        let op_code = u16::from_le_bytes([bytes[8], bytes[9]]);
        if op_code != ART_POLL_OPCODE {
            return Err("Invalid OpCode for ArtPoll");
        }

        if bytes.len() < 12 {
            return Err("ArtPoll packet is too short");
        }

        let protocol_version = u16::from_be_bytes([bytes[10], bytes[11]]);
        let talk_to_me = bytes[12];
        let priority = bytes[13];

        Ok(ArtPoll {
            id: *ART_NET_ID,
            op_code,
            protocol_version,
            talk_to_me,
            priority,
        })
    }
}

impl ArtPoll {
    /// Converts the `ArtPoll` struct into a raw packet for UDP transmission.
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut packet = Vec::with_capacity(14); // Fixed size for ArtPoll packets

        // Start with the Art-Net ID
        packet.extend_from_slice(&self.id);

        // OpCode in little-endian format
        packet.extend_from_slice(&self.op_code.to_le_bytes());

        // Protocol version, high byte first
        packet.extend_from_slice(&self.protocol_version.to_be_bytes());

        // TalkToMe and Priority
        packet.push(self.talk_to_me);
        packet.push(self.priority);

        packet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artpoll_encoding() {
        let art_poll = ArtPoll {
            id: *ART_NET_ID,
            op_code: ART_POLL_OPCODE,
            protocol_version: 0x0001,
            talk_to_me: 0b00000010,
            priority: 0x10,
        };

        let bytes = art_poll.into_bytes();
        assert_eq!(bytes[0..8], *ART_NET_ID);
        assert_eq!(u16::from_le_bytes([bytes[8], bytes[9]]), ART_POLL_OPCODE);
        assert_eq!(u16::from_be_bytes([bytes[10], bytes[11]]), 0x0001);
        assert_eq!(bytes[12], 0b00000010);
        assert_eq!(bytes[13], 0x10);
    }

    #[test]
    fn test_artpoll_decoding() {
        let packet = vec![
            b'A', b'r', b't', b'-', b'N', b'e', b't', b'\0', // Art-Net ID
            0x00, 0x20, // OpCode (0x2000 in little endian for ArtPoll)
            0x00, 0x01, // Protocol Version
            0b00000010, // TalkToMe
            0x10, // Priority
        ];

        let art_poll = ArtPoll::try_from(packet).expect("Decoding failed");
        assert_eq!(art_poll.id, *ART_NET_ID);
        assert_eq!(art_poll.op_code, ART_POLL_OPCODE);
        assert_eq!(art_poll.protocol_version, 0x0001);
        assert_eq!(art_poll.talk_to_me, 0b00000010);
        assert_eq!(art_poll.priority, 0x10);
    }
}
