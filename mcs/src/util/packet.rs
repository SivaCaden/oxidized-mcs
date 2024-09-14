// A class representing one generic packet for a Minecraft server according to 
// https://wiki.vg/Protocol#Packet_format.
// Also contains methods to decode/encode packets
//
// Authored by: Three Rats in a Trench Coat

pub trait Packet {
    fn encode(data: Vec<u8>) -> Self;

    fn decode(&mut self) -> Vec<u8>;

    fn get_length(&mut self) -> u8;

    fn get_id(&mut self) -> u8;
}

pub struct Handshake {
    length: u8,
    id: u8,
    data: Vec<u8>
}

impl Packet for Handshake {
    fn encode(data: Vec<u8>) -> Handshake {
        Handshake {
            length: 0,
            id: 0,
            data
        }
    }
    
    fn decode(&mut self) -> Vec<u8> {
        self.data.clone()
    }
    
    fn get_length(&mut self) -> u8 { self.length }

    fn get_id(&mut self) -> u8 { self.id }
}
