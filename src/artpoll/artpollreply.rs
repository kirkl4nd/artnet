use std::convert::TryFrom;
use std::net::Ipv4Addr;

const ART_NET_ID: &[u8; 8] = b"Art-Net\0";
const ART_POLL_REPLY_OPCODE: u16 = 0x2100; // Replace with the actual OpCode for ArtPollReply

#[derive(Debug, Clone)]
pub struct ArtPollReply {
    pub id: [u8; 8],
    pub op_code: u16,
    pub ip_address: Ipv4Addr,
    pub port_number: u16,
    pub vers_info_hi: u8,
    pub vers_info_lo: u8,
    pub net_switch: u8,
    pub sub_switch: u8,
    pub oem_hi: u8,
    pub oem_lo: u8,
    pub ubea_version: u8,
    pub status1: u8,
    pub esta_man: u16,
    pub short_name: [u8; 18],
    pub long_name: [u8; 64],
    pub node_report: [u8; 64],
    pub num_ports: u16,
    pub port_types: [u8; 4],
    pub good_input: [u8; 4],
    pub good_output: [u8; 4],
    pub sw_in: [u8; 4],
    pub sw_out: [u8; 4],
    pub sw_video: u8,
    pub sw_macro: u8,
    pub sw_remote: u8,
    // Not including spare1, spare2, spare3 for brevity
    pub style: u8,
    pub mac: [u8; 6],
    pub bind_ip: Ipv4Addr,
    pub bind_index: u8,
    pub status2: u8,
    pub filler: [u8; 26],
}

impl TryFrom<Vec<u8>> for ArtPollReply {
    type Error = &'static str;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() < 239 || &bytes[0..8] != ART_NET_ID {
            return Err("Invalid Art-Net packet");
        }

        let op_code = u16::from_be_bytes([bytes[8], bytes[9]]);
        if op_code != ART_POLL_REPLY_OPCODE {
            return Err("Invalid OpCode for ArtPollReply packet");
        }

        let ip_address = Ipv4Addr::new(bytes[10], bytes[11], bytes[12], bytes[13]);
        let port_number = u16::from_be_bytes([bytes[14], bytes[15]]);
        let vers_info_hi = bytes[16];
        let vers_info_lo = bytes[17];
        let net_switch = bytes[18];
        let sub_switch = bytes[19];
        let oem_hi = bytes[20];
        let oem_lo = bytes[21];
        let ubea_version = bytes[22];
        let status1 = bytes[23];
        let esta_man = u16::from_be_bytes([bytes[24], bytes[25]]);
        let short_name = bytes[26..44]
            .try_into()
            .map_err(|_| "Failed to parse short_name")?;
        let long_name = bytes[44..108]
            .try_into()
            .map_err(|_| "Failed to parse long_name")?;
        let node_report = bytes[108..172]
            .try_into()
            .map_err(|_| "Failed to parse node_report")?;
        let num_ports = u16::from_be_bytes([bytes[172], bytes[173]]);
        let port_types = bytes[174..178]
            .try_into()
            .map_err(|_| "Failed to parse port_types")?;
        let good_input = bytes[178..182]
            .try_into()
            .map_err(|_| "Failed to parse good_input")?;
        let good_output = bytes[182..186]
            .try_into()
            .map_err(|_| "Failed to parse good_output")?;
        let sw_in = bytes[186..190]
            .try_into()
            .map_err(|_| "Failed to parse sw_in")?;
        let sw_out = bytes[190..194]
            .try_into()
            .map_err(|_| "Failed to parse sw_out")?;
        let sw_video = bytes[194];
        let sw_macro = bytes[195];
        let sw_remote = bytes[196];
        let style = bytes[200];
        let mac = bytes[201..207]
            .try_into()
            .map_err(|_| "Failed to parse mac")?;
        let bind_ip = Ipv4Addr::new(bytes[207], bytes[208], bytes[209], bytes[210]);
        let bind_index = bytes[211];
        let status2 = bytes[212];
        let filler = bytes[213..239]
            .try_into()
            .map_err(|_| "Failed to parse filler")?;

        Ok(ArtPollReply {
            id: *ART_NET_ID,
            op_code,
            ip_address,
            port_number,
            vers_info_hi,
            vers_info_lo,
            net_switch,
            sub_switch,
            oem_hi,
            oem_lo,
            ubea_version,
            status1,
            esta_man,
            short_name,
            long_name,
            node_report,
            num_ports,
            port_types,
            good_input,
            good_output,
            sw_in,
            sw_out,
            sw_video,
            sw_macro,
            sw_remote,
            // We'll leave the spare fields out for brevity
            style,
            mac,
            bind_ip,
            bind_index,
            status2,
            filler,
        })
    }
}

impl ArtPollReply {
    /// Converts the `ArtPollReply` struct into a raw packet for UDP transmission.
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut packet = Vec::with_capacity(239); // The spec indicates a fixed size for ArtPollReply packets

        // Start with the Art-Net ID
        packet.extend_from_slice(&self.id);

        // OpCode in little-endian format
        packet.extend_from_slice(&self.op_code.to_le_bytes());

        // IP Address
        packet.extend_from_slice(&self.ip_address.octets());

        // Port Number in big-endian format
        packet.extend_from_slice(&self.port_number.to_be_bytes());

        // Version Info high and low bytes
        packet.push(self.vers_info_hi);
        packet.push(self.vers_info_lo);

        // NetSwitch and SubSwitch
        packet.push(self.net_switch);
        packet.push(self.sub_switch);

        // Oem high and low bytes
        packet.push(self.oem_hi);
        packet.push(self.oem_lo);

        // Ubea Version
        packet.push(self.ubea_version);

        // Status1
        packet.push(self.status1);

        // ESTA Manufacturer in big-endian format
        packet.extend_from_slice(&self.esta_man.to_be_bytes());

        // Short Name
        packet.extend_from_slice(&self.short_name);

        // Long Name
        packet.extend_from_slice(&self.long_name);

        // Node Report
        packet.extend_from_slice(&self.node_report);

        // Number of Ports in big-endian format
        packet.extend_from_slice(&self.num_ports.to_be_bytes());

        // Port Types
        packet.extend_from_slice(&self.port_types);

        // Good Input
        packet.extend_from_slice(&self.good_input);

        // Good Output
        packet.extend_from_slice(&self.good_output);

        // SwIn
        packet.extend_from_slice(&self.sw_in);

        // SwOut
        packet.extend_from_slice(&self.sw_out);

        // SwVideo, SwMacro, SwRemote
        packet.push(self.sw_video);
        packet.push(self.sw_macro);
        packet.push(self.sw_remote);

        // Fill in Spare1, Spare2, and Spare3 as 0
        packet.extend_from_slice(&[0; 3]);

        // Style
        packet.push(self.style);

        // MAC Address
        packet.extend_from_slice(&self.mac);

        // Bind IP
        packet.extend_from_slice(&self.bind_ip.octets());

        // BindIndex
        packet.push(self.bind_index);

        // Status2
        packet.push(self.status2);

        // Filler
        packet.extend_from_slice(&self.filler);

        packet
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::str::from_utf8;

    #[test]
    fn test_artpollreply_encoding() {
        let art_poll_reply = ArtPollReply {
            id: *ART_NET_ID,
            op_code: ART_POLL_REPLY_OPCODE,
            ip_address: Ipv4Addr::new(192, 168, 1, 50),
            port_number: 6454,
            vers_info_hi: 0,
            vers_info_lo: 14,
            net_switch: 0,
            sub_switch: 0,
            oem_hi: 0xFF,
            oem_lo: 0xFF,
            ubea_version: 0,
            status1: 0,
            esta_man: 0x20AC,
            short_name: *b"Test Node\0\0\0\0\0\0\0\0\0\0",
            long_name: *b"Example Long Name for a Test Node\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            node_report: *b"Everything is running smoothly\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            num_ports: 2,
            port_types: [0x80, 0x80, 0, 0],
            good_input: [0, 0, 0, 0],
            good_output: [0, 0, 0, 0],
            sw_in: [0, 0, 0, 0],
            sw_out: [0, 0, 0, 0],
            sw_video: 0,
            sw_macro: 0,
            sw_remote: 0,
            style: 0,
            mac: [0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x01],
            bind_ip: Ipv4Addr::new(192, 168, 1, 51),
            bind_index: 1,
            status2: 0,
            filler: [0; 26],
        };

        let bytes = art_poll_reply.into_bytes();

        // Basic structure assertions
        assert_eq!(&bytes[0..8], ART_NET_ID);
        assert_eq!(u16::from_be_bytes([bytes[8], bytes[9]]), ART_POLL_REPLY_OPCODE);
        assert_eq!(Ipv4Addr::new(bytes[10], bytes[11], bytes[12], bytes[13]), art_poll_reply.ip_address);
        assert_eq!(u16::from_be_bytes([bytes[14], bytes[15]]), art_poll_reply.port_number);
        assert_eq!(bytes[16], art_poll_reply.vers_info_hi);
        assert_eq!(bytes[17], art_poll_reply.vers_info_lo);
        // Continue with further assertions for all fields to ensure correct encoding
    }

    #[test]
    fn test_artpollreply_decoding() {
        let packet = vec![
            b'A', b'r', b't', b'-', b'N', b'e', b't', b'\0', // Art-Net ID
            0x21, 0x00, // OpCode (0x2100 in big-endian for ArtPollReply)
            192, 168, 1, 50, // IP Address
            0x19, 0x26, // Port Number (6454 in big-endian)
            0, 14, // Version Info
            0, 0, // NetSwitch, SubSwitch
            0xFF, 0xFF, // OEM Hi/Lo
            0, // UBEA Version
            0, // Status1
            0x20, 0xAC, // ESTA Manufacturer
            *b"Test Node\0\0\0\0\0\0\0\0\0\0", // Short Name
            *b"Example Long Name for a Test Node\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", // Long Name
            *b"Everything is running smoothly\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", // Node Report
            0x00, 0x02, // Num Ports
            0x80, 0x80, 0x00, 0x00, // Port Types
            0x00, 0x00, 0x00, 0x00, // Good Input
            0x00, 0x00, 0x00, 0x00, // Good Output
            0x00, 0x00, 0x00, 0x00, // SwIn
            0x00, 0x00, 0x00, 0x00, // SwOut
            0x00, // SwVideo
            0x00, // SwMacro
            0x00, // SwRemote
            0, // Style
            0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x01, // MAC Address
            192, 168, 1, 51, // Bind IP
            1, // BindIndex
            0, // Status2
            [0; 26].to_vec(), // Filler
        ]
        .into_iter()
        .flatten()
        .collect();

        let art_poll_reply = ArtPollReply::try_from(packet).expect("Decoding failed");

        // Verify each field
        assert_eq!(art_poll_reply.id, *ART_NET_ID);
        assert_eq!(art_poll_reply.op_code, ART_POLL_REPLY_OPCODE);
        assert_eq!(art_poll_reply.ip_address, Ipv4Addr::new(192, 168, 1, 50));
        assert_eq!(art_poll_reply.port_number, 6454);
        // Continue with further assertions for all fields to ensure correct decoding
    }
}
