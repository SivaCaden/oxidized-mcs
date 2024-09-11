#![allow(unused_imports, 
        dead_code, 
        unused_variables,
        unused_mut,
        unused_assignments,
        unreachable_code,
        non_snake_case,
        )]
use std::{
    collections::HashMap, 
    time::Duration,
    io::{
        prelude::*, 
        Error, 
        ErrorKind, 
        Result
    }, 
    net::SocketAddrV4
};
use tokio::{io::{AsyncWriteExt, AsyncReadExt, BufReader}, net::{TcpListener, TcpStream}};
use rand::thread_rng;

use crate::mc_datatypes::*;

use crate::big_parse::*;

use crate::packet_crafter::*;


#[derive(Debug, Copy, Clone)]
enum State {
    Handshake,
    Status,
    Login,
    Play,
}

const HOST: &str = "127.0.0.1";
const PORT: u16 = 25565;

async fn big_test() -> anyhow::Result<()> {

    println!("Spooling Server...");

    let host = "127.0.0.1";
    let port: u16 = 25565;
    let addr = format!("{}:{}", HOST, PORT).to_string();
    
    let mut state = State::Handshake;

    let server = TcpListener::bind(addr).await.unwrap();

    loop {

        let (stream, addr) = server.accept().await.unwrap();

        println!("\n=====================");
        println!("client connected");
        
        tokio::spawn(async move {
            let _ = handle_connection( addr.to_string(), stream, state).await.unwrap();
        });

    }
    
    Ok(())
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


async fn handle_connection( addr: String, mut stream: TcpStream, mut state: State ) -> Result<()>{
    let mut player_name = String::new();
    let mut player_uuid = String::new();
    
    let mut buf = Vec::new();
    loop {
        buf.clear();

        println!("attempting to read from stream");
        stream.readable().await?;
        let mut raw_data = Vec::new();
        match stream.try_read_buf( &mut buf) {
            Ok(0) => {
                println!("Connection Closed");
            }
            Ok(n) => {
                raw_data.extend_from_slice(&buf[..n]);
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                panic!("Would Block");
                state = State::Handshake;
            }
            Err(e) => {
                println!("Failed to read from socket; err = {:?}", e);
                return Err(e);
            }
        }

        println!("STATE: {:?}", state);

        println!("======================\n");

        let size = raw_data.len();
        let meme = raw_data.clone();
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
                        stream.writable().await?;
                        stream.flush().await?;


                    },
                    2 => {
                        state = State::Login;
                        stream.writable().await?;
                        stream.flush().await?;

                    },
                    _ => {
                        println!("Handshake Failed");
                    }
                }
                            }
            State::Status => {
                println!("Status");

                match packet.id {
                    0 => {
                        println!("Request");
                        let responce = craft_status_responce();
                        
                        stream.writable().await?;
                        stream.flush().await?;
                        stream.write_all(&responce).await?;
                        stream.writable().await?;
                        stream.flush().await?;
                    }
                    1 => {
                        println!("Ping");
                        let responce = raw_data.clone();
                        stream.writable().await?;
                        stream.write_all(&responce).await?;
                    }
                    _ => {
                        println!("Status Failed");
                    }
                }


            }
            State::Login => {
                println!("login");
                match packet.id {
                    0 => {
                        println!("Login Start");
                        let (name, uuid) = parse_login_start(packet.data);
                        player_name = name;
                        player_uuid = uuid;
                        println!("Name: {0}\nUUID: {1}", player_name, player_uuid);
                        println!("Sending Encryption Request");

                    }
                    1 => {
                        println!("Encryption Response");
                    }
                    _ => {
                        println!("Login Failed");
                    }
                }

            }

            _ => {
                println!("Not Handshake");
            }
        }
    }
    Ok(())
}

