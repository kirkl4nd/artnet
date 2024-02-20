use std::{sync::{mpsc::Receiver, Arc, Mutex}, thread};

/// takes in raw artpoll packets and sends artreplies via network stack.
/// hardcoded node limitations:
///  - replies to all artpolls
///  - sets up a passive node
pub fn create_artpoll_responder(artpoll_input: Receiver<Vec<u8>>) {
    thread::spawn(move || {
        for artpollpacket in artpoll_input {
            // TODO: convert the input packet to ArtPoll struct.
            // TODO: build ArtPollReply struct
            // TODO: serialize the ArtPollReply and send out the network stack
            //       to the ip of the artpoll sender.
        }
    });
}