use super::u8_traits::{Bit, NibblesU16, NibblesU8};

/// TODO: Check what should happen if a + b is neg
/// TODO: are signed number ones complement or sign magnitude?
/// Add b interpreted as a signed int into a
/// if the bool is true => overflow occured
pub fn signed_addition(a: u16, b: u8) -> (u16, bool, bool) {
    let signed_b: i32 = b as i8 as i32;
    let signed_a: i32 = a as i32;
    let result = signed_a + signed_b;
    let overflow = a.low_8nibble() as u16 + b as u16 >= 256;
    let half_carry = a.low_4nibble() + b.low_4nibble() >= 16;
    if signed_b > 0 {
        let result = a.wrapping_add(b as u16);
        return (result, half_carry, overflow);
    }
    // Perform signed addition
    (result as u16, half_carry, overflow)
}
/// return (result, sub, halfcarry, carry)
pub fn addition_16bit(a: u16, b: u16) -> (u16, bool, bool, bool) {
    let (result, carry) = a.overflowing_add(b);
    let halfcarry = (a.low_12() + b.low_12()) > 0xFFF;
    (result, false, halfcarry, carry)
}
/// TODO: check when halfcarry ?
/// return (result, zero, sub, halfcarry)
pub fn dec_16bit(a: u16) -> u16 {
    let b = 1u16;
    let (result, half_carry) = a.overflowing_sub(b);
    result
}
pub fn inc_16bit(a: u16) -> u16 {
    let b = 1u16;
    let (result, half_carry) = a.overflowing_add(b);
    result
}
/// return (result, zero, sub, halfcarry, carry)
pub fn addition(a: u8, b: u8) -> (u8, bool, bool, bool, bool) {
    let (result, carry) = a.overflowing_add(b);
    let halfcarry = (a.low_4nibble() + b.low_4nibble()) > 15;
    (result, result == 0, false, halfcarry, carry)
}
/// return (result, halfcarry, carry)
/// TODO: check what halfcarry means here and if/how subtraction should overflow
pub fn subtraction(a: u8, b: u8) -> (u8, bool, bool, bool, bool) {
    let carry = a < b;
    let result = a.wrapping_sub(b);
    let (_, halfcarry) = a.low_4nibble().overflowing_sub(b.low_4nibble());
    (result, result == 0, true, halfcarry, carry)
}
/// return (result, sub, zero, halfcarry, carry)
pub fn adc(a: u8, b: u8, c: bool) -> (u8, bool, bool, bool, bool) {
    // First adding the carry on and check if it causes overflow or halfcarry
    let mut b = b;
    let mut half_carry2 = false;
    let mut carry2 = false;
    if c {
        (b, _, _, half_carry2, carry2) = addition(b, 1);
    }
    let (result, z, _, halfcarry, carry) = addition(a, b);
    (result, z, false, halfcarry | half_carry2, carry | carry2)
}
/// return (result,sub,zero halfcarry, carry)
pub fn sbc(a: u8, b: u8, c: bool) -> (u8, bool, bool, bool, bool) {
    // First subtracting the carry on and check if it causes overflow or halfcarry
    let mut half_carry2 = false;
    let mut carry2 = false;
    let mut res1 = a;
    if c {
        (res1, _, _, half_carry2, carry2) = subtraction(a, 1);
    }
    let (result, z, _, halfcarry, carry) = subtraction(res1, b);
    (result, z, true, halfcarry | half_carry2, carry | carry2)
}
/// res,z,n,h
pub fn inc(a: u8) -> (u8, bool, bool, bool) {
    let (res, z, n, h, _) = addition(a, 1);
    (res, z, false, h)
}
/// res,z,n,h
pub fn dec(a: u8) -> (u8, bool, bool, bool) {
    let (res, z, n, h, _) = subtraction(a, 1);
    (res, z, true, h)
}
/// AND op
/// res, z,n,h,c
pub fn and(a: u8, b: u8) -> (u8, bool, bool, bool, bool) {
    let res = a & b;
    (res, res == 0, false, true, false)
}
/// XOR op
/// res, z,n,h,c
pub fn xor(a: u8, b: u8) -> (u8, bool, bool, bool, bool) {
    let res = a ^ b;
    (res, res == 0, false, false, false)
}
/// OR op
/// res, z,n,h,c
pub fn or(a: u8, b: u8) -> (u8, bool, bool, bool, bool) {
    let res = a | b;
    (res, res == 0, false, false, false)
}
/// z,n,h,c
pub fn compare(a: u8, b: u8) -> (bool, bool, bool, bool) {
    let (_, z, _, h, c) = subtraction(a, b);
    (z, true, h, c)
}
pub fn complement(a: u8) -> u8 {
    a ^ 0xFF
}
/// return (new_a, zero, carry)
pub fn daa(a: u8, h: bool, c: bool, n: bool) -> (u8, bool, bool) {
    let mut new_a = a;
    let low = a.low_4nibble();
    let high = a.high_nibble();
    let mut c1 = false;
    let mut c2 = false;
    let mut carry = false;
    match n {
        true => {
            carry = c;
            if c {
                (new_a, _, _, _, c2) = subtraction(new_a, 0x60);
            }
            if h {
                (new_a, _, _, _, c1) = subtraction(new_a, 0x06);
            }
        }
        false => {
            if c || (new_a > 0x99) {
                carry = true;
                (new_a, _, _, _, c2) = addition(new_a, 0x60);
            }
            if h || (new_a.low_4nibble() > 0x09) {
                (new_a, _, _, _, c1) = addition(new_a, 0x06);
            }
        }
    }
    (new_a, new_a == 0, carry)
}

//TODO:use helper function to set bit

pub fn swap_nibble(a: u8) -> (u8, bool, bool, bool, bool) {
    let res = (a.low_4nibble() << 4) + a.high_nibble();
    (res, res == 0, false, false, false)
}
pub fn rotate_right_carry(a: u8, carry: bool) -> (u8, bool, bool, bool, bool) {
    let overflow = a.get_bit(0);
    let mut res = a.rotate_right(1);
    // set carry bit in it's place
    res.set_bit(7, carry);
    (res, res == 0, false, false, overflow)
}
pub fn rotate_left_carry(a: u8, carry: bool) -> (u8, bool, bool, bool, bool) {
    let overflow = a.get_bit(7);
    let mut res = a.rotate_left(1);
    res.set_bit(0, carry);
    (res, res == 0, false, false, overflow)
}
pub fn rotate_right(a: u8) -> (u8, bool, bool, bool, bool) {
    let overflow = a.get_bit(0);
    let res = a.rotate_right(1);
    (res, res == 0, false, false, overflow)
}
pub fn rotate_left(a: u8) -> (u8, bool, bool, bool, bool) {
    let overflow = a.get_bit(7);
    let res = a.rotate_left(1);
    (res, res == 0, false, false, overflow)
}
pub fn shift_right_logical(a: u8) -> (u8, bool, bool, bool, bool) {
    let overflow = a.get_bit(0);
    let res = a >> 1;
    (res, res == 0, false, false, overflow)
}
pub fn shift_right_arithmetic(a: u8) -> (u8, bool, bool, bool, bool) {
    let sign = (a & 0b1000_0000) != 0;
    let overflow = a.get_bit(0);
    //we remove the sign bit to avoid it being carried down
    let mut res = a & 0b0111_1111;
    res = res >> 1;
    //we put the sign bit back
    res.set_bit(7, sign);
    (res, res == 0, false, false, overflow)
}
pub fn shift_left_arithmetic(a: u8) -> (u8, bool, bool, bool, bool) {
    let sign = a.get_bit(7);
    let overflow = a.get_bit(6);
    //we remove the sign bit to avoid it being carried down
    let mut res = a & 0b0111_1111;
    res = res << 1;
    //we put the sign bit back
    res.set_bit(7, sign);
    (res, res == 0, false, false, overflow)
}
// test result(z flag), n, h
pub fn test_bit(byte: u8, bit: u8) -> (bool, bool, bool) {
    let res = byte.get_bit(bit);
    (!res, false, true)
}
// set bit to 1
pub fn set_bit(byte: u8, bit: u8) -> u8 {
    let mut res = byte;
    res.set_bit(bit, true);
    res
}
// reset bit to 0
pub fn res_bit(byte: u8, bit: u8) -> u8 {
    let mut res = byte;
    res.set_bit(bit, false);
    res
}

#[cfg(test)]
mod tests {
    use crate::util::math_util::{signed_addition, Bit};

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
    #[test]
    fn bit_set_get_test() {
        let mut a: u8 = 0b0000_0000;
        a.set_bit(2, true);
        assert_eq!(a.get_bit(2), true);
        a.set_bit(7, true);
        assert_eq!(a.get_bit(7), true);
        a.set_bit(7, false);
        assert_eq!(a.get_bit(7), false);
        // false if weird value
        a.set_bit(9, true);
        assert_eq!(a.get_bit(9), false);
    }
}
