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


impl ArtDmx {
    /// Converts the `ArtDmx` struct into a raw packet for UDP transmission.
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut packet = Vec::new();

        // Start with the Art-Net ID
        packet.extend_from_slice(&self.id);

        // OpCode in little-endian format
        packet.extend_from_slice(&self.op_code.to_le_bytes());

        // Protocol version, high byte first
        packet.extend_from_slice(&self.version.to_be_bytes());

        // Sequence, Physical
        packet.push(self.sequence);
        packet.push(self.physical);

        // Sub-Net (lower 4 bits of the universe) and Universe (upper 7 bits of the universe)
        // Adjust these lines if the document specifies a different structure for the universe
        let sub_uni = (self.universe & 0xFF) as u8; // Lower 8 bits for Sub-Net and Universe
        let net = (self.universe >> 8) as u8; // Upper 8 bits for Net
        packet.push(sub_uni);
        packet.push(net);

        // Length of DMX data, high byte first
        packet.extend_from_slice(&self.length.to_be_bytes());

        // DMX data
        packet.extend(&self.data);

        packet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artdmx_encoding() {
        let art_dmx = ArtDmx {
            id: *ART_NET_ID,
            op_code: ART_DMX_OPCODE,
            version: 0x0001,
            sequence: 0x01,
            physical: 0x02,
            universe: 0x0102,
            length: 3,
            data: vec![0xAA, 0xBB, 0xCC],
        };

        let bytes = art_dmx.into_bytes();
        assert_eq!(bytes[0..8], *ART_NET_ID);
        assert_eq!(u16::from_le_bytes([bytes[8], bytes[9]]), ART_DMX_OPCODE);
        assert_eq!(u16::from_be_bytes([bytes[10], bytes[11]]), 0x0001);
        assert_eq!(bytes[12], 0x01);
        assert_eq!(bytes[13], 0x02);
        assert_eq!(bytes[14], 0x02); // Sub-Uni
        assert_eq!(bytes[15], 0x01); // Net
        assert_eq!(u16::from_be_bytes([bytes[16], bytes[17]]), 3);
        assert_eq!(bytes[18..], [0xAA, 0xBB, 0xCC]);
    }

    #[test]
    fn test_artdmx_decoding() {
        let packet = vec![
            b'A', b'r', b't', b'-', b'N', b'e', b't', b'\0', // Art-Net ID
            0x00, 0x50, // OpCode (0x5000 in little endian)
            0x00, 0x01, // Protocol Version
            0x01, // Sequence
            0x02, // Physical
            0x02, // Sub-Uni
            0x01, // Net
            0x00, 0x03, // Length (3)
            0xAA, 0xBB, 0xCC, // DMX Data
        ];

        let art_dmx = ArtDmx::try_from(packet).expect("Decoding failed");
        assert_eq!(art_dmx.id, *ART_NET_ID);
        assert_eq!(art_dmx.op_code, ART_DMX_OPCODE);
        assert_eq!(art_dmx.version, 0x0001);
        assert_eq!(art_dmx.sequence, 0x01);
        assert_eq!(art_dmx.physical, 0x02);
        assert_eq!(art_dmx.universe, 0x0102);
        assert_eq!(art_dmx.length, 3);
        assert_eq!(art_dmx.data, vec![0xAA, 0xBB, 0xCC]);
    }
}
