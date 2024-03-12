// Types for the Minecraft Server Protocol based on https://wiki.vg/Protocol#Data_types
// Authors: Foster Sullivan (SirGoatsalot), Caden Siva (SivaCaden)
// Created: 3/4/2024

// True is encoded as 0x01, false as 0x00.

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
        let mut unsplit_value = value;
        let first_bit = (value & 0xFF00 >> 16) as u8;
        let second_bit = (value & 0x00FF) as u8;
        [ first_bit, second_bit ]
    }

    pub fn decode(value: [u8; 2]) -> i16 { 0 }
}

// Unsigned 16-bit integer.
pub struct UShort;

impl UShort {
    pub fn encode(value: u16) -> [u8; 2] { 
        
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

