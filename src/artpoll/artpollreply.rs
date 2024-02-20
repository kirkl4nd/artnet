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
