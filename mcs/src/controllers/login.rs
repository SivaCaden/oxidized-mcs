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
use crate::server::Packet;
use crate::models::player::Player;


pub async fn login(mut client:Player, packet: &Packet, key_controller: &mut KeyController, stream: &mut TcpStream) -> Result<Player> {

    println!("login");
    println!("Packet data: {:x?}", packet.data);
    println!("client name: {}", client.name);
    println!("client uuid: {}", client.uuid);
    match packet.id {
        0 => {
            println!("Login Start");
            let (name, uuid) = parse_login_start(packet.data.clone());
            client.name = name.clone();
            client.uuid = uuid.clone();
            println!("    Name: {0}\n    UUID: {1}", name, uuid);
            println!("    Sending Encryption Request");
            let response = craft_encryption_request(key_controller.get_der_key());

            stream.writable().await?;
            stream.write_all(&response).await?;

            return Ok(client);
        }

        1 => {
            println!("Encryption Response");
            let ( mut encrypted_shared_secret, _encrypted_verify_token) =  parse_encryption_response(packet.data.clone());

            let decrypted_shared_secret = key_controller.decrypt(&mut encrypted_shared_secret);
            if decrypted_shared_secret.len() == 16 {
                println!("    Encryption key recieved from {} {}", client.name, client.uuid);
                client.set_aes(decrypted_shared_secret.clone());
                // send a login success packet to client
                
                let mut raw_responce = craft_login_success(client.uuid.clone(), client.name.clone());
                if let Some(ref mut aes_keeper) = client.aes_keeper {
                    aes_keeper.encrypt(&mut raw_responce);
                };

                stream.writable().await?;
                stream.write_all(&raw_responce).await?;
            } else {
                println!("    Encryption Failed");
                // send a login disconnect packet to client for encyrption failure
                // send_disconnect_packet(stream, "Encryption Failed".to_string());
            }
            return Ok(client);
        }
        _ => { println!("Login Failed");
            return Err(Error::new(ErrorKind::Other, "Login Failed"));
        } 
    }

}

