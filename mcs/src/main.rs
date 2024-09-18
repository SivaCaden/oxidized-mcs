/*
 * Main file for a Rust-based Minecraft Server (WIP)
 * based entirely on https://wiki.vg/Protocol
 * Authored by: Three rats in a trench coat
 *              (Caden Siva, Foster Sullivan, and the one brain cell)
 *
 * File Structure is/will be as follows:
 * src
 * | main.rs - this file (spools server)
 * | server.rs - primary server router 
 * |
 * |--- util - directory for various utility libraries
 * |    | mod.rs - links submodules together
 * |    | datatypes.rs - Converts hex packets into Rust-friendly objects
 * |    | vector.rs - Basic 2D and 3D Vector library
 * |    | packet.rs - Class describing a packet, with methods to encode/decode packets.
 * |
 * |--- models - directory for database models
 * |    | mod.rs - links submodules together
 * |    | block.rs - Block class representing a minecraft block  
 * |    | player.rs - Player class representing a minecraft player
 * |    | entity.rs - Entity class representing a minecraft entity
 * |    | world.rs - World class representing the minecraft world
 * |    | config.rs - Config class representing the minecraft config
 * |
 * |--- controllers - directory for controllers to be routed to and to control
 * |    |             database models to store data
 * |    | mod.rs - links submodules together
 * |    | game_control.rs - "Game" state packets go here (may be split later)
 * |    | login_control.rs - "Login" state packets go here
 * |    | handshake_control.rs - "Handshake" state packets go here 
 * |    | config_control.rs - "Config" state packets go here 
 * |    | status_control.rs - "Status" state packets go here (may be merged with HS later)
*/

// Including all directories in crate hierarchy
pub mod util;
pub mod controllers{
    pub mod handshake;
}
pub mod models;
pub mod server;
pub mod tests;

use std::io::{ ErrorKind, Result }; 
use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpStream}};
use rand::thread_rng;
use rsa::{RsaPrivateKey, RsaPublicKey};

use util::packet_crafter::*;
use util::packet_parser::*;

#[derive(Debug, Copy, Clone)]
pub enum State {
    Handshake,
    Status,
    Login,
    _Play,
}

const HOST: &str = "127.0.0.1";
const PORT: u16 = 25565;

#[tokio::main]
async fn main() {

    println!("Spooling Server...");
    println!("Generating Keys...");
    // ok I know this is not the secure way of generating and storing
    // cryptographic keys but I just want this to work ok?
    // throw this in a seperate file later please caden? (ok)
    let mut rng = thread_rng();
    let bits = 1024;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("   Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    println!("Keys Generated");

    let host = HOST;
    let port: u16 = 25565;
    let addr = format!("{}:{}", host, port).to_string();
        
    let state = State::Handshake;

    let server = match TcpListener::bind(addr.clone()).await {
        Ok(server) => {
            println!("Server Bound to: {}", addr);
            server
        }
        Err(e) => {
            println!("  Failed to bind to address: {}", e);
            println!("  binding to localhost instead");
            let addr = format!("{}:{}", "127.0.0.1", PORT).to_string();
            TcpListener::bind(addr).await.unwrap()
        }
    };

    loop {
        let public_key = public_key.clone();
        let private_key = private_key.clone();

        println!("Waiting for connection...");

        let (stream, addr) = server.accept().await.unwrap();

        println!("\n=====================");
        println!("client connected");
        
        tokio::spawn(async move {
            let _ = handle_connection( addr.to_string(), stream, state, public_key.clone(), private_key.clone()).await.unwrap();
        });
        
    }
}

pub struct Packet {
    length: i32,
    id: i32,
    data: Vec<u8>,
}

impl Packet {
    pub fn new(length: i32, id: i32, data: Vec<u8>) -> Self {
        Packet {
            length,
            id,
            data,
        }
    }
    pub fn get_info(&self) {
        println!("Packet Length: {0}\nPacket ID: {1}", self.length, self.id);
    }
}


async fn handle_connection( addr: String, mut stream: TcpStream, mut state: State , public_key: RsaPublicKey, _private_key: RsaPrivateKey) -> Result<()>{
    let mut player_name;
    let mut player_uuid;
    
    let mut buf = Vec::new();
    loop {

        println!("attempting to read from stream");
        stream.readable().await?;
        println!("Reading from stream");
        let mut raw_data = Vec::new();
        match stream.try_read_buf( &mut buf) {
            Ok(0) => {
                println!("  Connection Closed");
                break;
            }
            Ok(n) => {
                raw_data.extend_from_slice(&buf[..n]);
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                println!("Failed to read from socket; err = {:?}", e);
                return Err(e);
            }
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
                println!("initiating handshake with {addr}");
                match parse_handshake(packet.length, packet.id, packet.data) {

                    1 => {
                        state = State::Status;
                        println!("Handshake Success");
                    },

                    2 => {
                        state = State::Login;
                    },

                    _ => {
                        println!("Handshake Failed");
                        buf.clear();
                    }
                }
                stream.writable().await?;
                stream.flush().await?;
                buf.clear();
            }
            
            State::Status => {
                println!("Status");

                match packet.id {
                    0 => {
                        println!("Request");
                        let response = craft_status_response();
                        
                        stream.writable().await?;
                        stream.flush().await?;
                        stream.write_all(&response).await?;
                        stream.writable().await?;
                        stream.flush().await?;
                        buf.clear();
                    }

                    1 => {
                        println!("Ping");
                        let response = raw_data.clone();
                        println!("Pong!");
                        stream.writable().await?;
                        stream.write_all(&response).await?;
                        buf.clear();
                    }

                    _ => {
                        println!("Status Failed");
                        buf.clear();
                    }
                }
            }

            State::Login => {
                println!("login");
                println!("Packet data: {:x?}", packet.data);
                match packet.id {
                    0 => {
                        println!("Login Start");
                        let (name, uuid) = parse_login_start(packet.data);
                        player_name = name;
                        player_uuid = uuid;
                        println!("    Name: {0}\n    UUID: {1}", player_name, player_uuid);
                        println!("    Sending Encryption Request");
                        let response = craft_encryption_request(public_key.clone());

                        stream.writable().await?;
                        stream.write_all(&response).await?;
                        stream.writable().await?;
                        stream.flush().await?;

                        buf.clear();
                    }

                    1 => {
                        println!("Encryption Response");
                        
                        // if response is good
                        // send login success

                        buf.clear();
                    }
                    
                    _ => {
                        println!("Login Failed");
                        buf.clear();

                    }
                }
            }

            _ => {
                panic!("Unimplemented Packet State, {:?}", state);
            }
        }
    }
    Ok(())
}
