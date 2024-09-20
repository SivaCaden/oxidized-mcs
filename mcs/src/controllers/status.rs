/*
 * Server Controller for status packets.
 * handles only "Status" state packets.
 *
 * Authored by: Three rats in a trench coat.
*/



use std::io::{ ErrorKind, Result }; 
use tokio::{io::AsyncWriteExt, net::TcpStream};
use crate::util::packet_crafter::*;
use crate::server::Packet;


pub async fn handle_status(stream: &mut TcpStream, raw_data:Vec<u8>, packet: &Packet) -> Result<()> {
    match packet.id {
        0 => {
            println!("Request");
            let response = craft_status_response();

            stream.writable().await?;
            stream.flush().await?;
            stream.write_all(&response).await?;
            return Ok(());
        }

        1 => {
            println!("Ping");
            let response = raw_data.clone();
            println!("Pong!");
            stream.writable().await?;
            stream.write_all(&response).await?;
            return Ok(());
        }

        _ => {
            println!("Status Failed");
            return Err(std::io::Error::new(ErrorKind::InvalidData, "Invalid Packet ID"));
        }
    }
}
