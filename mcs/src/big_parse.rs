use crate::mc_datatypes::VarInt;

fn pain_packet_info(data: &[u8]) -> (u32, u32, Vec<u8>) {
    let mut some_varint: Vec<u8> = Vec::new();
    let mut out_vec: Vec<i32> = Vec::new();
    let mut out_data: Vec<u8> = Vec::new();
    let mut count = 0;

    for byte in data {
        if count >= 2 {
            out_data.push(*byte);
            continue;
        }

        if byte & 0x80 == 0x80 {
            some_varint.push(*byte);
        }
        if byte & 0x80 != 0x80 {
            count += 1;
            some_varint.push(*byte);
            out_vec.push(VarInt::decode(some_varint));
            println!("END OF NUMBER");
            some_varint.clear();
        }
    }
    let packet_length = out_vec[0] as u32;
    let packet_id = out_vec[1] as u32;
    (packet_length, packet_id, out_data)
}


fn pain_string_decode(data: &[u8]) -> (u32, u32, Vec<u8>) {
    let mut some_varint: Vec<u8> = Vec::new();
    let mut out_vec: Vec<i32> = Vec::new();
    let mut out_data: Vec<u8> = Vec::new();
    let mut count = 0;

    for byte in data {
        if count >= 2 {
            out_data.push(*byte);
            continue;
        }

        if byte & 0x80 == 0x80 {
            some_varint.push(*byte);
        }
        if byte & 0x80 != 0x80 {
            count += 1;
            some_varint.push(*byte);
            out_vec.push(VarInt::decode(some_varint));
            println!("END OF NUMBER");
            some_varint.clear();
        }
    }
    let packet_length = out_vec[0] as u32;
    let packet_id = out_vec[1] as u32;
    (packet_length, packet_id, out_data)
}


pub fn parse<T>(data: &[u8]) -> Data<T> {
   
    let (packet_length, packet_id, out_data) = pain_packet_info(data);

    match packet_id {
        _ => {
            println!("Packet ID: {}", packet_id);
            Parse_Handshake(packet_length, packet_id, &out_data)
        }
    }

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


pub fn Parse_Handshake<T>(length: u32, id: u32, data: &[u8]) -> Data<T>{
    let protocol_version: i32;
    let server_address: String;
    let server_port: u16;
    let next_state: i32;

    let expected_length = length as u32;
    protocol_version = VarInt::decode(data.to_vec());
    




}


