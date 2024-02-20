use crate::artdmx::artdmx::ArtDmx;

#[derive(Debug, Clone)]
pub struct Dmx512 {
    /// The DMX data for up to 512 channels.
    /// Each byte represents the intensity level (0-255) of a single channel.
    pub data: [u8; 512],
}

impl From<ArtDmx> for Dmx512 {
    fn from(art_dmx_packet: ArtDmx) -> Self {
        let mut data = [0u8; 512];
        let length = art_dmx_packet.length as usize;

        // Ensure we do not exceed the bounds of the DMX data array
        let copy_length = length.min(data.len());

        // Copy the DMX data from the ArtDmxPacket to the Dmx512Packet
        data[..copy_length].copy_from_slice(&art_dmx_packet.data[..copy_length]);

        Dmx512 { data }
    }
}