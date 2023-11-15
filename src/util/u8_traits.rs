use std::{u16::MAX as MAXu16, u8::MAX};
pub trait Bit {
    fn set_bit(&mut self, bit: u8, value: bool);
    fn get_bit(&self, bit: u8) -> bool;
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
}
pub trait NibblesU16 {
    fn low_nibble(self) -> u8;
    fn high_nibble(self) -> u8;
}
impl NibblesU16 for u16 {
    fn low_nibble(self) -> u8 {
        (self & 0x00FF) as u8
    }
    fn high_nibble(self) -> u8 {
        ((self & 0xFF00) >> 8) as u8
    }
}
pub trait NibblesU8 {
    fn low_nibble(self) -> Self;
    fn high_nibble(self) -> Self;
}
impl NibblesU8 for u8 {
    fn low_nibble(self) -> Self {
        self & 0b0000_1111
    }
    fn high_nibble(self) -> Self {
        self & 0b1111_0000
    }
}
