use std::{
    io::{ prelude::*, BufReader, Result, Error},
    net::{TcpListener, TcpStream},
    collections::HashMap,
};
use log::info;

use std::fs;

pub mod mc_datatypes;
use mc_datatypes::VarInt;


pub struct PubKeyInfo {
    sequence: String,
    algorithm: String,
    parameters: String,
}
pub struct PubKey {
    modulus: usize,
    public_exponent: usize,
}



fn main() ->  Result<()> {

    println!("Spooling Server...");

    {
        let host = "localhost";
        let port: u16 = 25565;
        let server = TcpListener::bind((host, port)).unwrap();
        
        loop {

            for stream in server.incoming() {
                println!("new client connected");
                handle_connection(stream.unwrap());
            }
        }
    }


    Ok(())

}
pub struct PackType {
    pub header: String,
    pub data_vec: Vec<String>,
}


pub enum PacketType {
    JoinGame (String, Vec<String>),
    PluginMessage (String, Vec<String>),
    ServerDifficutly (String, Vec<String>),
    PlayerAbilitys (String, Vec<String>),
    PlayerPositionAndLook (String, Vec<String>),
    KeepAlive (String, Vec<String>),
    TimeAlive (String, Vec<String>),
    ChatMessage (String, Vec<String>), 
}


fn parse_length_pack_id(data: &[u8]) -> (Vec<i32>, Vec<u8>){
    let mut some_varint:Vec<u8> = Vec::new();
    let mut out_vec: Vec<i32> = Vec::new();
    let mut FUCK: Vec<u8> = Vec::new();
    let mut count = 0;

    for byte in data{
        if count >= 2 {
            FUCK.push(*byte);
            continue;
        }
        if byte & 0b10000000 == 0b10000000{
            some_varint.push(*byte);
        }
        if byte & 0b10000000 != 0b10000000 {
            count += 1;
            some_varint.push(*byte);
            out_vec.push(VarInt::decode(some_varint.clone()));
            println!("END OF NUMBER");
            some_varint.clear();
        }
    }
    (out_vec, FUCK)
}


pub fn handle_connection( mut stream: TcpStream ) -> Result<()> {
    let mut buf_reader = BufReader::new(&mut stream);

    let mut data = Vec::new();
    let _ = buf_reader.read_to_end(&mut data);

    let size = data.len();
    println!("Data Size: {}", size);

    let (length_id, data) = parse_length_pack_id(&data);
    println!("Packet Length: {0}\nPacketID:{1}", length_id[0], length_id[1]); 
    for item in data {
        println!("{:08b}", item);
    }
    
    Ok(())
    
}



