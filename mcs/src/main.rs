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
use mc_datatypes::*;

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

        VarInt::encode(num1, &mut out);
        assert_eq!(vec![0xFF, 0xFF, 0xFF, 0xFF, 0x07], out);
        VarInt::encode(num2, &mut out);
        assert_eq!(vec![0xFF, 0xFF, 0xFF, 0xFF, 0x07, 0xDD, 0xC7, 0x01], out);
        VarInt::encode(num3, &mut out);
        assert_eq!(vec![0xFF, 0xFF, 0xFF, 0xFF, 0x07, 0xDD, 0xC7, 0x01, 0x01], out);
        VarInt::encode(num4, &mut out);
        assert_eq!(vec![0xFF, 0xFF, 0xFF, 0xFF, 0x07, 0xDD, 0xC7, 0x01, 0x01, 0x80, 0x80, 0x80, 0x80, 0x08], out);

    }

    #[test]
    fn string_mc() {
        // DECODE TEST
        let packet = vec![0x28,                                          // VarInt length
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
       let packet = vec![0x28,                                           // VarInt length
                        0x54, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x6f, 0x72, 
                        0x20, 0x6e, 0x6f, 0x74, 0x20, 0x74, 0x6f, 0x20,
                        0x62, 0x65, 0x2c, 0x20, 0x54, 0x68, 0x61, 0x74,
                        0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
                        0x71, 0x75, 0x65, 0x73, 0x74, 0x69, 0x6f, 0x6e]; // To be or not to be,
                                                                         // That is the question
       let string = String::from("To be or not to be, That is the question"); 
       let mut out = vec![];
       StringMC::encode(string,&mut out);
       assert_eq!(packet, out); 
    }

    #[test]
    fn bool_thru_double() {
        // DECODE TEST
        let packet = vec![0x01, 0x00,               // Bool true, false
                            0x81,                   // Byte -127
                            0xFF,                   // UByte 255
                            0x80, 0x01,             // Short -32767
                            0xFF, 0xFF,             // UShort 65535
                            0x80, 0x00, 0x00, 0x00, // Int -2147483648
                            0x7F, 0xFF, 0xFF, 0xFF,  
                            0xFF, 0xFF, 0xFF, 0xFF, // Long 9223372036854775807                                            
                            0x40, 0xDC, 0xCC, 0xCD, // Float 6.9 (nice)                        
                            0x40, 0x1B, 0x99, 0x99,
                            0x99, 0x99, 0x99, 0x9A, // Double 6.9 (also nice)
                            ];

        let (bool1, packet) = Bool::decode(packet);
        let (bool2, packet) = Bool::decode(packet);
        let (byte, packet) = Byte::decode(packet);
        let (ubyte, packet) = UByte::decode(packet);
        let (short, packet) = Short::decode(packet);
        let (ushort, packet) = UShort::decode(packet);
        let (int, packet) = Int::decode(packet);
        let (long, packet) = Long::decode(packet);
        let (float, packet) = Float::decode(packet);
        let (double, packet) = Double::decode(packet);

        assert_eq!(bool1, true);
        assert_eq!(bool2, false);
        assert_eq!(byte, -127 as i8);
        assert_eq!(ubyte, 255 as u8);
        assert_eq!(short, -32767 as i16);
        assert_eq!(ushort, 65535 as u16);
        assert_eq!(int, -2147483648 as i32);
        assert_eq!(long, 9223372036854775807 as i64);
        assert_eq!(float, 6.9 as f32);
        assert_eq!(double, 6.9 as f64);
    }
}

