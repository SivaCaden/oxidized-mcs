// Creates outgoing packets for Minecraft server.
// Authored by: Three rats in a trench coat.

#![allow(dead_code)]


use crate::util::datatypes::*;
// use serde::{Serialize};

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

// constatns for server responce
const MAX_PLAYERS: i32 = 10;
const VERSION: &str = "1.21.1";
const PROTOCOL: i32 = 767;

fn gift_wrap_packet(packet: Vec<u8>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    out = VarInt::encode(packet.len() as i32, out);
    out.extend_from_slice(&packet);
    out
}

struct StatusResponse {
    version: String,
    protocol: i32,
    players: i32,
    max_players: i32,
    description: String,
    favicon: String,
    enforces_secure_chat: bool,
    previews_chat: bool,
}

pub fn craft_status_response() -> Vec<u8> {
    let online_players = 0;
    let description = "C O R N E D B E E F H A S H W R A P";
    let favicon = "data:image/png;base64,<data>";
    let has_secure_chat = true;
    let has_previews_chat = true;

    let _raw = StatusResponse {
        version: VERSION.to_string(),
        protocol: PROTOCOL,
        players: online_players,
        max_players: MAX_PLAYERS,
        description: description.to_string(),
        favicon: favicon.to_string(),
        enforces_secure_chat: has_secure_chat,
        previews_chat: has_previews_chat,
    };


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

pub fn craft_encryption_request(public_key: Vec<u8>) -> (Vec<u8>, Vec<u8>) {


    println!("    crafting encryption request");

    println!("    generateing verify token");
    // generate a random 4 byte verify token
    let mut verify_token: Vec<u8> = Vec::new();
    for _ in 0..4 {
        verify_token.push(rand::random::<u8>());
    } 
    //print out the verify token
    println!("    verify token: {:x?}", verify_token);
    println!("    verify token length: {}", verify_token.len());

    let mut response: Vec<u8> = Vec::new();
    response = VarInt::encode(LOGIN_ENCRYPTION_REQUEST_PACKET_ID as i32, response);
    response = StringMC::encode("".to_string(), response);
    response = VarInt::encode(public_key.len() as i32, response);
    response.extend_from_slice(&public_key);
    response = VarInt::encode(verify_token.len() as i32, response);
    // verify token in bytes
    response.extend_from_slice(&verify_token);
    // should authenticate through mojang servers?
    response = Bool::encode(false, response);


    return (gift_wrap_packet(response), verify_token);

}

pub fn craft_login_success(uuid: String, name: String) -> Vec<u8> {
    println!("    crafting login success");
    let mut response: Vec<u8> = Vec::new();
    response = VarInt::encode(LOGIN_SUCCESS_PACKET_ID as i32, response);
    response = StringMC::encode(uuid, response);
    response = StringMC::encode(name, response);
    gift_wrap_packet(response)
}
