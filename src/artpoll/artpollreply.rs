use std::convert::TryFrom;
use std::net::Ipv4Addr;

const ART_NET_ID: &[u8; 8] = b"Art-Net\0";
const ART_POLL_REPLY_OPCODE: u16 = 0x2100; // OpCode for ArtPollReply

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
        if bytes.len() < 239 {
            return Err("Packet too short");
        }

        if &bytes[0..8] != ART_NET_ID {
            return Err("Invalid Art-Net ID");
        }

        let op_code = u16::from_be_bytes([bytes[8], bytes[9]]);
        if op_code != ART_POLL_REPLY_OPCODE {
            return Err("Invalid OpCode");
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

        let short_name = {
            let mut sn = [0u8; 18];
            sn.copy_from_slice(&bytes[26..44]);
            sn
        };

        let long_name = {
            let mut ln = [0u8; 64];
            ln.copy_from_slice(&bytes[44..108]);
            ln
        };

        let node_report = {
            let mut nr = [0u8; 64];
            nr.copy_from_slice(&bytes[108..172]);
            nr
        };

        let num_ports = u16::from_be_bytes([bytes[172], bytes[173]]);
        let port_types = {
            let mut pt = [0u8; 4];
            pt.copy_from_slice(&bytes[174..178]);
            pt
        };

        let good_input = {
            let mut gi = [0u8; 4];
            gi.copy_from_slice(&bytes[178..182]);
            gi
        };

        let good_output = {
            let mut go = [0u8; 4];
            go.copy_from_slice(&bytes[182..186]);
            go
        };

        let sw_in = {
            let mut swi = [0u8; 4];
            swi.copy_from_slice(&bytes[186..190]);
            swi
        };

        let sw_out = {
            let mut swo = [0u8; 4];
            swo.copy_from_slice(&bytes[190..194]);
            swo
        };

        let sw_video = bytes[194];
        let sw_macro = bytes[195];
        let sw_remote = bytes[196];
        let style = bytes[200];
        let mac = {
            let mut m = [0u8; 6];
            m.copy_from_slice(&bytes[201..207]);
            m
        };

        let bind_ip = Ipv4Addr::new(bytes[207], bytes[208], bytes[209], bytes[210]);
        let bind_index = bytes[211];
        let status2 = bytes[212];
        let filler = {
            let mut f = [0u8; 26];
            f.copy_from_slice(&bytes[213..239]);
            f
        };

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

        packet.extend_from_slice(&self.id);
        packet.extend_from_slice(&self.op_code.to_le_bytes()); // OpCode is little-endian
        packet.extend_from_slice(&self.ip_address.octets());
        packet.extend_from_slice(&self.port_number.to_be_bytes()); // Port number is big-endian
        packet.push(self.vers_info_hi);
        packet.push(self.vers_info_lo);
        packet.push(self.net_switch);
        packet.push(self.sub_switch);
        packet.push(self.oem_hi);
        packet.push(self.oem_lo);
        packet.push(self.ubea_version);
        packet.push(self.status1);
        packet.extend_from_slice(&self.esta_man.to_be_bytes()); // ESTA manufacturer is big-endian

        // Short Name, Long Name, and Node Report should be exactly the sizes specified, padded with zeros if needed
        packet.extend_from_slice(&self.short_name);
        packet.extend_from_slice(&self.long_name);
        packet.extend_from_slice(&self.node_report);

        packet.extend_from_slice(&self.num_ports.to_be_bytes()); // Number of ports is big-endian
        packet.extend_from_slice(&self.port_types);
        packet.extend_from_slice(&self.good_input);
        packet.extend_from_slice(&self.good_output);
        packet.extend_from_slice(&self.sw_in);
        packet.extend_from_slice(&self.sw_out);

        packet.push(self.sw_video);
        packet.push(self.sw_macro);
        packet.push(self.sw_remote);

        // Spare bytes should be zero
        packet.extend_from_slice(&[0; 3]);

        packet.push(self.style);
        packet.extend_from_slice(&self.mac);
        packet.extend_from_slice(&self.bind_ip.octets());
        packet.push(self.bind_index);
        packet.push(self.status2);

        // Filler bytes should be zero
        packet.extend_from_slice(&self.filler);

        packet
    }
}
