#![allow(unused_imports, 
        dead_code, 
        unused_variables,
        unused_mut,
        unused_assignments,
        unreachable_code,
        non_snake_case,
        )]
use std::{
    collections::HashMap, io::{ prelude::*, BufReader, Error, ErrorKind, Result}, net::SocketAddrV4
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
use mc_datatypes::*;

pub mod big_parse;
use big_parse::*;

#[derive(Debug, Copy, Clone)]
enum State {
    Handshake,
    Status,
    Login,
    Play,
}

#[tokio::main]
async fn main() ->  Result<()> {

    println!("Spooling Server...");

    {
        let host = "127.0.0.1";
        let port: u16 = 25565;
        let server = TcpListener::bind((host, port)).await.unwrap();
        
        let mut state = State::Handshake;

        loop {

            let (stream, addr) = server.accept().await.unwrap();

            println!("new client connected");
            state = handle_connection( addr.to_string(), stream, state).await.unwrap();
        }
    }
    Ok(())
}


async fn handle_connection( addr: String, mut stream: TcpStream, mut state: State ) -> Result<State> {
    
    
    stream.readable().await?;

    let mut data = Vec::new();
    let mut buf = Vec::new();

    match stream.try_read_buf( &mut buf) {
        Ok(0) => {
            println!("Connection Closed");
            return Ok(State::Handshake);
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

    match state {
        State::Handshake => {
            println!("initiating handshake with {addr}");
            parse_handshake(packet_length, packet_id, data.clone());
            println!("assuming request of state");
            return Ok(State::Status);
            

        }
        State::Status => {
            println!("Status");

        }
        
        _ => {
            println!("Not Handshake");
        }
    }


    println!("Packet Length: {0}\nPacketID:{1}", packet_length, packet_id); 
    //for item in data {
    //    println!("{:02X}", item);
    //}

    

    Ok(State::Handshake)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn varint() {
        // DECODE TESTS
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

        // ENCODE TESTS
        let num1: i32 = 2147483647;
        let num2: i32 = 25565;
        let num3: i32 = 1;
        let num4: i32 = -2147483648;
        
        let mut out: Vec<u8> = vec![];

        assert_eq!(vec![0xFF, 0xFF, 0xFF, 0xFF, 0x07], *VarInt::encode(num1, &mut out));
        assert_eq!(vec![0xFF, 0xFF, 0xFF, 0xFF, 0x07, 0xDD, 0xC7, 0x01], *VarInt::encode(num2, &mut out));
        assert_eq!(vec![0xFF, 0xFF, 0xFF, 0xFF, 0x07, 0xDD, 0xC7, 0x01, 0x01], *VarInt::encode(num3, &mut out));
        assert_eq!(vec![0xFF, 0xFF, 0xFF, 0xFF, 0x07, 0xDD, 0xC7, 0x01, 0x01, 0x80, 0x80, 0x80, 0x80, 0x08], *VarInt::encode(num4, &mut out));

    }

    #[test]
    fn string_mc() {
        // DECODE TEST
        let packet = vec![  0x28,                                        // VarInt length
                        0x54, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x6f, 0x72, 
                        0x20, 0x6e, 0x6f, 0x74, 0x20, 0x74, 0x6f, 0x20,
                        0x62, 0x65, 0x2c, 0x20, 0x54, 0x68, 0x61, 0x74,
                        0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
                        0x71, 0x75, 0x65, 0x73, 0x74, 0x69, 0x6f, 0x6e,  // To be or not to be, That is the question
                        0x69, 0x69, 0x69, 0x69, 0x69, 0x69, 0x69, 0x69]; // Misc. other data

       let (string, packet) = StringMC::decode(packet);
       assert_eq!(string, String::from("To be or not to be, That is the question"));
       assert_eq!(packet, vec![0x69, 0x69, 0x69, 0x69, 0x69, 0x69, 0x69, 0x69]);

       // ENCODE TEST
       let packet = vec![  0x28,                                         // VarInt length
                        0x54, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x6f, 0x72, 
                        0x20, 0x6e, 0x6f, 0x74, 0x20, 0x74, 0x6f, 0x20,
                        0x62, 0x65, 0x2c, 0x20, 0x54, 0x68, 0x61, 0x74,
                        0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
                        0x71, 0x75, 0x65, 0x73, 0x74, 0x69, 0x6f, 0x6e]; // To be or not to be,
                                                                         // That is the question
       let string = String::from("To be or not to be, That is the question"); 
       assert_eq!(packet, StringMC::encode(string)); 
    }
}

