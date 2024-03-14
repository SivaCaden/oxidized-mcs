#![allow(unused_imports, 
        dead_code, 
        unused_variables,
        unused_mut,
        unused_assignments,
        unreachable_code,
        non_snake_case,
        )]
use std::{
    io::{ prelude::*, BufReader, Result, Error, ErrorKind},
        collections::HashMap,
};
use tokio::net::{TcpListener, TcpStream};
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

pub mod mc_datatypes;
use mc_datatypes::VarInt;

pub mod big_parse;
use big_parse::*;

#[tokio::main]
async fn main() ->  Result<()> {

    println!("Spooling Server...");

    {
        let host = "127.0.0.1";
        let port: u16 = 25565;
        let server = TcpListener::bind((host, port)).await.unwrap();

        loop {

            let (stream, addr) = server.accept().await.unwrap();

            println!("new client connected");
            let output = handle_connection(stream).await;
        }
    }


    Ok(())

}





async fn handle_connection( mut stream: TcpStream ) -> Result<()> {
    stream.readable().await?;

    let mut data = Vec::new();
    let mut buf = Vec::new();

    match stream.try_read_buf( &mut buf) {
        Ok(0) => {
            println!("Connection Closed");
            return Ok(());
        }
        Ok(n) => {
            data.extend_from_slice(&buf[..n]);
        }
        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
            println!("Would Block");
        }
        Err(e) => {
            println!("Failed to read from socket; err = {:?}", e);
            return Err(e);
        }
    }
    
    



    let size = data.len();
    println!("Data Size: {}", size);


    let (packet_length, packet_id, data) = parse_length_packid(data);


    println!("Packet Length: {0}\nPacketID:{1}", packet_length, packet_id); 
    for item in data {
        println!("{:02X}", item);
    }

    let responce = "ligma";
    if packet_id == 0x00 {
        let responce = "ligma";
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

