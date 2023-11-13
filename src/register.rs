use std::u16::MAX as MAXu16;

struct Register(u16);
pub struct Registers {
    /// Acculmulator & Flag
    /// F bit :
    /// 7 zero flag
    /// 6 Subtraction flag
    /// 5 Half Carry flag
    /// 4 Carry flag
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    /// Stackpointer
    pub sp: u16,
    /// Program counter/Pointer
    pub pc: u16,
}
impl Registers {
    pub fn new() -> Registers {
        Registers {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
        }
    }
    pub fn set_flags(&mut self, z: bool, n: bool, h: bool, c: bool) {
        self.set_flag_z(z);
        self.set_flag_n(n);
        self.set_flag_h(h);
        self.set_flag_c(c);
    }
    pub fn set_flag_z(&mut self, value: bool) {
        set_bit(&mut self.af, 7, value);
    }
    pub fn set_flag_n(&mut self, value: bool) {
        set_bit(&mut self.af, 6, value);
    }
    pub fn set_flag_h(&mut self, value: bool) {
        set_bit(&mut self.af, 5, value);
    }
    pub fn set_flag_c(&mut self, value: bool) {
        set_bit(&mut self.af, 4, value);
    }

    pub fn flag_z(&self) -> bool {
        return get_bit(&self.af, 7);
    }
    pub fn flag_n(&self) -> bool {
        return get_bit(&self.af, 6);
    }
    pub fn flag_h(&self) -> bool {
        return get_bit(&self.af, 5);
    }
    pub fn flag_c(&self) -> bool {
        return get_bit(&self.af, 4);
    }

    pub fn set_a(&mut self, val: u8) {
        set_high(&mut self.af, val);
    }
    pub fn set_b(&mut self, val: u8) {
        set_high(&mut self.bc, val);
    }
    pub fn set_d(&mut self, val: u8) {
        set_high(&mut self.de, val);
    }
    pub fn set_h(&mut self, val: u8) {
        set_high(&mut self.hl, val);
    }
    pub fn set_c(&mut self, val: u8) {
        set_low(&mut self.bc, val);
    }
    pub fn set_e(&mut self, val: u8) {
        set_low(&mut self.de, val);
    }
    pub fn set_l(&mut self, val: u8) {
        set_low(&mut self.hl, val);
    }

    pub fn get_a(&self) -> u8 {
        return get_high(&self.af);
    }
    pub fn get_b(&self) -> u8 {
        return get_high(&self.bc);
    }
    pub fn get_d(&self) -> u8 {
        return get_high(&self.de);
    }
    pub fn get_h(&self) -> u8 {
        return get_high(&self.hl);
    }
    pub fn get_c(&self) -> u8 {
        return get_low(&self.bc);
    }
    pub fn get_e(&self) -> u8 {
        return get_low(&self.de);
    }
    pub fn get_l(&self) -> u8 {
        return get_low(&self.hl);
    }
    /// return pc + 0x0001 the address of the byte after the pointer pc
    pub fn get_pc_next(&self) -> u16 {
        return self.pc + 0x0001;
    }
    /// TODO: check that wrapping is what should happen
    pub fn hl_plus(&mut self) {
        self.hl = self.hl.wrapping_add(1);
    }
    /// TODO: check that wrapping is what should happen
    pub fn hl_minus(&mut self) {
        self.hl = self.hl.wrapping_sub(1);
    }
}
fn get_bit(bytes: &u16, bit: u8) -> bool {
    let mask = match bit {
        0 => 1,
        1 => 2,
        2 => 4,
        3 => 8,
        4 => 16,
        5 => 32,
        6 => 64,
        7 => 128,
        _ => 0,
    };
    return (bytes & mask as u16) != 0;
}
fn set_bit(bytes: &mut u16, bit: u8, value: bool) {
    let mask = match bit {
        0 => 1,
        1 => 2,
        2 => 4,
        3 => 8,
        4 => 16,
        5 => 32,
        6 => 64,
        7 => 128,
        _ => 0,
    };
    if value {
        *bytes = *bytes | mask;
    } else {
        *bytes = *bytes & (mask ^ MAXu16)
    }
}
fn set_low(bytes: &mut u16, byte: u8) {
    *bytes = (*bytes & 0xFF00) | (byte as u16);
}
fn set_high(bytes: &mut u16, byte: u8) {
    *bytes = (*bytes & 0x00FF) | ((byte as u16) << 8);
}
fn get_low(bytes: &u16) -> u8 {
    return (bytes & 0xFF) as u8;
}
fn get_high(bytes: &u16) -> u8 {
    return (bytes >> 8 & 0xFF) as u8;
}

#[cfg(test)]
mod tests {
    use crate::register::{get_bit, set_bit, set_high, set_low};

    use super::Registers;
    #[test]
    fn set_low_high_test() {
        let mut zero = 0b0000_0000_0000_0000;
        set_low(&mut zero, 0b0000_0001);
        assert_eq!(zero, 0b0000_0000_0000_0001);
        set_low(&mut zero, 0b0000_1000);
        assert_eq!(zero, 0b0000_0000_0000_1000);
        set_low(&mut zero, 0b1111_1111);
        assert_eq!(zero, 0b0000_0000_1111_1111);
        set_high(&mut zero, 0b1111_1111);
        assert_eq!(zero, 0b1111_1111_1111_1111);
        set_high(&mut zero, 0b1101_1111);
        assert_eq!(zero, 0b1101_1111_1111_1111);
        set_low(&mut zero, 0b0000_1000);
        assert_eq!(zero, 0b1101_1111_0000_1000);
    }
    #[test]
    fn get_bit_test() {
        let bytes = 0b0110_0000_1010_1001;
        assert!(get_bit(&bytes, 0));
        assert!(!get_bit(&bytes, 1));
        assert!(!get_bit(&bytes, 2));
        assert!(get_bit(&bytes, 3));
        assert!(!get_bit(&bytes, 4));
        assert!(get_bit(&bytes, 5));
        assert!(!get_bit(&bytes, 6));
        assert!(get_bit(&bytes, 7));
        // assert!(!get_bit(&bytes, 0b0001_0001));
        // assert!(!get_bit(&bytes, 0b0000_0100));
    }
    #[test]
    fn high_low_test() {
        let reg = Registers {
            af: 0b0000_0011_1010_0101,
            bc: 0x1128,
            de: 0x1223,
            hl: 0x3142,
            sp: 0x5125,
            pc: 0x1682,
        };
        assert_eq!(0b0000_0011, reg.get_a());
        assert_eq!(0x11, reg.get_b());
        assert_eq!(0x12, reg.get_d());
        assert_eq!(0x31, reg.get_h());
        assert_eq!(0x28, reg.get_c());
        assert_eq!(0x23, reg.get_e());
        assert_eq!(0x42, reg.get_l());
    }
    #[test]
    fn set_bit_test() {
        let mut a = 0b0000_0011_1010_0101u16;
        set_bit(&mut a, 1, true);
        assert_eq!(0b0000_0011_1010_0111, a);
        set_bit(&mut a, 7, true);
        assert_eq!(0b0000_0011_1010_0111, a);
        set_bit(&mut a, 1, false);
        assert_eq!(0b0000_0011_1010_0101, a);
        set_bit(&mut a, 1, true);
        assert_eq!(0b0000_0011_1010_0111, a);
    }
    #[test]
    fn flag_test() {
        //flag : bit 7,6,5,4 = z,n,h,c
        let mut reg = Registers {
            af: 0b0000_0011_1010_0101,
            bc: 0x1128,
            de: 0x1223,
            hl: 0x3142,
            sp: 0x5125,
            pc: 0x1682,
        };
        assert_eq!(reg.flag_z(), true);
        assert_eq!(reg.flag_n(), false);
        assert_eq!(reg.flag_h(), true);
        assert_eq!(reg.flag_c(), false);
        reg.set_flag_z(false);
        reg.set_flag_n(false);
        reg.set_flag_h(true);
        reg.set_flag_c(true);
        assert_eq!(reg.flag_z(), false);
        assert_eq!(reg.flag_n(), false);
        assert_eq!(reg.flag_h(), true);
        assert_eq!(reg.flag_c(), true);
        reg.set_flags(true, true, true, false);
        assert_eq!(reg.flag_z(), true);
        assert_eq!(reg.flag_n(), true);
        assert_eq!(reg.flag_h(), true);
        assert_eq!(reg.flag_c(), false);
    }
}
