use crate::mc_datatypes::*;



const STATUS_RESPONCE_PACKET_ID: u8 = 0x00;
pub fn craft_status_responce() -> Vec<u8> {
    let json_responce = r#"
    {
        "version": {
            "name": "1.20.4",
            "protocol": 754
        },
        "players": {
            "max": 42,
            "online": 0,
            "sample": [
                {
                    "name": "thinkofdeath",
                    "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
                } 
            ]
        },
        "description": {
            "text": "TO MANY CRABS IN THE BUCKET!"
        }
    }
    "#;

    let mut responce: Vec<u8> = Vec::new();
    responce.push(STATUS_RESPONCE_PACKET_ID);
    StringMC::encode(json_responce.to_string(), &mut responce);

    let lenght = responce.len() as i32;


    let mut out: Vec<u8> = Vec::new();
    VarInt::encode(lenght, &mut out);
    out.extend_from_slice(&responce);
    out
}



