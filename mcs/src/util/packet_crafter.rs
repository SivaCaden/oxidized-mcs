// Creates outgoing packets for Minecraft server.
// Authored by: Three rats in a trench coat.

#![allow(dead_code)]

use base64::{engine::general_purpose, Engine as _};
use rsa::{traits::PublicKeyParts, RsaPublicKey};

use crate::util::datatypes::*;

// mode 1 status packet ids
const STATUS_RESPONCE_PACKET_ID: u8 = 0x00;
const STATUS_PING_PACKET_ID: u8 = 0x01;
// mode 2 login packet ids
const LOGIN_DISCONNECT_PACKET_ID: u8 = 0x00;
const LOGIN_ENCRYPTION_REQUEST_PACKET_ID: u8 = 0x01;
const LOGIN_SUCCESS_PACKET_ID: u8 = 0x02;
const LOGIN_SET_COMPRESSION_PACKET_ID: u8 = 0x03;
const LOGIN_PLUGIN_REQUEST_PACKET_ID: u8 = 0x04;
const LOGIN_COOKIE_REQUEST_PACKET_ID: u8 = 0x05;



fn gift_wrap_packet(packet: Vec<u8>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    out = VarInt::encode(packet.len() as i32, out);
    out.extend_from_slice(&packet);
    out
}


pub fn craft_status_response() -> Vec<u8> {
    let json_response = r#"
    {
        "version": {
            "name": "1.21.1",
            "protocol": 767
        },
        "players": {
            "max": 10,
            "online": 0,
            "sample": [
            {
            }
            ]
        },
        "description": {
            "text": "C O R N E D B E E F H A S H W R A P"
        },
        "favicon": "data:image/png;base64,<data>",
        "enforcesSecureChat": true,
        "previewsChat": true
    } 
    "#;

    println!("  sending status response");

    let mut response: Vec<u8> = Vec::new();
    response = VarInt::encode(STATUS_RESPONCE_PACKET_ID as i32, response);
    response = StringMC::encode(json_response.to_string(), response);
    let _length = response.len() as i32;
    gift_wrap_packet(response)
}

pub fn craft_encryption_request(public_key: Vec<u8>) -> Vec<u8> {


    println!("    crafting encryption request");

    println!("    generateing verify token");
    // generate a random 4 byte verify token
    let mut verify_token: Vec<u8> = Vec::new();
    for _ in 0..4 {
        verify_token.push(rand::random::<u8>());
    } 
    //print out the verify token
    println!("    verify token: {:x?}", verify_token);

    let mut response: Vec<u8> = Vec::new();
    response = VarInt::encode(LOGIN_ENCRYPTION_REQUEST_PACKET_ID as i32, response);
    response = StringMC::encode("".to_string(), response);
    response = VarInt::encode(public_key.len() as i32, response);
    response.extend_from_slice(&public_key);
    response = VarInt::encode(verify_token.len() as i32, response);
    response.extend_from_slice(&verify_token);
    // should authenticate through mojang servers?
    response = Bool::encode(false, response);


    gift_wrap_packet(response)




}
