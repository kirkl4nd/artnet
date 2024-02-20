use std::{sync::mpsc::{self, Receiver}, thread};

use crate::{artdmx::artdmx::ArtDmx, dmx512::dmx512::Dmx512, };

pub fn create_artdmx_to_dmx_handler(artdmxpacket_input: Receiver<Vec<u8>>) -> Receiver<Dmx512> {
    let (source, dest) = mpsc::channel::<Dmx512>();
    thread::spawn(move || {
        for artdmxpacket in artdmxpacket_input {
            // Specify the type that you are trying to convert to using the turbofish syntax
            match TryInto::<ArtDmx>::try_into(artdmxpacket) {
                Ok(artdmx) => {
                    // Convert ArtDmx into Dmx512 here. Assuming `into()` is implemented for ArtDmx -> Dmx512 conversion
                    let dmx512packet: Dmx512 = artdmx.into();
                    // Handle the result of send, assuming it can fail
                    if let Err(send_error) = source.send(dmx512packet) {
                        eprintln!("Failed to send Dmx512 packet: {}", send_error);
                    }
                },
                Err(e) => {
                    eprintln!("Failed to convert to ArtDmx: {}", e);
                }
            }
        }
        
    });

    return dest;
}