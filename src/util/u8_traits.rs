use std::{u16::MAX as MAXu16, u8::MAX};
pub trait Bit {
    fn set_bit(&mut self, bit: u8, value: bool);
    fn get_bit(&self, bit: u8) -> bool;
    fn to_bits_array(&self) -> [u8; 8];
}
impl Bit for u8 {
    fn get_bit(&self, bit: u8) -> bool {
        let mask = if bit > 7 { 0u8 } else { 1u8 << bit };
        return (self & mask) != 0;
    }
    fn set_bit(&mut self, bit: u8, value: bool) {
        let mask = if bit > 7 { 0u8 } else { 1u8 << bit };
        if value {
            *self = *self | mask
        } else {
            *self = *self & (mask ^ MAX)
        }
    }
    // element 0 = bit 7
    fn to_bits_array(&self) -> [u8; 8] {
        let mut bits = [0; 8];
        for i in 0..8 {
            bits[i] = (self >> (7 - i)) & 1;
        }
        bits
    }
}
pub trait NibblesU16 {
    fn low_8nibble(self) -> u8;
    fn low_4nibble(self) -> u8;
    fn high_8nibble(self) -> u8;
    fn low_12(self) -> u16;
}
impl NibblesU16 for u16 {
    fn low_8nibble(self) -> u8 {
        (self & 0x00FF) as u8
    }
    fn low_4nibble(self) -> u8 {
        (self & 0b0000_1111) as u8
    }
    fn high_8nibble(self) -> u8 {
        ((self & 0xFF00) >> 8) as u8
    }
    fn low_12(self) -> u16 {
        self & 0x0FFF
    }
}
pub trait NibblesU8 {
    fn low_4nibble(self) -> Self;
    fn high_nibble(self) -> Self;
}
impl NibblesU8 for u8 {
    fn low_4nibble(self) -> Self {
        self & 0b0000_1111
    }
    fn high_nibble(self) -> Self {
        (self & 0b1111_0000) >> 4
    }
}

#[cfg(test)]
mod tests {
    use crate::util::u8_traits::Bit;


    #[test]
    fn get_set_bit_test(){
        let mut byte = 0b0000_0000u8;
        byte.set_bit(0,true);
        assert_eq!(byte, 0b0000_0001);
        byte.set_bit(1,true);
        assert_eq!(byte, 0b0000_0011);
        assert_eq!(true, byte.get_bit(1));
        assert_eq!(true, byte.get_bit(0));
        assert_eq!(false, byte.get_bit(2));
    }
}
