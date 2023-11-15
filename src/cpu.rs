use std::{cell::RefCell, rc::Rc};

use crate::util::math_util::{
    adc, addition, and, compare, complement, daa, dec, inc, or, res_bit, rotate_left,
    rotate_left_carry, rotate_right, rotate_right_carry, sbc, set_bit, shift_left_arithmetic,
    shift_right_arithmetic, shift_right_logical, subtraction, swap_nibble, test_bit, xor,
};
use crate::{
    bus::Bus,
    register::Registers,
    util::{
        extract_opcode::{load_json, Instruction, Operand},
        math_util::signed_addition,
        opcode_dict_util::{NopreOpcodeMnemonics, NopreOperands, PrefixOpcodeMnemonics},
    },
};
pub struct CPU {
    pub reg: Registers,
    pub bus: Rc<RefCell<Bus>>,
    cycles_since_last_cmd: u64,
    cycles_to_wait: u64,
    instruction_dict_notprefixed: Vec<Instruction>,
    instruction_dict_prefixed: Vec<Instruction>,
}
impl CPU {
    pub fn new(bus: Rc<RefCell<Bus>>) -> CPU {
        CPU {
            reg: Registers::new(),
            bus,
            cycles_to_wait: 0,
            cycles_since_last_cmd: 0,
            instruction_dict_notprefixed: load_json("opcodes_nopre.json").unwrap(),
            instruction_dict_prefixed: load_json("opcodes_pre.json").unwrap(),
        }
    }
    // Check if I should do stuff and wait the proper amount of cycle
    // or wait and then do stuff
    pub fn next_tick(&mut self) {
        if self.cycles_since_last_cmd >= self.cycles_to_wait {
            self.cycles_since_last_cmd = 0;
            // Run next command
        }
        self.cycles_since_last_cmd += 1;
    }
    pub fn tick(&mut self, opcode: u8) {
        if opcode == 0xCB {
            // Set opcode to next byte
            let opcode = self.bus.borrow().read_byte(self.reg.get_pc_next());
            self.opcode_prefixed_tick(self.instruction_dict_prefixed[opcode as usize].clone());
        } else {
            self.opcode_noprefix_tick(
                self.instruction_dict_notprefixed[opcode as usize].clone(),
                opcode,
            );
        };
    }

    fn opcode_noprefix_tick(&mut self, instruction: Instruction, opcode: u8) {
        let mnemonic: NopreOpcodeMnemonics = instruction.mnemonic.clone().into();
        let reg = &mut self.reg;
        match mnemonic {
            NopreOpcodeMnemonics::AND
            | NopreOpcodeMnemonics::OR
            | NopreOpcodeMnemonics::XOR
            | NopreOpcodeMnemonics::ADD
            | NopreOpcodeMnemonics::SUB
            | NopreOpcodeMnemonics::ADC
            | NopreOpcodeMnemonics::SBC
            | NopreOpcodeMnemonics::DEC
            | NopreOpcodeMnemonics::INC
            | NopreOpcodeMnemonics::CP
            | NopreOpcodeMnemonics::DAA
            | NopreOpcodeMnemonics::CPL
            | NopreOpcodeMnemonics::CCF
            | NopreOpcodeMnemonics::SCF => self.op_arith_8bit(instruction, opcode),
            NopreOpcodeMnemonics::IllegalD3
            | NopreOpcodeMnemonics::IllegalDb
            | NopreOpcodeMnemonics::IllegalDd
            | NopreOpcodeMnemonics::IllegalE3
            | NopreOpcodeMnemonics::IllegalE4
            | NopreOpcodeMnemonics::IllegalEb
            | NopreOpcodeMnemonics::IllegalEc
            | NopreOpcodeMnemonics::IllegalEd
            | NopreOpcodeMnemonics::IllegalF4
            | NopreOpcodeMnemonics::IllegalFc
            | NopreOpcodeMnemonics::IllegalFd => panic!("Illegal instruction"),
            NopreOpcodeMnemonics::LD => self.op_load(instruction, opcode),
            // Check
            NopreOpcodeMnemonics::LDH => self.op_load(instruction, opcode),
            NopreOpcodeMnemonics::PREFIX => self.opcode_prefixed_tick(instruction),
            NopreOpcodeMnemonics::RLA => {
                let (res, _, n, h, c) = rotate_left(reg.get_a());
                reg.set_a(res);
                reg.set_flags(false, n, h, c);
            }
            NopreOpcodeMnemonics::RLCA => {
                let (res, _, n, h, c) = rotate_left_carry(reg.get_a(), reg.flag_c());
                reg.set_a(res);
                reg.set_flags(false, n, h, c);
            }
            NopreOpcodeMnemonics::RRA => {
                let (res, _, n, h, c) = rotate_right(reg.get_a());
                reg.set_a(res);
                reg.set_flags(false, n, h, c);
            }
            NopreOpcodeMnemonics::RRCA => {
                let (res, _, n, h, c) = rotate_right_carry(reg.get_a(), reg.flag_c());
                reg.set_a(res);
                reg.set_flags(false, n, h, c);
            }

            NopreOpcodeMnemonics::JP => self.op_jump(instruction),
            NopreOpcodeMnemonics::JR => self.op_jump_rel(instruction),
            NopreOpcodeMnemonics::CALL => todo!(),
            NopreOpcodeMnemonics::RET => todo!(),
            NopreOpcodeMnemonics::RETI => todo!(),
            NopreOpcodeMnemonics::RST => todo!(),

            NopreOpcodeMnemonics::POP => todo!(),
            NopreOpcodeMnemonics::PUSH => todo!(),

            NopreOpcodeMnemonics::NOP => todo!(),
            NopreOpcodeMnemonics::EI => todo!(),
            NopreOpcodeMnemonics::DI => todo!(),
            NopreOpcodeMnemonics::HALT => todo!(),
            NopreOpcodeMnemonics::STOP => todo!(),
            NopreOpcodeMnemonics::INVALID => panic!("Illegal invalid instruction INVALID"),
        }
    }
    fn opcode_prefixed_tick(&mut self, instruction: Instruction) {
        let mnemonic: PrefixOpcodeMnemonics = instruction.mnemonic.clone().into();
        match mnemonic {
            PrefixOpcodeMnemonics::RLC => {
                self.op_pre_1arg_with_carry(instruction, rotate_right_carry)
            }
            PrefixOpcodeMnemonics::RRC => {
                self.op_pre_1arg_with_carry(instruction, rotate_left_carry)
            }
            PrefixOpcodeMnemonics::RL => self.op_pre_1arg_carryless(instruction, rotate_right),
            PrefixOpcodeMnemonics::RR => self.op_pre_1arg_carryless(instruction, rotate_left),
            PrefixOpcodeMnemonics::SLA => {
                self.op_pre_1arg_carryless(instruction, shift_left_arithmetic)
            }
            PrefixOpcodeMnemonics::SRA => {
                self.op_pre_1arg_carryless(instruction, shift_right_arithmetic)
            }
            PrefixOpcodeMnemonics::SRL => {
                self.op_pre_1arg_carryless(instruction, shift_right_logical)
            }
            PrefixOpcodeMnemonics::SWAP => self.op_pre_1arg_carryless(instruction, swap_nibble),

            PrefixOpcodeMnemonics::SET => self.op_pre_res_set(instruction, set_bit),
            PrefixOpcodeMnemonics::RES => self.op_pre_res_set(instruction, res_bit),
            PrefixOpcodeMnemonics::BIT => self.op_pre_bit(instruction, test_bit),
            PrefixOpcodeMnemonics::INVALID => panic!("Invalid prefixed mnemonic"),
        }
    }
    fn op_inc_dec_8bit<F>(&mut self, a: NopreOperands, b: NopreOperands, f: F)
    where
        F: Fn(u8) -> (u8, bool, bool, bool),
    {
        let bus = self.bus.borrow_mut();
        let mut reg = &mut self.reg;
        let carry = reg.flag_c();
        let result = match b {
            NopreOperands::A
            | NopreOperands::B
            | NopreOperands::C
            | NopreOperands::D
            | NopreOperands::E
            | NopreOperands::H => f(reg.get(b)),
            NopreOperands::HL => f(bus.read_byte(reg.hl)),
            NopreOperands::n8 => f(bus.read_next_byte(reg.pc)),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
        let (new_a, z, n, h) = result;
        reg.set_flag_z(z);
        reg.set_flag_n(n);
        reg.set_flag_h(h);
        reg.set_a(new_a);
    }

    //call
    fn op_call(&mut self, instruction: Instruction) {
        let bus = self.bus.borrow_mut();
        let reg = &mut self.reg;
        let conditional = instruction.operands.len() == 2;
        // set to true so if not conditional call the last if is still visited
        let mut condition = true;
        if conditional {
            let condition_operand = instruction.operands[0].name.clone().into();
            condition = match condition_operand {
                NopreOperands::Z => reg.flag_z(),
                NopreOperands::NZ => !reg.flag_z(),
                NopreOperands::C => reg.flag_c(),
                NopreOperands::NC => !reg.flag_c(),
                _ => panic!("Invalid jump condition"),
            }
        }
        //TODO: doubt about SP=SP-2
        //call to nn, SP=SP-2, (SP)=PC, PC=nn
        if condition {
            reg.sp = reg.sp-2;
            bus.write_2_bytes(reg.sp, reg.pc);
        }
    }
    //jr
    fn op_jump_rel(&mut self, instruction: Instruction) {
        let bus = self.bus.borrow_mut();
        let reg = &mut self.reg;
        let conditional = instruction.operands.len() == 2;
        let mut condition = true;
        if conditional {
            let condition_operand = instruction.operands[0].name.clone().into();
            condition = match condition_operand {
                NopreOperands::Z => reg.flag_z(),
                NopreOperands::NZ => !reg.flag_z(),
                NopreOperands::C => reg.flag_c(),
                NopreOperands::NC => !reg.flag_c(),
                _ => panic!("Invalid jump condition"),
            }
        }
        if condition {
            let next_byte = bus.read_next_byte(reg.pc);
            (reg.pc, _) = signed_addition(reg.pc, next_byte);
        }
    }
    //jp and conditional jp
    fn op_jump(&mut self, instruction: Instruction) {
        let bus = self.bus.borrow_mut();
        let reg = &mut self.reg;
        let conditional = instruction.operands.len() == 2;
        let mut i = 0;
        let mut condition = true;
        if conditional {
            let condition_operand = instruction.operands[0].name.clone().into();
            i = 1;
            condition = match condition_operand {
                NopreOperands::Z => reg.flag_z(),
                NopreOperands::NZ => !reg.flag_z(),
                NopreOperands::C => reg.flag_c(),
                NopreOperands::NC => !reg.flag_c(),
                _ => panic!("Invalid jump condition"),
            }
        }
        let target = instruction.operands[i].name.clone().into();
        let new_pc = match target {
            NopreOperands::HL => reg.hl,
            NopreOperands::a16 => bus.get_a16_address(reg.pc),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
        if condition {
            reg.pc = new_pc;
        }
    }
    fn op_cp_8bit(&mut self, a: NopreOperands, b: NopreOperands) {
        let bus = self.bus.borrow_mut();
        let reg = &mut self.reg;
        let a = reg.get_a();
        let result = match b {
            NopreOperands::A
            | NopreOperands::B
            | NopreOperands::C
            | NopreOperands::D
            | NopreOperands::E
            | NopreOperands::H => compare(a, reg.get(b)),
            NopreOperands::HL => compare(a, bus.read_byte(reg.hl)),
            NopreOperands::n8 => compare(a, bus.read_next_byte(reg.pc)),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
        let (z, n, h, c) = result;
        reg.set_flags(z, n, h, c);
    }
    fn op_adc_sbc_8bit<F>(&mut self, a: NopreOperands, b: NopreOperands, f: F)
    where
        F: Fn(u8, u8, bool) -> (u8, bool, bool, bool, bool),
    {
        let bus = self.bus.borrow_mut();
        let mut reg = &mut self.reg;
        let a = reg.get_a();
        let carry = reg.flag_c();
        let result = match b {
            NopreOperands::A
            | NopreOperands::B
            | NopreOperands::C
            | NopreOperands::D
            | NopreOperands::E
            | NopreOperands::H => f(a, reg.get(b), carry),
            NopreOperands::HL => f(a, bus.read_byte(reg.hl), carry),
            NopreOperands::n8 => f(a, bus.read_next_byte(reg.pc), carry),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
        let (new_a, z, n, h, c) = result;
        reg.set_flags(z, n, h, c);
        reg.set_a(new_a);
    }

    fn op_add_sub_bit_8bit<F>(&mut self, a: NopreOperands, b: NopreOperands, f: F)
    where
        F: Fn(u8, u8) -> (u8, bool, bool, bool, bool),
    {
        let bus = self.bus.borrow_mut();
        let mut reg = &mut self.reg;
        let a = reg.get_a();
        let result = match b {
            NopreOperands::A
            | NopreOperands::B
            | NopreOperands::C
            | NopreOperands::D
            | NopreOperands::E
            | NopreOperands::H => f(a, reg.get(b)),
            NopreOperands::HL => f(a, bus.read_byte(reg.hl)),
            NopreOperands::n8 => f(a, bus.read_next_byte(reg.pc)),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
        let (new_a, z, n, h, c) = result;
        reg.set_flags(z, n, h, c);
        reg.set_a(new_a);
    }
    fn op_add_16bit(&mut self, instruction: Instruction, opcode: u8) {}

    // Filter ADD INC DEC that have both 8 and 16 bit version
    fn op_arith(&mut self, instruction: Instruction, opcode: u8) {
        //Dispatch 8bit and 16bit load instruction
        match opcode {
            0x03 | 0x13 | 0x23 | 0x33 | 0x09 | 0x19 | 0x29 | 0x39 | 0x0b | 0x1b | 0x2b | 0x3b
            | 0xe8 => self.op_arith_16bit(instruction, opcode),
            _ => self.op_load_8bit(instruction),
        }
    }

    fn op_arith_16bit(&mut self, instruction: Instruction, opcode: u8) {
        let mnemonic: NopreOpcodeMnemonics = instruction.mnemonic.into();
        let value = match mnemonic {
            NopreOpcodeMnemonics::ADD => todo!(),
            NopreOpcodeMnemonics::INC => todo!(),
            NopreOpcodeMnemonics::DEC => todo!(),
            NopreOpcodeMnemonics::INVALID => todo!(),
            _ => panic!("No operand match"),
        };
    }

    fn op_arith_8bit(&mut self, instruction: Instruction, opcode: u8) {
        let (a, b) = instruction.operands_tuple().unwrap();
        let mnemonic: NopreOpcodeMnemonics = instruction.mnemonic.into();
        let a_type: NopreOperands = a.name.into();
        let b_type: NopreOperands = b.name.into();
        let mut reg = &mut self.reg;

        let value = match mnemonic {
            NopreOpcodeMnemonics::AND => self.op_add_sub_bit_8bit(a_type, b_type, and),
            NopreOpcodeMnemonics::OR => self.op_add_sub_bit_8bit(a_type, b_type, or),
            NopreOpcodeMnemonics::XOR => self.op_add_sub_bit_8bit(a_type, b_type, xor),
            NopreOpcodeMnemonics::ADD => self.op_add_sub_bit_8bit(a_type, b_type, addition),
            NopreOpcodeMnemonics::SUB => self.op_add_sub_bit_8bit(a_type, b_type, subtraction),
            NopreOpcodeMnemonics::ADC => self.op_adc_sbc_8bit(a_type, b_type, adc),
            NopreOpcodeMnemonics::SBC => self.op_adc_sbc_8bit(a_type, b_type, sbc),
            NopreOpcodeMnemonics::DEC => self.op_inc_dec_8bit(a_type, b_type, dec),
            NopreOpcodeMnemonics::INC => self.op_inc_dec_8bit(a_type, b_type, inc),
            NopreOpcodeMnemonics::CP => self.op_cp_8bit(a_type, b_type),
            NopreOpcodeMnemonics::DAA => {
                let (res, z, c) = daa(reg.get_a(), reg.flag_h(), reg.flag_c());
                reg.set_a(res);
                reg.set_flag_z(z);
                reg.set_flag_c(c);
            }
            NopreOpcodeMnemonics::CPL => {
                reg.set_a(complement(reg.get_a()));
            }
            NopreOpcodeMnemonics::CCF => {
                reg.set_flag_n(false);
                reg.set_flag_h(false);
                reg.set_flag_c(reg.flag_c() ^ true);
            }
            NopreOpcodeMnemonics::SCF => {
                reg.set_flag_n(false);
                reg.set_flag_h(false);
                reg.set_flag_c(true);
            }
            _ => panic!("No operand match"),
        };
    }
    fn op_load(&mut self, instruction: Instruction, opcode: u8) {
        //Dispatch 8bit and 16bit load instruction
        match opcode {
            0x01 | 0x11 | 0x21 | 0x31 | 0x08 | 0xF8 | 0xF9 => {
                self.op_load_16bit(instruction, opcode)
            }
            _ => self.op_load_8bit(instruction),
        }
    }
    fn op_load_16bit(&mut self, instruction: Instruction, opcode: u8) {
        //TODO: F8 3 operands
        //Check for 2 operands
        let (into, from) = instruction.operands_tuple().unwrap();
        let into_type: NopreOperands = into.name.into();
        let from_type: NopreOperands = from.name.into();
        let bus = self.bus.borrow_mut();
        let reg = &mut self.reg;

        let value = match from_type {
            NopreOperands::INVALID => panic!("Invalid LD operands"),

            NopreOperands::HL => reg.hl,
            NopreOperands::SP => {
                match opcode {
                    0xF8 => {
                        // TODO: flags h check
                        let (result, c) = signed_addition(reg.sp, bus.read_next_byte(reg.pc));
                        reg.set_flag_c(c);
                        // reg.set_flags(false, false, false, result.1);
                        result
                    }
                    0x08 => reg.sp,
                    _ => panic!("Impossible state"),
                }
            }
            // TODO: I think a16 and n16 are the same but used in different context:
            // examples
            // n16 when put the next 2 bytes in a Registers
            // a16 to make calls or jump to the address written by the next 2 bytes
            // [a16] when accesccing the value at a16 address
            NopreOperands::n16 => bus.get_a16_address(reg.pc),

            _ => panic!("No operand match"),
        };
        match into_type {
            // TODO: How to write 16bits in 8bits?
            NopreOperands::a16 => todo!(),
            NopreOperands::BC => reg.bc = value,
            NopreOperands::SP => reg.sp = value,
            NopreOperands::DE => reg.de = value,
            NopreOperands::HL => reg.hl = value,
            NopreOperands::INVALID => panic!("Invalid operands"),
            _ => panic!("No operand match"),
        }
    }
    fn op_load_8bit(&mut self, instruction: Instruction) {
        let (into, from) = instruction.operands_tuple().unwrap();
        let into_type: NopreOperands = into.name.into();
        let from_type: NopreOperands = from.name.into();
        let mut bus = self.bus.borrow_mut();
        let reg = &mut self.reg;

        let value = match from_type {
            NopreOperands::A
            | NopreOperands::B
            | NopreOperands::D
            | NopreOperands::E
            | NopreOperands::H
            | NopreOperands::L => reg.get(from_type),
            NopreOperands::a8 => bus.read_next_byte(reg.pc),
            NopreOperands::BC => bus.read_byte(reg.bc),
            //check if c or [c] with bytes?,
            NopreOperands::C => {
                if from.immediate {
                    reg.get_c()
                } else {
                    bus.read_byte(0xFF00 + reg.get_c() as u16)
                }
            }
            NopreOperands::DE => bus.read_byte(reg.de),
            NopreOperands::HL => {
                if from.increment.is_some() {
                    reg.hl_plus();
                } else if from.decrement.is_some() {
                    reg.hl_minus();
                }
                bus.read_byte(reg.hl)
            }
            NopreOperands::n8 => bus.read_next_byte(reg.pc),
            NopreOperands::a16 => bus.get_a16_value(reg.pc),
            NopreOperands::INVALID => panic!("Invalid LD operands"),
            _ => panic!("Missing operand"),
        };
        match into_type {
            NopreOperands::A => reg.set_a(value),
            NopreOperands::a16 => {
                let a16 = bus.get_a16_address(reg.pc);
                bus.write_byte(a16, value)
            }
            NopreOperands::a8 => {
                let a8 = get_a8_address(bus.read_next_byte(reg.pc));
                bus.write_byte(a8, value)
            }
            NopreOperands::B => reg.set_b(value),
            NopreOperands::BC => bus.write_byte(reg.bc, value),
            //check if c or [c] with bytes?,
            NopreOperands::C => {
                if from.immediate {
                    reg.set_c(value)
                } else {
                    bus.write_byte(0xFF00 + reg.get_c() as u16, value)
                }
            }
            NopreOperands::D => reg.set_d(value),
            NopreOperands::DE => bus.write_byte(reg.bc, value),
            NopreOperands::E => reg.set_e(value),
            NopreOperands::H => reg.set_h(value),
            NopreOperands::HL => {
                // TODO: extract logic
                if from.increment.is_some() {
                    reg.hl_plus();
                } else if from.decrement.is_some() {
                    reg.hl_minus();
                }
                bus.write_byte(reg.hl, value)
            }
            NopreOperands::L => reg.set_l(value),
            NopreOperands::INVALID => panic!("Invalid operands"),

            _ => panic!("Missing operands"),
        }
    }
    fn op_pre_bit<F>(&mut self, instruction: Instruction, f: F)
    where
        F: Fn(u8, u8) -> (bool, bool, bool),
    {
        let bit_number = match instruction.operands[0].name.clone().parse::<u8>() {
            Ok(number) => number,
            Err(_) => panic!("res/set instruction operand not a parsable number"),
        };
        let b = instruction.operands[1].name.clone().into();
        let bus = self.bus.borrow_mut();
        let reg = &mut self.reg;
        let (z, n, h) = match b {
            NopreOperands::A => f(reg.get_a(), bit_number),
            NopreOperands::B => f(reg.get_b(), bit_number),
            NopreOperands::C => f(reg.get_c(), bit_number),
            NopreOperands::D => f(reg.get_d(), bit_number),
            NopreOperands::E => f(reg.get_e(), bit_number),
            NopreOperands::H => f(reg.get_h(), bit_number),
            NopreOperands::L => f(reg.get_l(), bit_number),
            NopreOperands::HL => f(bus.read_byte(reg.hl), bit_number),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for res/set ?"),
        };
        reg.set_flag_z(z);
        reg.set_flag_n(n);
        reg.set_flag_h(h);
    }
    fn op_pre_res_set<F>(&mut self, instruction: Instruction, f: F)
    where
        F: Fn(u8, u8) -> u8,
    {
        let bit_number = match instruction.operands[0].name.clone().parse::<u8>() {
            Ok(number) => number,
            Err(_) => panic!("res/set instruction operand not a parsable number"),
        };
        let b = instruction.operands[1].name.clone().into();
        let bus = self.bus.borrow_mut();
        let reg = &mut self.reg;
        match b {
            NopreOperands::A => f(reg.get_a(), bit_number),
            NopreOperands::B => f(reg.get_b(), bit_number),
            NopreOperands::C => f(reg.get_c(), bit_number),
            NopreOperands::D => f(reg.get_d(), bit_number),
            NopreOperands::E => f(reg.get_e(), bit_number),
            NopreOperands::H => f(reg.get_h(), bit_number),
            NopreOperands::L => f(reg.get_l(), bit_number),
            NopreOperands::HL => f(bus.read_byte(reg.hl), bit_number),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for res/set ?"),
        };
    }
    fn op_pre_1arg_with_carry<F>(&mut self, instruction: Instruction, f: F)
    where
        F: Fn(u8, bool) -> (u8, bool, bool, bool, bool),
    {
        let a = instruction.operands[0].name.clone().into();
        let bus = self.bus.borrow_mut();
        let reg = &mut self.reg;
        let carry = reg.flag_c();
        let result = match a {
            NopreOperands::A => f(reg.get_a(), carry),
            NopreOperands::B => f(reg.get_b(), carry),
            NopreOperands::C => f(reg.get_c(), carry),
            NopreOperands::D => f(reg.get_d(), carry),
            NopreOperands::E => f(reg.get_e(), carry),
            NopreOperands::H => f(reg.get_h(), carry),
            NopreOperands::L => f(reg.get_l(), carry),
            NopreOperands::HL => f(bus.read_byte(reg.hl), carry),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
        let (new_a, z, n, h, c) = result;
        reg.set_flags(z, n, h, c);
        reg.set_a(new_a);
    }
    // prefixed instruction with only one arg (RLC, RRC...)
    fn op_pre_1arg_carryless<F>(&mut self, instruction: Instruction, f: F)
    where
        F: Fn(u8) -> (u8, bool, bool, bool, bool),
    {
        let a = instruction.operands[0].name.clone().into();
        let bus = self.bus.borrow_mut();
        let mut reg = &mut self.reg;
        let result = match a {
            NopreOperands::A => f(reg.get_a()),
            NopreOperands::B => f(reg.get_b()),
            NopreOperands::C => f(reg.get_c()),
            NopreOperands::D => f(reg.get_d()),
            NopreOperands::E => f(reg.get_e()),
            NopreOperands::H => f(reg.get_h()),
            NopreOperands::L => f(reg.get_l()),
            NopreOperands::HL => f(bus.read_byte(reg.hl)),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
        let (new_a, z, n, h, c) = result;
        reg.set_flags(z, n, h, c);
        reg.set_a(new_a);
    }
}
fn get_a8_address(a8: u8) -> u16 {
    return 0xFF00 + a8 as u16;
}
#[cfg(test)]
mod tests {
    use super::CPU;
    use crate::bus::Bus;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn bus_access() {
        let bus = Rc::new(RefCell::new(Bus::new()));
        let cpu = CPU::new(Rc::clone(&bus));
        let cpu2 = CPU::new(Rc::clone(&bus));
        {
            cpu.bus.borrow_mut().write_slice(0x0010, &[1, 2, 3]);
            let binding = cpu.bus.borrow();
            let slice = binding.read_bytes_range(0x0010, 3);
            assert_eq!(slice, &[1, 2, 3]);
        }
        {
            cpu2.bus.borrow_mut().write_byte(0xFF00, 7);
            let binding = cpu2.bus.borrow();
            let slice = binding.read_bytes_range(0xFF00, 3);
            assert_eq!(slice, &[7, 0, 0]);
        }
    }
    #[test]
    fn load_opcode_from_file() {
        let bus = Rc::new(RefCell::new(Bus::new()));
        let cpu = CPU::new(bus);
        assert_eq!(cpu.instruction_dict_notprefixed[0].mnemonic, "NOP");
        assert_eq!(cpu.instruction_dict_notprefixed[0xFF].mnemonic, "RST");
        assert_eq!(cpu.instruction_dict_prefixed[0xFF].mnemonic, "SET");
    }
}
