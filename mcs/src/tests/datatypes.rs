// Testing file to test utils/datatypes.rs
// Authored by: Three rats in a trench coat

#![cfg(test)]

use crate::util::datatypes::*;

#[test]
fn bool_thru_double() {
    // DECODE TEST
    let packet = vec![0x01, 0x00,               // Bool true, false
                        0x81,                   // Byte -127
                        0xFF,                   // UByte 255
                        0x80, 0x01,             // Short -32767
                        0xFF, 0xFF,             // UShort 65535
                        0x80, 0x00, 0x00, 0x00, // Int -2147483648
                        0x7F, 0xFF, 0xFF, 0xFF,  
                        0xFF, 0xFF, 0xFF, 0xFF, // Long 9223372036854775807                                            
                        0x40, 0xDC, 0xCC, 0xCD, // Float 6.9 (nice)                        
                        0x40, 0x1B, 0x99, 0x99,
                        0x99, 0x99, 0x99, 0x9A, // Double 6.9 (also nice)
                        ];

    let (bool1, packet) = Bool::decode(packet);
    let (bool2, packet) = Bool::decode(packet);
    let (byte, packet) = Byte::decode(packet);
    let (ubyte, packet) = UByte::decode(packet);
    let (short, packet) = Short::decode(packet);
    let (ushort, packet) = UShort::decode(packet);
    let (int, packet) = Int::decode(packet);
    let (long, packet) = Long::decode(packet);
    let (float, packet) = Float::decode(packet);
    let (double, _packet) = Double::decode(packet);

    assert_eq!(bool1, true);
    assert_eq!(bool2, false);
    assert_eq!(byte, -127 as i8);
    assert_eq!(ubyte, 255 as u8);
    assert_eq!(short, -32767 as i16);
    assert_eq!(ushort, 65535 as u16);
    assert_eq!(int, -2147483648 as i32);
    assert_eq!(long, 9223372036854775807 as i64);
    assert_eq!(float, 6.9 as f32);
    assert_eq!(double, 6.9 as f64);
}


#[test]
fn varint() {
    // DECODE TESTS
    let packet = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x07,// Max Value of VarInt 2147483647
                        0xDD, 0xC7, 0x01,              // 25565, default port
                        0x01,                          // 1
                        0x80, 0x80, 0x80, 0x80, 0x08,  // Min Value of VarInt -2147483648
                        0x69, 0x69];                   // Other data at end of packet
    let (num1, packet) = VarInt::decode(packet); 
    let (num2, packet) = VarInt::decode(packet);
    let (num3, packet) = VarInt::decode(packet);
    let (num4, packet) = VarInt::decode(packet);

    assert_eq!(num1, 2147483647);
    assert_eq!(num2, 25565);
    assert_eq!(num3, 1);
    assert_eq!(num4, -2147483648);
    assert_eq!(packet, vec![0x69, 0x69]);

    // ENCODE TESTS
    let num1: i32 = 2147483647;
    let num2: i32 = 25565;
    let num3: i32 = 1;
    let num4: i32 = -2147483648;
    
    let out = VarInt::encode(num1, vec![]);
    assert_eq!(vec![0xFF, 0xFF, 0xFF, 0xFF, 0x07], out);
    let out = VarInt::encode(num2, out);
    assert_eq!(vec![0xFF, 0xFF, 0xFF, 0xFF, 0x07, 0xDD, 0xC7, 0x01], out);
    let out = VarInt::encode(num3, out);
    assert_eq!(vec![0xFF, 0xFF, 0xFF, 0xFF, 0x07, 0xDD, 0xC7, 0x01, 0x01], out);
    let out = VarInt::encode(num4, out);
    assert_eq!(vec![0xFF, 0xFF, 0xFF, 0xFF, 0x07, 0xDD, 0xC7, 0x01, 0x01, 0x80, 0x80, 0x80, 0x80, 0x08], out);

}

#[test]
fn varlong() {
    // DECODE TEST
    let packet = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x07, // 2147483647
                    0x80, 0x80, 0x80, 0x80, 0x80, 
                    0x80, 0x80, 0x80, 0x80, 0x01];  // -9223372036854775808

    let (varlong1, packet) = VarLong::decode(packet);
    let (varlong2, _packet) = VarLong::decode(packet);

    assert_eq!(varlong1, 2147483647);
    assert_eq!(varlong2, -9223372036854775808);

    // ENCODE TEST
    let varlong1 = 2147483647;
    let varlong2 = -9223372036854775808;

    let test_packet = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x07, // 2147483647
                    0x80, 0x80, 0x80, 0x80, 0x80, 
                    0x80, 0x80, 0x80, 0x80, 0x01];  // -9223372036854775808

    let mut packet = VarLong::encode(varlong1, vec![]);
    packet = VarLong::encode(varlong2, packet);

    assert_eq!(test_packet, packet);
}

#[test]
fn string_mc() {
    // DECODE TEST
    let packet = vec![0x28,                                          // VarInt length
                    0x54, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x6f, 0x72, 
                    0x20, 0x6e, 0x6f, 0x74, 0x20, 0x74, 0x6f, 0x20,
                    0x62, 0x65, 0x2c, 0x20, 0x54, 0x68, 0x61, 0x74,
                    0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
                    0x71, 0x75, 0x65, 0x73, 0x74, 0x69, 0x6f, 0x6e,  // To be or not to be, That is the question
                    0x69, 0x69, 0x69, 0x69, 0x69, 0x69, 0x69, 0x69]; // Misc. other data
    let my_nuts = vec![ 0x07, 0x41, 0x4b, 0x41, 0x5f, 0x64, 0x65, 0x6e ];


   let (string, packet) = StringMC::decode(packet);
   assert_eq!(string, String::from("To be or not to be, That is the question"));
   assert_eq!(packet, vec![0x69, 0x69, 0x69, 0x69, 0x69, 0x69, 0x69, 0x69]);
   assert_eq!(my_nuts, StringMC::encode(String::from("AKA_den"), vec![])); 

   // ENCODE TEST
   let test_packet = vec![0x69, 0x69, 0x69, 0x69, 0x69, 0x69, 0x69, 0x69,// Some other data
                    0x28,                                                // VarInt length
                    0x54, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x6f, 0x72,      // String data 
                    0x20, 0x6e, 0x6f, 0x74, 0x20, 0x74, 0x6f, 0x20,
                    0x62, 0x65, 0x2c, 0x20, 0x54, 0x68, 0x61, 0x74,
                    0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
                    0x71, 0x75, 0x65, 0x73, 0x74, 0x69, 0x6f, 0x6e];
                                                                    
   let string = String::from("To be or not to be, That is the question"); 
   let out = StringMC::encode(string, vec![0x69, 0x69, 0x69, 0x69, 0x69, 0x69, 0x69, 0x69]);
   assert_eq!(test_packet, out); 
}

#[test]
fn identifier() {
    // DECODE TEST
    let packet = vec![0x28, 0x6D, 0x69, 0x6E, // minecraft:infested_chiseled_stone_bricks 
                0x65, 0x63, 0x72, 0x61, 
                0x66, 0x74, 0x3A, 0x69,
                0x6E, 0x66, 0x65, 0x73,
                0x74, 0x65, 0x64, 0x5F,
                0x63, 0x68, 0x69, 0x73,
                0x65, 0x6C, 0x65, 0x64,
                0x5F, 0x73, 0x74, 0x6F,
                0x6E, 0x65, 0x5F, 0x62,
                0x72, 0x69, 0x63, 0x6B, 
                0x73];

    let (identifier, _) = Identifier::decode(packet);

    assert_eq!(identifier, String::from("minecraft:infested_chiseled_stone_bricks"));

    // ENCODE TEST
    let identifier = String::from("minecraft:infested_chiseled_stone_bricks");
    let test_packet = vec![0x28, 0x6D, 0x69, 0x6E, // minecraft:infested_chiseled_stone_bricks 
                0x65, 0x63, 0x72, 0x61, 
                0x66, 0x74, 0x3A, 0x69,
                0x6E, 0x66, 0x65, 0x73,
                0x74, 0x65, 0x64, 0x5F,
                0x63, 0x68, 0x69, 0x73,
                0x65, 0x6C, 0x65, 0x64,
                0x5F, 0x73, 0x74, 0x6F,
                0x6E, 0x65, 0x5F, 0x62,
                0x72, 0x69, 0x63, 0x6B, 
                0x73];

    assert_eq!(test_packet, Identifier::encode(identifier, vec![]));
}

#[test]
fn position() { 
    // DECODE TEST
    // (18357644, -20882616, 831)
    let packet = vec![0x46, 0x07, 0x63, 0x2C, 0x15, 0xB4, 0x83, 0x3F];

    let (position, _packet) = Position::decode(packet);
    assert_eq!(position, (18357644, 831, -20882616));

    // ENCODE TEST
    let position = (18357644, 831, -20882616);
    let mut packet = vec![];

    packet = Position::encode(position, packet);
    assert_eq!(packet, vec![0x46, 0x07, 0x63, 0x2C, 0x15, 0xB4, 0x83, 0x3F]);
}

#[test]
fn angle() {
    // DECODE TEST
    let packet = vec![0x40, 0x80, 0xC0]; // 64/256, 128/256, 192/256 steps in a turn

    let (angle1, packet) = Angle::decode_256(packet);
    let (angle2, packet) = Angle::decode_256(packet);
    let (angle3, _) = Angle::decode_256(packet);

    assert_eq!(angle1, 64);
    assert_eq!(angle2, 128);
    assert_eq!(angle3, 192);

    let packet = vec![0x40, 0x80, 0xC0]; // 90, 180, 270 degrees

    let (angle1, packet) = Angle::decode_360(packet);
    let (angle2, packet) = Angle::decode_360(packet);
    let (angle3, _packet) = Angle::decode_360(packet);

    assert_eq!(angle1, 90);
    assert_eq!(angle2, 180);
    assert_eq!(angle3, 270);

    // ENCODE TEST
}

#[test]
fn uuid() {
    // DECODE TEST -- using UUID for Sir_Goatsalot (me :))
    let packet = vec![0x02, 0x75, 0x48, 0xC6, 
                        0xB4, 0xD3, 0x44, 0xEF,
                        0x8F, 0xDB, 0xC4, 0xAA,
                        0x0A, 0xE3, 0x72, 0x67];
    // Random UUID -- nice
    let packet_2 = vec![0x69, 0x69, 0x69, 0x69,
                        0x69, 0x69, 0x69, 0x69,
                        0x69, 0x69, 0x69, 0x69,
                        0x69, 0x69, 0x69, 0x69];

    // UUID for AKA_den -- 37b2eb48109340d7872469e5e1ef9574
    let packet_3 = vec![0x37, 0xB2, 0xEB, 0x48,
                        0x10, 0x93, 0x40, 0xD7,
                        0x87, 0x24, 0x69, 0xE5,
                        0xE1, 0xEF, 0x95, 0x74];

    let (uuid, _packet) = Uuid::decode(packet);
    let (uuid2, _packet) = Uuid::decode(packet_2);
    let (uuid3, _packet) = Uuid::decode(packet_3);

    assert_eq!(uuid, 0x027548c6b4d344ef8fdbc4aa0ae37267);
    assert_eq!(uuid2, 0x69696969696969696969696969696969);
    assert_eq!(uuid3, 0x37b2eb48109340d7872469e5e1ef9574);

    // ENCODE TEST
    let uuid: u128 = 0x027548c6b4d344ef8fdbc4aa0ae37267;
    let uuid2: u128 = 0x69696969696969696969696969696969;
    let uuid3: u128 = 0x37b2eb48109340d7872469e5e1ef9574;

    // UUID for Sir_Goatsalot -- 0x027548c6b4d344ef8fdbc4aa0ae37267
    let packet = vec![0x02, 0x75, 0x48, 0xC6, 
                        0xB4, 0xD3, 0x44, 0xEF,
                        0x8F, 0xDB, 0xC4, 0xAA,
                        0x0A, 0xE3, 0x72, 0x67];
    // Random UUID -- nice
    let packet_2 = vec![0x69, 0x69, 0x69, 0x69,
                        0x69, 0x69, 0x69, 0x69,
                        0x69, 0x69, 0x69, 0x69,
                        0x69, 0x69, 0x69, 0x69];

    // UUID for AKA_den -- 0x37b2eb48109340d7872469e5e1ef9574
    let packet_3 = vec![0x37, 0xB2, 0xEB, 0x48,
                        0x10, 0x93, 0x40, 0xD7,
                        0x87, 0x24, 0x69, 0xE5,
                        0xE1, 0xEF, 0x95, 0x74];

    assert_eq!(packet, Uuid::encode(uuid, vec![]));
    assert_eq!(packet_2, Uuid::encode(uuid2, vec![]));
    assert_eq!(packet_3, Uuid::encode(uuid3, vec![]));
}
