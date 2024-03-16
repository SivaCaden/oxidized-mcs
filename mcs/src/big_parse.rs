use crate::mc_datatypes::{
    VarInt,
    StringMC,
    UShort,
};

pub fn parse<T>(data: &[u8]){
    let (packet_length, packet_id, data) = parse_length_packid(data.to_vec()); 
    match packet_id {
        _ => {
            println!("Packet ID: {}", packet_id);
            parse_handshake(packet_length, packet_id, data.to_vec());
        }
    }
}

pub fn parse_length_packid(data: Vec<u8>) -> (i32, i32, Vec<u8>) {
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
    (packet_length, packet_id, data)
}



pub struct Data<T> {
    pub packet_length: u32,
    pub packet_id: u32,
    pub data: T,
}


pub struct Handshake {
    pub packet_length: u32,
    pub packet_id: u32,
    pub protocol_version: u32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: u32,
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

pub fn parse_mystery_packet(length: i32, id: i32, data: Vec<u8>) {
    println!("Mystery Packet");
    println!("Length: {}", length);
    println!("ID: {}", id);
    
    for item in data.clone() {
       println!("{:08b}", item);
    } 







}


