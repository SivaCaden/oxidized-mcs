// Types for the Minecraft Server Protocol based on https://wiki.vg/Protocol#Data_types
// Authors: Foster Sullivan (SirGoatsalot), Caden Siva (SivaCaden)
// Created: 3/4/2024

// True is encoded as 0x01, false as 0x00.
pub struct Bool { }

impl Bool {
    pub fn encode(value: bool) -> i8 {
        match value {
            true => 0x01,
            false => 0x00
        }
    }

    pub fn decode(value: i8) -> bool {
         match value {
            0x01 => true,
            0x00 => false
         }
    }
}

// Signed 8-bit integer, two's complement.
pub struct Byte { }

impl Byte {
    pub fn encode(value: i8) -> i8 { value }

    pub fn decode(value: i8) -> i8 { value }
}

// Unsigned 8-bit integer.
pub struct UByte { }

impl UByte {
    pub fn encode(value: u8) -> u8 { value }

    pub fn decode(value: u8) -> u8 { value }
}

// Signed 16-bit integer, two's complement.
pub struct Short { }

impl Short {
    pub fn encode(value: i16) -> i16 { value }

    pub fn decode(value: i16) -> i16 { value }
}

// Unsigned 16-bit integer.
pub struct UShort { }

impl UShort {
    pub fn encode(value: u16) -> u16 { value }

    pub fn decode(value: u16) -> u16 { value }
}

// Signed 32-bit Integer, two's complement.
pub struct Int { }

impl Int {
    pub fn encode(value: i32) -> i32 { value }

    pub fn decode(value: i32) -> i32 { value }
}

// Signed 64-bit integer, two's complement.
pub struct Long { }

impl Long {
    pub fn encode(value: i64) -> i64 { value }

    pub fn decode(value: i64) -> i64 { value }
}

// Single-Precision 32-bit floating point number.
pub struct Float { }

impl Float {
    pub fn encode(value: f32) -> f32 { value }

    pub fn decode(value: f32) -> f32 { value }
}

// Double-Precision 64-bit floating point number.
pub struct Double { }

impl Double {
    pub fn encode(value: f64) -> f64 { value }

    pub fn decode(value: f64) -> f64 { value }
}


