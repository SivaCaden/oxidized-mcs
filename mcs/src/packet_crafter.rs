use crate::mc_datatypes::*;
use base64::{engine::general_purpose, Engine as _};
use rsa::{traits::PublicKeyParts, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};



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


pub fn craft_status_responce() -> Vec<u8> {
    let json_responce = r#"
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

    println!("  sending status responce");

    let mut responce: Vec<u8> = Vec::new();
    responce = VarInt::encode(STATUS_RESPONCE_PACKET_ID as i32, responce);
    responce = StringMC::encode(json_responce.to_string(), responce);
    let lenght = responce.len() as i32;
    gift_wrap_packet(responce)
}

pub fn craft_encryption_request(public_key: RsaPublicKey) -> Vec<u8> {


    println!("    crafting encryption request");

    println!("    generateing verify token");
    // generate a random 4 byte verify token
    let mut verify_token: Vec<u8> = Vec::new();
    for _ in 0..4 {
        verify_token.push(rand::random::<u8>());
    }

    let mut responce: Vec<u8> = Vec::new();
    responce = VarInt::encode(LOGIN_ENCRYPTION_REQUEST_PACKET_ID as i32, responce);
    // server id "appears to be empty"
    responce = StringMC::encode("".to_string(), responce);
    // public key length as a Varint
    responce = VarInt::encode(public_key.size() as i32, responce);
    // encode the public key bites as base64 and wrap in PEM
    let base_64_key = general_purpose::STANDARD.encode(public_key.n().to_bytes_be());
    let pem = format!("-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----", base_64_key);
    responce.extend_from_slice(pem.as_bytes());
    // verify token length as a Varint
    responce = VarInt::encode(verify_token.len() as i32, responce);
    // verify token in bytes
    responce.extend_from_slice(&verify_token);
    // should authenticate through mojang servers?
    responce = Bool::encode(false, responce);


    gift_wrap_packet(responce)




}
