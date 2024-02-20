use std::net::Ipv4Addr;

/// Represents an Art-Net packet header.
const ART_NET_ID: &[u8; 8] = b"Art-Net\0";
const ART_DMX_OPCODE: u16 = 0x5000;

#[derive(Debug, Clone)]
pub struct ArtDmx {
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

impl TryFrom<Vec<u8>> for ArtDmx {
    type Error = &'static str;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() < 18 || &bytes[0..8] != ART_NET_ID {
            return Err("Invalid Art-Net packet");
        }

        let op_code = u16::from_le_bytes([bytes[8], bytes[9]]);
        if op_code != ART_DMX_OPCODE {
            return Err("Invalid OpCode for ArtDmx packet");
        }

        let version = u16::from_be_bytes([bytes[10], bytes[11]]);
        let sequence = bytes[12];
        let physical = bytes[13];
        let sub_uni = bytes[14];
        let net = bytes[15];
        let length = u16::from_be_bytes([bytes[16], bytes[17]]);
        
        if bytes.len() < (18 + length as usize) {
            return Err("ArtDmx data length mismatch");
        }

        let data = bytes[18..(18 + length as usize)].to_vec();

        Ok(ArtDmx {
            id: *ART_NET_ID,
            op_code,
            version,
            sequence,
            physical,
            universe: ((net as u16) << 8) | (sub_uni as u16), // Combine net and sub_uni to form the universe
            length,
            data,
        })
    }
}