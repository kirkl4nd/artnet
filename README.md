# artnet - a Rust implementation
### WORK IN PROGRESS

This crate is designed to make it easy for developers to use Art-Net in their projects.

A node is created to either send or receive Art-Net data. Nodes additionally handle the ArtPoll discovery process, both by replying to ArtPolls, and by sending ArtPolls and accepting ArtPollReply packets.

The translation process from udp packet to DMX512 packet is handled entirely asynchronously. Channels are used for communication between threads.

## Done

- Structs to represent ArtPoll, ArtPollReply, ArtDmx packets
- Conversions from raw UDP packets to structs
- Networking and packet dispatch


## In progress

- Sending and receiving ArtPolls, managing hosts.
- Generating ArtDMX packets from a Dmx512 data packet

## Things to improve

- Naming and organization

## Things to do later

- Make a packet trait for all structs representing data packets. This will standardize and simplify converting to and from raw packets.

# Intended usage

This project is a framework, in which a Node struct represents all types of ArtNet nodes and their common funcitonality.

The behavior of the node is determined by which handlers are started.

To create an Art-Net to DMX node:
(Incomplete syntax... for now)
```
let artnet_to_dmx = Node::new([ip]:[port]);
artnet_to_dmx.enable_discovery([ip] or [0.0.0.0] for all);
let dmx_receiver: Receiver<Dmx512Packet> = artnet_to_dmx.get_dmx_stream();
```
To create a DMX to Art-Net (Artnet controller) node:
```
let dmx_to_artnet = Node::new([ip]:[port], dmx_input: Receiver<Dmx512Packet>);
dmx_to_artnet.start_artpoll();