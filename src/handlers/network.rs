use std::sync::mpsc::{channel, Receiver};

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

    // TODO:    Create udp socket on ip and port
    // TODO:    Parse packets received by opcode, and send them to proper channels.

    return networkhandlerchannels;
}