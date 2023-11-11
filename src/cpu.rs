use crate::register::Register;

struct Operand {
    name: String,
    immediate: bool,
    bytes: u8,
}
struct Flags{}
struct Instruction {
    opcode: u8,
    mnemonic: String,
    bytes: u8,
    cycles: Vec<u8>,
    operands: Vec<Operand>,
    immediate: bool,
    flags: Flags,
}

struct CPU {
    reg: Register,
}
pub fn tick() {

}
