/*
 * Server Controller for login packets.
 * handles only "Login" state packets.
 *
 * Authored by: Three rats in a trench coat.
*/
use std::io::{ ErrorKind, Result, Error };
use tokio::{io::AsyncWriteExt, net::TcpStream};
use crate::util::packet_parser::*;
use crate::util::packet_crafter::*;
use crate::controllers::key_controller::KeyController;
use crate::Packet;


pub async fn login(packet: &Packet, key_controller: &KeyController, stream: &mut TcpStream) -> Result<()> {

                println!("login");
                println!("Packet data: {:x?}", packet.data);
                match packet.id {
                    0 => {
                        println!("Login Start");
                        let (name, uuid) = parse_login_start(packet.data.clone());
                        let player_name = name;
                        let player_uuid = uuid;
                        println!("    Name: {0}\n    UUID: {1}", player_name, player_uuid);
                        println!("    Sending Encryption Request");
                        let response = craft_encryption_request(key_controller.get_public_key());

                        stream.writable().await?;
                        stream.write_all(&response).await?;

                        return Ok(());
                    }

                    1 => {
                        println!("Encryption Response");
                        
                        // if response is good
                        // send login success

                        return Ok(());

                    }
                    
                    _ => { println!("Login Failed");
                        return Err(Error::new(ErrorKind::Other, "Login Failed"));
                    

                    } 
                }
                
}

