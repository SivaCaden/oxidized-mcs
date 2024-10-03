/*
 * Server Controller for login packets.
 * handles only "Login" state packets.
 *
 * Authored by: Three rats in a trench coat.
*/


/*
 * Invistigation of the verify token missmatch case
 * 
 * Things that need to happen before the token is sent
 * --------------------------------------------------------
 * Making public and private keys
 *
 * Encodeing the public key in the ASN.1 DER format
 *      the server dosen't crash when the key is "encoded" in the right format
 *
 * Generating the verify token
 *      the verify token is a random 4 byte number according the wiki
 *
 *      server "appears" to generate a vaild token and even print it out to the terminal
 *      
 *
 *      testing ------
 *
 *      we can try to make the verify token not to 
 *      spec and see if the notchian client will accept it
 *
 * sending the verify token and public key to the client
 *      intercept the packets with WireShark???
 *
 *
 * recieving the encyrpted AES key and verify token 
 * from the client using the server's public key
 *
 *      we can recieve a encrypted AES key and what is supposed to be
 *      the verify token from the client
 *
 *      we can decrypt the AES key with the server's private key
 *      but we CANNOT decrypt the verify token as it throws an error
 *
 *
 *
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
    println!("client name: {}", client.name);
    println!("client uuid: {}", client.uuid);
    match packet.id {
        0 => {
            println!("Login Start");
            let (name, uuid) = parse_login_start(packet.data.clone());
            client.name = name.clone();
            client.uuid = uuid.clone();
            println!("    Crafting Encryption Request");
            let (response, verify_token) = craft_encryption_request(key_controller.get_der_key());

            let meme = verify_token.clone();
            if the_meaning_of_life(meme, key_controller) {
                println!("    the program can generate, encrypt and decrypt the verify token on its own with pgp encryption");
            } else {
                println!("    the server sould be scrapped compleatly and thrown into a fire");
            }


            let encyrpted_verify_token = key_controller.encrypt(&mut verify_token.clone());
            client.set_verify_token(encyrpted_verify_token);

            println!("    Sending Encryption Request");
            stream.writable().await?;
            stream.write_all(&response).await?;

            return Ok(client);
        }

        1 => {
            println!("Encryption Response");
            let ( mut encrypted_shared_secret, encrypted_verify_token) =  parse_encryption_response(packet.data.clone());

            if encrypted_verify_token != client.get_verify_token() {
                println!("    Verify Token Mismatch");
                println!("    Expected: {:x?}", client.get_verify_token());
                println!("    Recieved: {:x?}", encrypted_verify_token);
                panic!("Verify Token Mismatch");
                // send a login disconnect packet to client for verify token mismatch
                // send_disconnect_packet(stream, "Verify Token Mismatch".to_string());
            }

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


fn the_meaning_of_life(verify_token: Vec<u8>, key_controller: &mut KeyController) -> bool{

    let encyrpted_verify_token = key_controller.encrypt(&mut verify_token.clone());

    println!("    test encrypting the verify token: {:x?}", verify_token);

    let decrypted_verify_token = key_controller.decrypt(&mut encyrpted_verify_token.clone());

    println!("    test decrypting the verify token: {:x?}", decrypted_verify_token);

    if verify_token == decrypted_verify_token {
        return true
    }
    




    false

}
