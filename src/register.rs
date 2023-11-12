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
    pub fn flag_z(&self) -> bool {
        return get_bit(&self.af, 7);
    }
    pub fn flag_n(&self) -> bool {
        return get_bit(&self.af, 7);
    }
    pub fn flag_h(&self) -> bool {
        return get_bit(&self.af, 7);
    }
    pub fn flag_c(&self) -> bool {
        return get_bit(&self.af, 7);
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
    use crate::register::{get_bit, set_low, set_high};

    use super::Registers;
    #[test]
    fn set_bit_test() {
        let mut zero = 0b0000_0000_0000_0000;
        set_low(&mut zero, 0b0000_0001);
        assert_eq!(zero, 0b0000_0000_0000_0001);
        set_low(&mut zero, 0b0000_1000);
        assert_eq!(zero, 0b0000_0000_0000_1000);
        set_low(&mut zero, 0b1111_1111);
        assert_eq!(zero, 0b0000_0000_1111_1111);
        set_high(&mut zero,0b1111_1111);
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
}
