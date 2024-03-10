#![allow(dead_code)]
mod mc_datatypes;

#[allow(unused_imports)]
use mc_datatypes::*;

fn main() { 

    let f1: f32 = 6.9;
    println!("{:X}", f1.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_encode() {
        assert_eq!(0x00, Bool::encode(false));
        assert_eq!(0x01, Bool::encode(true));
    }

    #[test]
    fn bool_decode() {
        assert_eq!(false, Bool::decode(0x00));
        assert_eq!(true, Bool::decode(0x01));
    }

    #[test]
    fn byte_encode() {
        assert_eq!(0b00001010 as u8, Byte::encode(10));
        assert_eq!(0b11110110 as u8, Byte::encode(-10));
    }

    #[test]
    fn byte_decode() {
        assert_eq!(10, Byte::decode(0b00001010));
        assert_eq!(-10, Byte::decode(0b11110110));
    }

    #[test]
    fn ubyte_encode() {
        assert_eq!(0b00001010 as u8, UByte::encode(10));
    }

    #[test]
    fn ubyte_decode() {
        assert_eq!(10, UByte::decode(0b00001010));
    }

    #[test]
    fn short_encode() {
        assert_eq!([0x0F, 0xFF], Short::encode(4095));
        assert_eq!([0xFF, 0xFF], Short::encode(-1));
    }

    #[test]
    fn short_decode() {
        assert_eq!(4095, Short::decode([0x0F, 0xFF]));
        assert_eq!(-1, Short::decode([0xFF, 0xFF]));
    }

    #[test]
    fn ushort_encode() {
        assert_eq!([0xFF, 0xFF], UShort::encode(65535));
    }

    #[test]
    fn ushort_decode() {
        assert_eq!(65535, UShort::decode([0xFF, 0xFF]));
    }

    #[test]
    fn int_encode() {
        assert_eq!([0x0F, 0xFF, 0xFF, 0xFF], Int::encode(268435455));
        assert_eq!([0xFF, 0xFF, 0xFF, 0xFF], Int::encode(-1));
    }

    #[test]
    fn int_decode() {
        assert_eq!(4095, Int::decode([0x0F, 0xFF, 0xFF, 0xFF]));
        assert_eq!(-1, Int::decode([0xFF, 0xFF, 0xFF, 0xFF]));
    }

    #[test]
    fn long_encode() {
        assert_eq!([0x0F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], Long::encode(1152921504606846975));
        assert_eq!([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], Long::encode(-1));
    }

    #[test]
    fn long_decode() {
        assert_eq!(1152921504606846975, Long::decode([0x0F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]));
        assert_eq!(-1, Long::decode([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]));
    }

    #[test]
    fn float_encode() {
        assert_eq!([0x42, 0x8A, 0x00, 0x00], Float::encode(6.9 as f32));
    }

    #[test]
    fn float_decode() {
        assert_eq!(6.9 as f32, Float::decode([0x42, 0x8A, 0x00, 0x00]));
    }

    #[test]
    fn double_encode() {
        assert_eq!([0x40, 0x1B, 0x99, 0x99, 0x99, 0x99, 0x99, 0x9A], Double::encode(6.9 as f64));
    }

    #[test]
    fn double_decode() {
        assert_eq!(6.9 as f64, Double::decode([0x40, 0x1B, 0x99, 0x99, 0x99, 0x99, 0x99, 0x9A]));
    }

}
