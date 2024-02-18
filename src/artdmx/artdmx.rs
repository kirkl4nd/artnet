use std::net::Ipv4Addr;

/// Represents an Art-Net packet header.
const ART_NET_ID: &[u8; 8] = b"Art-Net\0";
const ART_DMX_OPCODE: u16 = 0x5000;

#[derive(Debug, Clone)]
pub struct ArtDmxPacket {
    /// Art-Net ID, always "Art-Net" followed by a null terminator.
    pub id: [u8; 8],
    /// OpCode, representing the packet type. For ArtDmx, it's 0x5000.
    pub op_code: u16,
    /// Protocol version. High byte first.
    pub version: u16,
    /// Sequence number used to ensure that ArtDmx packets are processed in the correct order.
    pub sequence: u8,
    /// Physical input port from which DMX512 data was input.
    pub physical: u8,
    /// 15 bit universe number.
    pub universe: u16,
    /// The length of the DMX data array. This value should be between 2 and 512.
    pub length: u16,
    /// DMX512 data array. Length is defined by `length`. Max length is 512.
    pub data: Vec<u8>,
}