// Decodes incoming packets for minecraft server into rust-friendly objects.
// Authored by: Three rats in a trench coat.

#![allow(unused_variables, dead_code)]

use crate::util::datatypes::{
    VarInt,
    StringMC,
    Uuid,
};

use crate::Packet;

pub fn make_packet(data: Vec<u8>) -> Packet {
    let data = data;
    let (packet_length, data) = VarInt::decode(data);
    let (packet_id, data) = match packet_length {
        0x00 => {
            panic!("Packet Length is 0x00");
        }
        _ => {
            VarInt::decode(data)
        }
    };
    Packet::new(packet_length as i32, packet_id as i32, data)
}

pub fn parse_handshake(length: i32, id: i32, data: Vec<u8>) -> u32 {
    let protocol_version: i32;
    let server_address: String;
    let server_port: u16;
    let next_state: i32;

    let expected_length = length as u32;
    let (protocol_version, data) = VarInt::decode(data);
    println!("Protocol Version: {}", protocol_version);

    let (server_address, mut data) = StringMC::decode(data);
    println!("Server Address: {}", server_address);
    // remove the first two bytes of the data vec 

    let snip: Vec<u8> = data
        .drain(0..2)
        .collect();

    let server_port: u16 = 25565;
    println!("Server Port: {}", server_port);

    let (next_state, data) = VarInt::decode(data);
    match next_state {
        1 => println!("Next State: Status"),
        2 => println!("Next State: Login"),
        _ => println!("Next State: Handshake"),
    }

    next_state as u32
}
pub fn parse_login_start(data: Vec<u8>) -> (String, String) {
    println!("    Parsing login start");
    let (name, data) = StringMC::decode(data);
    let (uuid, data) = Uuid::decode(data);
    (name, uuid.to_string())
}

