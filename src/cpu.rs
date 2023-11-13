use std::{cell::RefCell, rc::Rc};

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
    // Filter ADD INC DEC that have both 8 and 16 bit version
    fn op_arith(&mut self, instruction: Instruction, opcode: u8) {
        //Dispatch 8bit and 16bit load instruction
        match opcode {
            0x03 | 0x13 | 0x23 | 0x33 | 0x09 | 0x19 | 0x29 | 0x39 | 0x0b | 0x1b | 0x2b | 0x3b
            | 0xe8 => self.op_arith_16bit(instruction, opcode),
            _ => self.op_load_8bit(instruction),
        }
    }
    fn op_add_8bit(&mut self, a: NopreOperands, b: NopreOperands) {
        let bus = self.bus.borrow_mut();
        let mut reg = &mut self.reg;
        let z: bool;
        let n = 0;
        let h: bool;
        let c: bool;
        let new_a = match b {
            NopreOperands::A => reg.get_a() + reg.get_a(),
            NopreOperands::B => reg.get_a() + reg.get_b(),
            NopreOperands::C => reg.get_a() + reg.get_c(),
            NopreOperands::D => reg.get_a() + reg.get_d(),
            NopreOperands::E => reg.get_a() + reg.get_e(),
            NopreOperands::H => reg.get_a() + reg.get_h(),
            NopreOperands::HL => reg.get_a() + bus.read_byte(reg.hl),
            NopreOperands::n8 => reg.get_a() + bus.read_next_byte(reg.pc),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
        reg.set_a(new_a);
    }
    fn op_add_16bit(&mut self, instruction: Instruction, opcode: u8) {}
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

        let value = match mnemonic {
            NopreOpcodeMnemonics::ADD => self.op_add_8bit(a_type, b_type),
            NopreOpcodeMnemonics::ADC => todo!(),
            NopreOpcodeMnemonics::AND => todo!(),
            NopreOpcodeMnemonics::CCF => todo!(),
            NopreOpcodeMnemonics::CP => todo!(),
            NopreOpcodeMnemonics::CPL => todo!(),
            NopreOpcodeMnemonics::DAA => todo!(),
            NopreOpcodeMnemonics::DEC => todo!(),
            NopreOpcodeMnemonics::INC => todo!(),
            NopreOpcodeMnemonics::OR => todo!(),
            NopreOpcodeMnemonics::SBC => todo!(),
            NopreOpcodeMnemonics::SCF => todo!(),
            NopreOpcodeMnemonics::SUB => todo!(),
            NopreOpcodeMnemonics::XOR => todo!(),
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
                        let result = signed_addition(reg.sp, bus.read_next_byte(reg.pc));
                        reg.set_flags(false, false, false, result.1);
                        result.0
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
            NopreOperands::A => reg.get_a(),
            NopreOperands::a8 => bus.read_next_byte(reg.pc),
            NopreOperands::B => reg.get_b(),
            NopreOperands::BC => bus.read_byte(reg.bc),
            //check if c or [c] with bytes?,
            NopreOperands::C => {
                if from.immediate {
                    reg.get_c()
                } else {
                    bus.read_byte(0xFF00 + reg.get_c() as u16)
                }
            }
            NopreOperands::D => reg.get_d(),
            NopreOperands::DE => bus.read_byte(reg.de),
            NopreOperands::E => reg.get_e(),
            NopreOperands::H => reg.get_h(),
            NopreOperands::HL => {
                if from.increment.is_some() {
                    reg.hl_plus();
                } else if from.decrement.is_some() {
                    reg.hl_minus();
                }
                bus.read_byte(reg.hl)
            }
            NopreOperands::L => reg.get_l(),
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
    fn opcode_noprefix_tick(&mut self, instruction: Instruction, opcode: u8) {
        let mnemonic: NopreOpcodeMnemonics = instruction.mnemonic.clone().into();
        match mnemonic {
            NopreOpcodeMnemonics::ADC => todo!(),
            NopreOpcodeMnemonics::ADD => todo!(),
            NopreOpcodeMnemonics::AND => todo!(),
            NopreOpcodeMnemonics::CALL => todo!(),
            NopreOpcodeMnemonics::CCF => todo!(),
            NopreOpcodeMnemonics::CP => todo!(),
            NopreOpcodeMnemonics::CPL => todo!(),
            NopreOpcodeMnemonics::DAA => todo!(),
            NopreOpcodeMnemonics::DEC => todo!(),
            NopreOpcodeMnemonics::DI => todo!(),
            NopreOpcodeMnemonics::EI => todo!(),
            NopreOpcodeMnemonics::HALT => todo!(),
            NopreOpcodeMnemonics::INC => todo!(),
            NopreOpcodeMnemonics::IllegalD3 => todo!(),
            NopreOpcodeMnemonics::IllegalDb => todo!(),
            NopreOpcodeMnemonics::IllegalDd => todo!(),
            NopreOpcodeMnemonics::IllegalE3 => todo!(),
            NopreOpcodeMnemonics::IllegalE4 => todo!(),
            NopreOpcodeMnemonics::IllegalEb => todo!(),
            NopreOpcodeMnemonics::IllegalEc => todo!(),
            NopreOpcodeMnemonics::IllegalEd => todo!(),
            NopreOpcodeMnemonics::IllegalF4 => todo!(),
            NopreOpcodeMnemonics::IllegalFc => todo!(),
            NopreOpcodeMnemonics::IllegalFd => todo!(),
            NopreOpcodeMnemonics::JP => todo!(),
            NopreOpcodeMnemonics::JR => todo!(),
            NopreOpcodeMnemonics::LD => self.op_load(instruction, opcode),
            NopreOpcodeMnemonics::LDH => todo!(),
            NopreOpcodeMnemonics::NOP => todo!(),
            NopreOpcodeMnemonics::OR => todo!(),
            NopreOpcodeMnemonics::POP => todo!(),
            NopreOpcodeMnemonics::PREFIX => todo!(),
            NopreOpcodeMnemonics::PUSH => todo!(),
            NopreOpcodeMnemonics::RET => todo!(),
            NopreOpcodeMnemonics::RETI => todo!(),
            NopreOpcodeMnemonics::RLA => todo!(),
            NopreOpcodeMnemonics::RLCA => todo!(),
            NopreOpcodeMnemonics::RRA => todo!(),
            NopreOpcodeMnemonics::RRCA => todo!(),
            NopreOpcodeMnemonics::RST => todo!(),
            NopreOpcodeMnemonics::SBC => todo!(),
            NopreOpcodeMnemonics::SCF => todo!(),
            NopreOpcodeMnemonics::STOP => todo!(),
            NopreOpcodeMnemonics::SUB => todo!(),
            NopreOpcodeMnemonics::XOR => todo!(),
            NopreOpcodeMnemonics::INVALID => todo!(),
        }
    }
    fn opcode_prefixed_tick(&mut self, instruction: Instruction) {
        let mnemonic: PrefixOpcodeMnemonics = instruction.mnemonic.into();
        match mnemonic {
            PrefixOpcodeMnemonics::BIT => todo!(),
            PrefixOpcodeMnemonics::INVALID => todo!(),
            PrefixOpcodeMnemonics::RES => todo!(),
            PrefixOpcodeMnemonics::RL => todo!(),
            PrefixOpcodeMnemonics::RLC => todo!(),
            PrefixOpcodeMnemonics::RR => todo!(),
            PrefixOpcodeMnemonics::RRC => todo!(),
            PrefixOpcodeMnemonics::SET => todo!(),
            PrefixOpcodeMnemonics::SLA => todo!(),
            PrefixOpcodeMnemonics::SRA => todo!(),
            PrefixOpcodeMnemonics::SRL => todo!(),
            PrefixOpcodeMnemonics::SWAP => todo!(),
        }
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
