use std::sync::mpsc::Receiver;

use crate::{dmx512::dmx512::Dmx512, handlers::{artnet_to_dmx::create_artdmx_to_dmx_handler, artpoll::create_artpoll_responder, network::{self, create_network_handler}}};

pub fn create_artnet_to_dmx_node(ip_and_port: &str) -> Receiver<Dmx512> {

    // create a network thread
    // binds to ip and port
    // sends received packets to proper channels
    let networkhandlerchannels = create_network_handler(ip_and_port);

    // create a thread to handle replying to artpolls
    // receives artpoll packets from the network handler
    create_artpoll_responder(networkhandlerchannels.artpoll_output);

    // create a thread to translate received artdmx packets to dmx512
    // receives artdmx packets from the network handler
    let dmx_output = create_artdmx_to_dmx_handler(networkhandlerchannels.artdmx_output);

    return dmx_output;
}