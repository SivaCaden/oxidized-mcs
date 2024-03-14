#![allow(unused_imports, 
        dead_code, 
        unused_variables,
        unused_mut,
        unused_assignments,
        unreachable_code
        )]
use std::{
    io::{ prelude::*, BufReader, Result, Error},
    net::{TcpListener, TcpStream},
    collections::HashMap,
};


pub mod mc_datatypes;
use mc_datatypes::VarInt;

// pub mod big_parse;



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





pub fn handle_connection( mut stream: TcpStream ) -> Result<()> {
    let mut buf_reader = BufReader::new(&mut stream);

    let mut data = Vec::new();
    let _ = buf_reader.read_to_end(&mut data);

    let size = data.len();
    println!("Data Size: {}", size);

    let (packet_length, packet_id, data) = (0, 0, vec![0]); // parse_length_pack_id(&data);
    println!("Packet Length: {0}\nPacketID:{1}", packet_length, packet_id); 
    for item in data {
        println!("{:02X}", item);
    }
    
    Ok(())
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn varint() {
        let mut packet = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x07,// Max Value of VarInt 2147483647
                            0xDD, 0xC7, 0x01,              // 25565, default port
                            0x01,                          // 1
                            0x80, 0x80, 0x80, 0x80, 0x08,  // Min Value of VarInt -2147483648
                            0x69, 0x69];                   // Other data at end of packet
        let (num1, packet) = VarInt::decode(packet); 
        let (num2, packet) = VarInt::decode(packet);
        let (num3, packet) = VarInt::decode(packet);
        let (num4, packet) = VarInt::decode(packet);

        assert_eq!(num1, 2147483647);
        assert_eq!(num2, 25565);
        assert_eq!(num3, 1);
        assert_eq!(num4, -2147483648);
        assert_eq!(packet, vec![0x69, 0x69]);
    }
}

