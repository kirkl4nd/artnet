use std::net::Ipv4Addr;

const ART_POLL_REPLY_OPCODE: u16 = 0x2100;

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