// Types for the Minecraft Server Protocol based on https://wiki.vg/Protocol#Data_types
// Authors: Foster Sullivan (SirGoatsalot), Caden Siva (SivaCaden)
// Created: 3/4/2024

// True is encoded as 0x01, false as 0x00.
use log::info;

pub struct Bool;

impl Bool {
    pub fn encode(value: bool) -> u8 {
        match value {
            true => 0x01,
            false => 0x00
        }
    }

    pub fn decode(value: u8) -> bool {
         match value {
            0x01 => true,
            0x00 => false,
            _ => panic!("Bool must be encoded as 0x00 or 0x01, got {}", value)
         }
    }
}

// Signed 8-bit integer, two's complement.
pub struct Byte;

impl Byte {
    pub fn encode(value: i8) -> u8 { 
        if value >= 0 { value.try_into().unwrap() }
        else { 
            let output: u8 = (value * -1).try_into().unwrap();
            !output + 1 
        }
    }

    pub fn decode(value: u8) -> i8 { value as i8 }
}

// Unsigned 8-bit integer.
pub struct UByte;

impl UByte {
    pub fn encode(value: u8) -> u8 { value }

    pub fn decode(value: u8) -> u8 { value }
}

// Signed 16-bit integer, two's complement.
pub struct Short;

impl Short {
    pub fn encode(value: i16) -> [u8; 2] { 
        // let mut unsplit_value = value;
        // let first_bit = (value & 0xFF00 >> 16) as u8;
        // let second_bit = (value & 0x00FF) as u8;
        // [ first_bit, second_bit ]
        [0, 0]
    }

    pub fn decode(value: [u8; 2]) -> i16 { 0 }
}

// Unsigned 16-bit integer.
pub struct UShort;

impl UShort {
    pub fn encode(value: u16) -> [u8; 2] { 
        [0, 0]
    }

    pub fn decode(value: [u8; 2]) -> u16 { 0 }
}

// Signed 32-bit Integer, two's complement.
pub struct Int;

impl Int {
    pub fn encode(value: i32) -> [u8; 4] { [0, 0, 0, 0] }

    pub fn decode(value: [u8; 4]) -> i32 { 0 }
}

// Signed 64-bit integer, two's complement.

pub struct Long;

impl Long {
    pub fn encode(value: i64) -> [u8; 8] { [0, 0, 0, 0, 0, 0, 0, 0] }

    pub fn decode(value: [u8; 8]) -> i64 { 0 }
}

// Single-Precision 32-bit floating point number.
pub struct Float;

impl Float {
    pub fn encode(value: f32) -> [u8; 4] { [0, 0, 0, 0] }

    pub fn decode(value: [u8; 4]) -> f32 { 0.0 as f32 }
}

// Double-Precision 64-bit floating point number.
pub struct Double;

impl Double {
    pub fn encode(value: f64) -> [u8; 8] { [0, 0, 0, 0, 0, 0, 0, 0] }

    pub fn decode(value: [u8; 8]) -> f64 { 0.0 as f64 }
}

// A sequence of Unicode scalar values. See https://wiki.vg/Protocol#Type:String
pub struct StringMC { }

impl StringMC {
    pub fn encode(value: String) -> Vec<u8> { vec![0] }

    pub fn decode(data: Vec<u8>) -> ( String, Vec<u8> ) {
        // A String of UTF-8 Characters, prefixed with its size in bytes as a VarInt.
        let mut length_buffer: Vec<u8> = Vec::new();
        let length: i32;

        // Step 1: Find the VarInt
        let ( length, data ) = VarInt::decode(data);

        // Step 2: Interpret those as a UTF-8 String
        ( String::from_utf8(data[..length as usize].to_vec()).unwrap(), data[length as usize..].to_vec() ) 
     } 
}

// Varialbe-length data encoding a two's complement 32-bit integer. See https://wiki.vg/Protocol#VarInt_and_VarLong
pub struct VarInt { }

impl VarInt {
    pub fn encode(value: i32, mut packet: Vec<u8>) -> Vec<u8> {
        // 129 ->  [0b10000001, 0b00000001]
        let segment_bits = 0x00;
        let current_byte_out = 0;
        let mut result: Vec<u8> = vec![];
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        let bytes = value.to_be_bytes();
        for byte in bytes {
            println!("BYTE INP 0b{:08b}", byte)
        }
        packet
    } 

    pub fn decode(bytes: Vec<u8>) -> (i32, Vec<u8>) {
        // FORMAT: 0b00000001 -> 1
        // 0b10000001, 0b00000001 -> 129
        let data_bits = 0b01111111;
        let signal_bit = 0b10000000;
        let mut bytes_clone = bytes.clone();
        let mut position = 0;
        let mut result: i32 = 0;
        for byte in bytes.iter() {
            info!("BYTE INP 0b{:08b}", byte);
            result |= ((byte & data_bits) << position) as i32;
            bytes_clone.remove(0);
            if (byte & signal_bit) == signal_bit { break }
            position += 7;  
            if position >= 32 { panic!("VarInt is too big!") }                      
        }
        (result, bytes_clone)
    }

}

