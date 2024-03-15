use std::convert::TryFrom;
use std::convert::Into;

/// Id field for all Art-Net packets.
const ART_NET_ID: &[u8; 8] = b"Art-Net\0";
/// OpCode for ArtPoll packets.
const OP_CODE_ART_POLL: u16 = 0x2000;
/// OpCode for ArtPollReply packets.
const OP_CODE_ART_POLL_REPLY: u16 = 0x2100;
/// OpCode for ArtDmx packets.
const OP_CODE_ART_DMX: u16 = 0x5000;
/// OpCode for ArtNzs packets.
const OP_CODE_ART_NZS: u16 = 0x5100;
/// OpCode for ArtSync packets.
const OP_CODE_ART_SYNC: u16 = 0x5200;
/// OpCode for ArtAddress packets.
const OP_CODE_ART_ADDRESS: u16 = 0x6000;
/// OpCode for ArtInput packets.
const OP_CODE_ART_INPUT: u16 = 0x7000;
/// OpCode for ArtTodRequest packets.
const OP_CODE_ART_TOD_REQUEST: u16 = 0x8000;
/// OpCode for ArtTodData packets.
const OP_CODE_ART_TOD_DATA: u16 = 0x8100;
/// OpCode for ArtTodControl packets.
const OP_CODE_ART_TOD_CONTROL: u16 = 0x8200;
/// OpCode for ArtRdm packets.
const OP_CODE_ART_RDM: u16 = 0x8300;
/// OpCode for ArtRdmSub packets.
const OP_CODE_ART_RDM_SUB: u16 = 0x8400;
/// OpCode for ArtVideoSetup packets.
const OP_CODE_ART_VIDEO_SETUP: u16 = 0xa010;
/// OpCode for ArtVideoPalette packets.
const OP_CODE_ART_VIDEO_PALETTE: u16 = 0xa020;
/// OpCode for ArtVideoData packets.
const OP_CODE_ART_VIDEO_DATA: u16 = 0xa040;
/// OpCode for ArtTimeCode packets.
const OP_CODE_ART_TIME_CODE: u16 = 0x9700;
/// OpCode for ArtTimeSync packets.
const OP_CODE_ART_TIME_SYNC: u16 = 0x9800;
/// OpCode for ArtTrigger packets.
const OP_CODE_ART_TRIGGER: u16 = 0x9900;
/// OpCode for ArtDirectory packets.
const OP_CODE_ART_DIRECTORY: u16 = 0x9a00;
/// OpCode for ArtDirectoryReply packets.
const OP_CODE_ART_DIRECTORY_REPLY: u16 = 0x9b00;
/// OpCode for ArtIpProg packets.
const OP_CODE_ART_IP_PROG: u16 = 0xf800;
/// OpCode for ArtIpProgReply packets.
const OP_CODE_ART_IP_PROG_REPLY: u16 = 0xf900;
/// OpCode for ArtMedia packets.
const OP_CODE_ART_MEDIA: u16 = 0x9000;
/// OpCode for ArtMediaPatch packets.
const OP_CODE_ART_MEDIA_PATCH: u16 = 0x9100;
/// OpCode for ArtMediaControl packets.
const OP_CODE_ART_MEDIA_CONTROL: u16 = 0x9200;
/// OpCode for ArtMediaControlReply packets.
const OP_CODE_ART_MEDIA_CONTROL_REPLY: u16 = 0x9300;
/// OpCode for ArtFirmwareMaster packets.
const OP_CODE_ART_FIRMWARE_MASTER: u16 = 0xf200;
/// OpCode for ArtFirmwareReply packets.
const OP_CODE_ART_FIRMWARE_REPLY: u16 = 0xf300;

pub trait Packet: TryFrom<Vec<u8>, Error = std::io::Error> + Into<Vec<u8>> {
    // add any additional requirements for the Packet trait.
}
