use crate::mc_datatypes::*;



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


pub struct PacketCrafter {
    pub packet_id: u8,
    pub packet_data: Vec<u8>,
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
    let mut out: Vec<u8> = Vec::new();
    out = VarInt::encode(lenght, out);
    out.extend_from_slice(&responce);
    out
}


