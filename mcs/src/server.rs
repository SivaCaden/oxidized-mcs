// Primary server router for Minecraft Server following https://wiki.vg/Protocol
// Authored by: Three rats in a trench coat


use crate::{ controllers::key_controller::KeyController, util::packet_parser::make_packet};
use crate::controllers::{handshake::handel_handshake, status::handle_status, login::login};
use std::io::{ ErrorKind, Result };

use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpStream}};

const HOST: &str = "127.0.0.1";
const PORT: u16 = 25565;

#[derive(Debug, Copy, Clone)]
pub enum State { Handshake, Status, Login, _Play, }

pub async fn run(addr: String) {
    // ok I know this is not the secure way of generating and storing
    // cryptographic keys but I just want this to work ok?
    // throw this in a seperate file later please caden? (ok)
    
    let key_controller = KeyController::new();
    

        
    let state = State::Handshake;

    let server = match TcpListener::bind(addr.clone()).await {
        Ok(server) => { println!("Server Bound to: {}", addr); server }
        Err(e) => {
            println!("  Failed to bind to address: {}", e);
            println!("  binding to localhost instead");
            let addr = format!("{}:{}", "127.0.0.1", PORT).to_string();
            TcpListener::bind(addr).await.unwrap()
        }
    };

    loop {
        let clone_key_controller = key_controller.clone();


        println!("Waiting for connection...");

        let (stream, addr) = server.accept().await.unwrap();

        println!("\n=====================");
        println!("client connected");
        
        tokio::spawn(async move { let _ = handle_connection( addr.to_string(), stream, state, clone_key_controller).await.unwrap(); });
        
    }
}

pub struct Packet { pub length: i32, pub id: i32, pub data: Vec<u8>, }

impl Packet {
    pub fn new(length: i32, id: i32, data: Vec<u8>) -> Self {
        Packet { length, id, data, }
    }
    pub fn get_info(&self) { println!("Packet Length: {0}\nPacket ID: {1}", self.length, self.id); }

    pub fn get_data(&self) -> Vec<u8> { self.data.clone() }
}


async fn handle_connection( addr: String, mut stream: TcpStream, mut state: State , key_controller: KeyController) -> Result<()>{

    let mut buf = Vec::new();
    loop {

        println!("attempting to read from stream");
        stream.readable().await?;
        println!("Reading from stream");
        let mut raw_data = Vec::new();
        match stream.try_read_buf( &mut buf) {
            Ok(0) => { println!("  Connection Closed"); break; }
            Ok(n) => { raw_data.extend_from_slice(&buf[..n]); }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => { continue; }
            Err(e) => { println!("Failed to read from socket; err = {:?}", e); return Err(e); }
        }

        println!("STATE: {:?}", state);

        println!("======================\n");

        let size = raw_data.len();
        let _meme = raw_data.clone();
        let packet = make_packet(raw_data.clone());

        println!("Data Size: {}", size);
        packet.get_info();


        match state {
            State::Handshake => {
                state = match handel_handshake(&addr, &packet).await{
                    Ok(new_state) => new_state,
                    Err(e) => { println!("Handshake Failed: {:?}", e); State::Handshake }
                };
            }

            State::Status => {
                println!("Status");

                match handle_status(&mut stream, raw_data.clone(), &packet).await {
                    Ok(_) => {println!("Status Success"); },
                    Err(e) => { println!("Status Failed: {:?}", e); }
                }

            }

            State::Login => {
                println!("Login");

                match login(&packet, &key_controller, &mut stream).await {
                    Ok(_) => {println!("Login Success"); },
                    Err(e) => { println!("Login Failed: {:?}", e); }
                }
            }
            State::_Play => {
                println!("Play");
            }

        }



        stream.writable().await?;
        stream.flush().await?;
        buf.clear();
    }
   Ok(()) 
}
