use crate::mc_datatypes::*;



const STATUS_RESPONCE_PACKET_ID: u8 = 0x00;
pub fn craft_status_responce() -> Vec<u8> {
    let json_responce = r#"
    {
        "version": {
            "name": "1.20.4",
            "protocol": 765
        },
        "players": {
            "max": 100,
            "online": 5,
            "sample": [
            {
                "name": "thinkofdeath",
                "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
            }
            ]
        },
        "description": {
            "text": "dev test server :: ballz"
        },
        "favicon": "data:image/png;base64,<data>",
        "enforcesSecureChat": true,
        "previewsChat": true
    } 
    "#;

    let mut responce: Vec<u8> = Vec::new();

    responce = VarInt::encode(STATUS_RESPONCE_PACKET_ID as i32, responce);

    responce = StringMC::encode(json_responce.to_string(), responce);

    let lenght = responce.len() as i32;


    let mut out: Vec<u8> = Vec::new();
    out = VarInt::encode(lenght, out);
    out.extend_from_slice(&responce);
    out
}


pub fn mult_encode_test() {

    let mut out: Vec<u8> = Vec::new();
    
    for i in 0..10 {
        out = VarInt::encode((i + 1) * 10, out);
    }

    let text = "hello world".to_string();
    out = StringMC::encode(text, out);


    let (num, data) = VarInt::decode(out);
    println!("num1: {:X}", num);
    let (num, data) = VarInt::decode(data);
    println!("num2: {:X}", num);
    let (num, data) = VarInt::decode(data);
    println!("num3: {:X}", num);
    let (num, data) = VarInt::decode(data);
    println!("num4: {:X}", num);
    let (num, data) = VarInt::decode(data);
    println!("num5: {:X}", num);
    let (num, data) = VarInt::decode(data);
    println!("num6: {:X}", num);
    let (num, data) = VarInt::decode(data);
    println!("num7: {:X}", num);
    let (num, data) = VarInt::decode(data);
    println!("num8: {:X}", num);
    let (num, data) = VarInt::decode(data);
    println!("num9: {:X}", num);
    let (num, data) = VarInt::decode(data);
    println!("num10: {:X}", num);


    let (text, _) = StringMC::decode(data);
    println!("text: {}", text);




}
