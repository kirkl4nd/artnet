use std::net::Ipv4Addr;

#[derive(Debug, Clone)]
pub struct ArtPollReply {
    pub id: [u8; 8], // Art-Net packet ID, always "Art-Net" followed by null
    pub op_code: u16, // OpCode for ArtPollReply packet
    pub ip_address: Ipv4Addr, // Node IP address
    pub port_number: u16, // Port number, default is 0x1936
    pub version_info: u16, // Version of the node's firmware
    pub net_switch: u8, // Bits 14-8 of the 15-bit Port-Address
    pub sub_switch: u8, // Bits 7-4 of the 15-bit Port-Address
    pub oem_code: u16, // OEM Code
    pub ubea_version: u8, // UBEA version
    pub status1: u8, // Status register
    pub esta_man: u16, // ESTA manufacturer code
    pub short_name: [u8; 18], // Short name of the node
    pub long_name: [u8; 64], // Long name of the node
    pub node_report: [u8; 64], // Node report #0000 to #9999
    pub num_ports: u16, // Number of ports
    pub port_types: [u8; 4], // Port types, bitfield
    pub good_input: [u8; 4], // Input status of the node
    pub good_output: [u8; 4], // Output status of the node
    pub sw_in: [u8; 4], // Input address settings
    pub sw_out: [u8; 4], // Output address settings
    pub sw_video: u8, // Video switch
    pub sw_macro: u8, // Macro switch
    pub sw_remote: u8, // Remote switch
    pub spare: [u8; 3], // Spare bytes for future use
    pub style: u8, // Node style
    pub mac_address: [u8; 6], // MAC address of the node
    pub bind_ip: Ipv4Addr, // Bind IP address
    pub bind_index: u8, // Bind index
    pub status2: u8, // Status register
    pub filler: [u8; 26], // Filler bytes, currently not used
}