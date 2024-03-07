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
#[allow(dead_code)]
pub struct Byte;

#[allow(dead_code)]
impl Byte {
    pub fn encode(value: i8) -> u8 { 
        if value >= 0 { value.try_into().unwrap() }
        else { 
            let output: u8 = (value * -1).try_into().unwrap();
            !output + 1 
        }
    }

    pub fn decode(_value: u8) -> i8 { 0 }
}

// Unsigned 8-bit integer.
#[allow(dead_code)]
pub struct UByte;

#[allow(dead_code)]
impl UByte {
    pub fn encode(value: u8) -> u8 { value }

    pub fn decode(value: u8) -> u8 { value }
}

// Signed 16-bit integer, two's complement.
#[allow(dead_code)]
pub struct Short;

#[allow(dead_code)]
impl Short {
    pub fn encode(value: i16) -> [u8; 2] { [0, 0] }

    pub fn decode(value: [u8; 2]) -> i16 { 0 }
}

// Unsigned 16-bit integer.
#[allow(dead_code)]
pub struct UShort;

#[allow(dead_code)]
impl UShort {
    pub fn encode(value: u16) -> [u8; 2] { [0, 0] }

    pub fn decode(value: [u8; 2]) -> u16 { 0 }
}

// Signed 32-bit Integer, two's complement.
#[allow(dead_code)]
pub struct Int;

#[allow(dead_code)]
impl Int {
    pub fn encode(value: i32) -> i32 { value }

    pub fn decode(value: i32) -> i32 { value }
}

// Signed 64-bit integer, two's complement.

pub struct Long;

impl Long {
    pub fn encode(value: i64) -> i64 { value }

    pub fn decode(value: i64) -> i64 { value }
}

// Single-Precision 32-bit floating point number.
pub struct Float;

impl Float {
    pub fn encode(value: f32) -> f32 { value }

    pub fn decode(value: f32) -> f32 { value }
}

// Double-Precision 64-bit floating point number.
pub struct Double;

impl Double {
    pub fn encode(value: f64) -> f64 { value }

    pub fn decode(value: f64) -> f64 { value }
}

