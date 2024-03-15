pub struct Dmx512 {
    /// The DMX data for up to 512 channels.
    /// Each byte represents the intensity level (0-255) of a single channel.
    pub data: [u8; 512],
}