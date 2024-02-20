use std::{
    net::UdpSocket,
    sync::mpsc::{channel, Receiver},
    thread,
};

const ART_POLL: u16 = 0x2000;
const ART_POLL_REPLY: u16 = 0x2100;
const ART_DMX: u16 = 0x5000;

pub struct NetworkHandlerChannels {
    pub artpoll_output: Receiver<Vec<u8>>,
    pub artpoll_reply_output: Receiver<Vec<u8>>,
    pub artdmx_output: Receiver<Vec<u8>>,
}

pub fn create_network_handler(ip_and_port: &str) -> NetworkHandlerChannels {
    let (artpoll_src, artpoll_dest) = channel::<Vec<u8>>();
    let (artpoll_reply_src, artpoll_reply_dest) = channel::<Vec<u8>>();
    let (artdmx_src, artdmx_dest) = channel::<Vec<u8>>();

    let networkhandlerchannels: NetworkHandlerChannels = NetworkHandlerChannels {
        artpoll_output: artpoll_dest,
        artpoll_reply_output: artpoll_reply_dest,
        artdmx_output: artdmx_dest,
    };

    let socket = UdpSocket::bind(ip_and_port).expect("Failed to bind UDP socket");
    socket
        .set_nonblocking(true)
        .expect("Failed to set socket to non-blocking");

    // Pre-allocate a buffer outside of the loop.
    let mut buffer = vec![0u8; 1024]; // Adjust this buffer size as needed.

    thread::spawn(move || {
        loop {
            match socket.recv(&mut buffer) {
                Ok(num_bytes) => {
                    // Process packet
                    if num_bytes >= 10 {
                        // Ensure packet has enough bytes for an opcode
                        let opcode = u16::from(buffer[8]) | (u16::from(buffer[9]) << 8);

                        match opcode {
                            ART_POLL => artpoll_src
                                .send(buffer[..num_bytes].to_vec())
                                .unwrap_or_else(|e| eprintln!("Error: {}", e)),
                            ART_POLL_REPLY => artpoll_reply_src
                                .send(buffer[..num_bytes].to_vec())
                                .unwrap_or_else(|e| eprintln!("Error: {}", e)),
                            ART_DMX => artdmx_src
                                .send(buffer[..num_bytes].to_vec())
                                .unwrap_or_else(|e| eprintln!("Error: {}", e)),
                            _ => eprintln!("Unknown opcode: {:04x}", opcode),
                        }
                    }
                }
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::WouldBlock {
                        // Handle error, but ignore 'WouldBlock' since we're non-blocking
                        eprintln!("Error receiving packet: {}", e);
                    }
                }
            }
        }
    });

    networkhandlerchannels
}
