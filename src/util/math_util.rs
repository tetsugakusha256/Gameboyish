trait Nibbles {
    fn low_nibble(self) -> Self;
    fn high_nibble(self) -> Self;
}

impl Nibbles for u8 {
    fn low_nibble(self) -> Self {
        self & 0b0000_1111
    }
    fn high_nibble(self) -> Self {
        self & 0b1111_0000
    }
}
/// TODO: Check what should happen if a + b is neg
/// TODO: are signed number ones complement or sign magnitude?
/// Add b interpreted as a signed int into a
/// if the bool is true => overflow occured
pub fn signed_addition(a: u16, b: u8) -> (u16, bool) {
    let signed_b: i32 = b as i8 as i32;
    let signed_a: i32 = a as i32;
    let result = signed_a + signed_b;

    let overflow = result < 0 || result > 0xFFFF;
    // Perform signed addition
    let result = signed_a + signed_b;
    println!("sa:{},  sb:{}", signed_a, signed_b);

    (result as u16, overflow)
}
/// return (result, zero, halfcarry, carry)
pub fn addition(a: u8, b: u8) -> (u8, bool, bool, bool) {
    let (result, carry) = a.overflowing_add(b);
    let halfcarry = (a.low_nibble() + b.low_nibble()) > 15;
    (result, result == 0, halfcarry, carry)
}
/// return (result, halfcarry, carry)
pub fn adc(a: u8, b: u8, c: bool) -> (u8, bool, bool, bool) {
    // First adding the carry on and check if it causes overflow or halfcarry
    let mut b = b;
    let mut half_carry2 = false;
    let mut carry2 = false;
    if c {
        (b, _, half_carry2, carry2) = addition(b, 1);
    }
    let (result, z, halfcarry, carry) = addition(a, b);
    (result, z, halfcarry | half_carry2, carry | carry2)
}
/// return (result, halfcarry, carry)
pub fn sbc(a: u8, b: u8, c: bool) -> (u8, bool, bool, bool) {
    // First subtracting the carry on and check if it causes overflow or halfcarry
    let mut b = b;
    let mut half_carry2 = false;
    let mut carry2 = false;
    if c {
        (b, _, half_carry2, carry2) = subtraction(b, 1);
    }
    let (result, z, halfcarry, carry) = subtraction(a, b);
    (result, z, halfcarry | half_carry2, carry | carry2)
}
/// return (result, halfcarry, carry)
/// TODO: check what halfcarry means here and if/how subtraction should overflow
pub fn subtraction(a: u8, b: u8) -> (u8, bool, bool, bool) {
    let (result, carry) = a.overflowing_sub(b);
    let (_, halfcarry) = a.low_nibble().overflowing_sub(b.low_nibble());
    (result, result == 0, halfcarry, carry)
}
pub fn compare(a: u8, b: u8) -> (bool, bool, bool) {
    let (_, z, h, c) = subtraction(a, b);
    (z, h, c)
}
pub fn inc(a: u8) -> (u8, bool, bool, bool) {
    addition(a, 1)
}
pub fn dec(a: u8) -> (u8, bool, bool, bool) {
    subtraction(a, 1)
}
pub fn complement(a: u8) -> u8 {
    a ^ 0xFF
}
/// (new_a, zero, carry)
pub fn daa(a: u8, h: bool, c: bool) -> (u8, bool, bool) {
    let mut new_a = a;
    let high = a.high_nibble();
    let low = a.low_nibble();
    let mut c1 = false;
    let mut c2 = false;
    if h || (low > 9) {
        (new_a, _, _, c1) = addition(new_a, 0x06);
    }
    if c || (high > 9) {
        (new_a, _, _, c2) = addition(new_a, 0x60);
    }
    (new_a, new_a == 0, c1 | c2)
}

#[cfg(test)]
mod tests {
    use crate::util::math_util::signed_addition;

    #[test]
    fn signed_addition_test() {
        let a = 200u16;
        let b = 0b1111_1111u8;
        assert_eq!(199, signed_addition(a, b).0)
    }
    #[test]
    fn addition_test() {
        let a = 200u16;
        let b = 0b1111_1111u8;
        assert_eq!(199, signed_addition(a, b).0)
    }
}
