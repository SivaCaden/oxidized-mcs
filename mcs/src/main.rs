use std::{
    io::{ prelude::*, BufReader, Result, Error},
    net::{TcpListener, TcpStream},
    collections::HashMap,
};
use std::fs;

pub mod mc_datatypes;



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
const JOINGAME = ( "join_game".to_string(), vec!["i32", "u8", "u8", "u8", "u8", "u8", "bool;





pub fn handle_connection( mut stream: TcpStream ) {
    let mut buf_reader = BufReader::new(&mut stream);

    let mut buf: Vec<u8> = Vec::new();

    let data = buf_reader.read(&mut buf);

    

    let size = buf.len();


    println!("{}", size);
    
    



    

    
}


fn start_encryption() -> T {
    let mut data = String::new();


}
