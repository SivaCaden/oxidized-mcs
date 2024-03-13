use crate::mc_datatypes::VarInt;

pub fn parse<T>(data: &[u8]) -> Data<T> {
    let data = data.to_vec();
    let (packet_length, data) = VarInt::decode(data);
    let (packet_id, data) = match packet_length {
        0x00 => {
            panic!("Packet Length is 0x00");
            
        }
        _ => {
            VarInt::decode(data)
        }
    };
    




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


