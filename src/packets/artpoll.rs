const ART_NET_ID: &[u8; 8] = b"Art-Net\0";
const ART_POLL_OPCODE: u16 = 0x2000;

#[derive(Debug, Clone)]
pub struct ArtPoll {
    pub id: [u8; 8], // Art-Net packet ID, always "Art-Net" followed by null
    pub op_code: u16, // OpCode for ArtPoll packet
    pub protocol_version: u16, // High byte first
    pub talk_to_me: u8, // Behavior flags
    pub priority: u8, // Diagnostic message priority
}