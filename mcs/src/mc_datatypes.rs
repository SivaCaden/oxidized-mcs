// Types for the Minecraft Server Protocol based on https://wiki.vg/Protocol#Data_types
// Authors: Foster Sullivan (SirGoatsalot), Caden Siva (SivaCaden)
// Created: 3/4/2024

use log::info;

pub struct Bool;

impl Bool {
    pub fn encode(value: bool, packet: Vec<u8>) -> Vec<u8> {
        let mut new_packet = packet.clone();
        new_packet.push(match value {
            true => 0x01,
            false => 0x00
        });
        new_packet
    }

    pub fn decode(packet: Vec<u8>) -> (bool, Vec<u8>) {
         let mut new_packet = packet.clone();
         let result = match new_packet.remove(0) {
            0x01 => true,
            0x00 => false,
            _ => panic!("Bool must be encoded as 0x00 or 0x01, got 0x{:02X?}", packet[0])
         };
         ( result, new_packet ) 
    }
}

// Signed 8-bit integer, two's complement.
pub struct Byte;

impl Byte {
    pub fn encode(value: i8, packet: Vec<u8>) -> Vec<u8> { 
        let mut new_packet = packet.clone();
        new_packet.push( 
            if value >= 0 { value.try_into().unwrap() }
            else { 
                let output: u8 = (value * -1).try_into().unwrap();
                !output + 1 
        });
        new_packet
    }

    pub fn decode(packet: Vec<u8>) -> (i8, Vec<u8>) { 
        let mut out = packet.clone();
        out.remove(0);
        ( packet[0] as i8, out )
    }
}

// Unsigned 8-bit integer.
pub struct UByte;

impl UByte {
    pub fn encode(value: u8, packet: Vec<u8>) -> Vec<u8> {
        let mut new_packet = packet.clone();
        new_packet.push(value);
        new_packet
    }

    pub fn decode(packet: Vec<u8>) -> (u8, Vec<u8>) {
        let mut out = packet.clone();
        ( out.remove(0), out )
    }
}

// Signed 16-bit integer, two's complement.
pub struct Short;

impl Short {
    pub fn encode(value: i16, packet: Vec<u8>) -> Vec<u8> { 
        let bytes = value.to_be_bytes();
        let mut new_packet = packet.clone();
        for byte in bytes { new_packet.push(byte) }
        new_packet
    }

    pub fn decode(packet: Vec<u8>) -> (i16, Vec<u8>) { 
        let mut new_packet = packet.clone();
        let first_byte = (new_packet.remove(0) as u16) << 8;
        let second_byte = new_packet.remove(0) as u16;
        ( (first_byte | second_byte) as i16, new_packet )
    }
}

// Unsigned 16-bit integer.
pub struct UShort;

impl UShort {
    pub fn encode(value: u16, packet: Vec<u8>) -> Vec<u8> { 
        let bytes = value.to_be_bytes();
        let mut new_packet = packet.clone();
        for byte in bytes { new_packet.push(byte) }
        new_packet           
    }

    pub fn decode(packet: Vec<u8>) -> (u16, Vec<u8>) { 
        let mut new_packet = packet.clone();
        let first_byte = (new_packet.remove(0) as u16) << 8;
        let second_byte = new_packet.remove(0) as u16;
        ( first_byte | second_byte, new_packet )
    }
}

// Signed 32-bit Integer, two's complement.
pub struct Int;

impl Int {
    pub fn encode(value: i32, packet: Vec<u8>) -> Vec<u8> { 
        let mut new_packet = packet.clone();
        let bytes = value.to_be_bytes();
        for byte in bytes { new_packet.push(byte) }
        new_packet
    }

    pub fn decode(packet: Vec<u8>) -> (i32, Vec<u8>) {
        let mut out = packet.clone();
        let mut value: i32 = 0;
        let mut shift: i32 = 0;
        for byte in out.drain(0..4) { 
            value |= byte as i32;
            value = value << shift;
            if shift == 0 { shift = 8 }
        }
        ( value, out )
    }
}

// Signed 64-bit integer, two's complement.

pub struct Long;

impl Long {
    pub fn encode(value: i64, packet: Vec<u8>) -> Vec<u8> { 
        let mut new_packet = packet.clone();
        let bytes = value.to_be_bytes();
        for byte in bytes { new_packet.push(byte) }
        new_packet
    }

    pub fn decode(packet: Vec<u8>) -> (i64, Vec<u8>) { 
        let mut out = packet.clone();
        let mut value: i64 = 0;
        let mut position: i64 = 56;
        for byte in out.drain(0..8) { 
            value |= (byte as i64) << position;
            position -= 8;
        }
        ( value, out )
    }

}

// Single-Precision 32-bit floating point number.
pub struct Float;

impl Float {
    pub fn encode(value: f32, packet: Vec<u8>) -> Vec<u8> { 
        let mut new_packet = packet.clone();
        new_packet.append(&mut value.to_be_bytes().to_vec());
        new_packet
    }

    pub fn decode(packet: Vec<u8>) -> (f32, Vec<u8>) { 
        let mut out = packet.clone();
        let _ = out.drain(0..4);
        let val_bytes = &packet[0..4];
        let mut val = f32::from_be_bytes(val_bytes.try_into().unwrap());
        ( val, out )
    }
}

// Double-Precision 64-bit floating point number.
pub struct Double;

impl Double {
    pub fn encode(value: f64, packet: Vec<u8>) -> Vec<u8> { 
        let mut new_packet = packet.clone();
        new_packet.append(&mut value.to_be_bytes().to_vec());
        new_packet
    }

    pub fn decode(packet: Vec<u8>) -> (f64, Vec<u8>) { 
        let mut out = packet.clone();
        let _ = out.drain(0..4);
        let val_bytes = &packet[0..8];
        let mut val = f64::from_be_bytes(val_bytes.try_into().unwrap());
        ( val, out )
    }
}

// A sequence of Unicode scalar values preceded by its length. See https://wiki.vg/Protocol#Type:String
pub struct StringMC; 

impl StringMC {
    pub fn encode(value: String, packet: &mut Vec<u8>) { 
        // Step 1: Convert length of String into VarInt and make that the first thing
        let mut bytes_out: Vec<u8> = vec![];
        VarInt::encode(value.len() as i32, &mut bytes_out); 

        // Step 2: Append utf-8 bytes to out bytes
        let string_bytes = value.into_bytes();
        for byte in string_bytes { bytes_out.push(byte) }
        for byte in bytes_out { packet.push(byte) }
    }

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
pub struct VarInt; 

impl VarInt {
    pub fn encode(value: i32, packet: &mut Vec<u8>) {
        // Step 1: Split value into bytes & append continue bits to the front
        let segment_bits = 0b01111111;
        let continue_bit = 0b10000000;

        let mut raw_bytes = value.to_be_bytes();
        let mut raw_value: u32 = u32::from_be_bytes(raw_bytes);
        let mut out_bytes: Vec<u8> = vec![];

        loop { 
            if (raw_value & !segment_bits) == 0 { 
                out_bytes.push(raw_value.try_into().unwrap());
                break;
            }
            out_bytes.push( ((raw_value & segment_bits) | continue_bit).try_into().unwrap() );
            raw_value = raw_value >> 7;
        } 
        
        // Step 2: Add those bytes to the end of the packet 
        for byte in out_bytes { packet.push(byte) }
    } 

    pub fn decode(packet: Vec<u8>) -> (i32, Vec<u8>) {
        // Step 1: Separate which bytes are in the varint
        let signal_bit = 0b10000000; 
        let mut packet_out = packet.clone();
        let mut buf: Vec<u8> = vec![];
        for byte in packet { 
            buf.push(byte);
            packet_out.remove(0);
            if (byte & signal_bit) != signal_bit { break } // END OF NUMBER 
        }

        // Step 2: Decode the VarInt from the buffer
        let data_bits = 0b01111111;
        let mut value: i32 = 0;
        let mut position = 0;
        for byte in buf {
            value |= ((byte & data_bits) as i32) << position;
            position += 7;
        }

        (value, packet_out)
    }

}

