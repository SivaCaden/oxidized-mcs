/*
 * Server Controller for handshake packets.
 * handles only "handshake" state packets.
 *
 * Authored by: Three rats in a trench coat.
*/

use std::io::{ Error, ErrorKind, Result };
use crate::State;
use crate::Packet;
use crate::util::packet_parser::*;


async fn handel_handshake(addr: &String, packet: &Packet) -> Result<State>{

        println!("initiating handshake with {addr}");
        match parse_handshake(packet.length, packet.id, packet.data.clone()) {
            1 => {
                let state = State::Status;
                println!("Handshake Success");
                return Ok(state)
            },

            2 => {
                return Ok(State::Login);
            },

            _ => {
                println!("Handshake Failed");
                return Err(Error::new(ErrorKind::Other, "Handshake Failed"));
            }
        }
    }
    

