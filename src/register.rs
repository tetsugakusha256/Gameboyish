pub struct Register {
    /// Acculmulator & Flag
    /// F bit :
    /// 7 zero flag
    /// 6 Subtraction flag
    /// 5 Half Carry flag
    /// 4 Carry flag
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    /// Stackpointer
    sp: u16,
    /// Program counter/Pointer
    pc: u16,
}
impl Register {
    pub fn flag_z(&self) -> bool {
        return get_bit(&self.af,7);
    }
    pub fn flag_n(&self) -> bool {
        return get_bit(&self.af,7);
    }
    pub fn flag_h(&self) -> bool {
        return get_bit(&self.af,7);
    }
    pub fn flag_c(&self) -> bool {
        return get_bit(&self.af,7);
    }
    pub fn a(&self) -> u8 {
        return get_high(&self.af);
    }
    pub fn b(&self) -> u8 {
        return get_high(&self.bc);
    }
    pub fn d(&self) -> u8 {
        return get_high(&self.de);
    }
    pub fn h(&self) -> u8 {
        return get_high(&self.hl);
    }
    pub fn c(&self) -> u8 {
        return get_low(&self.bc);
    }
    pub fn e(&self) -> u8 {
        return get_low(&self.de);
    }
    pub fn l(&self) -> u8 {
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
fn get_low(bytes: &u16) -> u8 {
    return (bytes & 0xFF) as u8;
}
fn get_high(bytes: &u16) -> u8 {
    return (bytes >> 8 & 0xFF) as u8;
}

#[cfg(test)]
mod tests {
    use crate::register::get_bit;

    use super::Register;

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
        let reg = Register {
            af: 0b0000_0011_1010_0101,
            bc: 0x1128,
            de: 0x1223,
            hl: 0x3142,
            sp: 0x5125,
            pc: 0x1682,
        };
        assert_eq!(0b0000_0011, reg.a());
        assert_eq!(0x11, reg.b());
        assert_eq!(0x12, reg.d());
        assert_eq!(0x31, reg.h());
        assert_eq!(0x28, reg.c());
        assert_eq!(0x23, reg.e());
        assert_eq!(0x42, reg.l());
    }
}
