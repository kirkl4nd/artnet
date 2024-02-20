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