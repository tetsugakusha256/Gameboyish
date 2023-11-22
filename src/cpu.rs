use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::{cell::RefCell, rc::Rc};

use crate::bus::{InteruptReg, InteruptType};
use crate::util::math_util::{
    adc, addition, addition_16bit, and, compare, complement, daa, dec, dec_16bit, inc, inc_16bit,
    or, res_bit, rotate_left, rotate_left_carry, rotate_right, rotate_right_carry, sbc, set_bit,
    shift_left_arithmetic, shift_right_arithmetic, shift_right_logical, subtraction, swap_nibble,
    test_bit, xor,
};
use crate::{
    bus::Bus,
    register::Registers,
    util::{
        extract_opcode::{load_json, Instruction},
        math_util::signed_addition,
        opcode_dict_util::{NopreOpcodeMnemonics, NopreOperands, PrefixOpcodeMnemonics},
    },
};
pub struct CPU {
    pub reg: Registers,
    pub bus: Rc<RefCell<Bus>>,
    cycles_since_last_cmd: u64,
    cycles_to_wait: u8,
    total_tick: u64,

    ime: bool,
    interupt_reg: InteruptReg,
    interupt_happened: bool,
    opcode: u8,
    current_interupt: Option<InteruptType>,
    interupt_stage: u8,
    next_pc: u16,

    log_buffer: Option<io::BufWriter<File>>,

    instruction_dict_notprefixed: Vec<Instruction>,
    instruction_dict_prefixed: Vec<Instruction>,
}
impl CPU {
    pub fn new(bus: Rc<RefCell<Bus>>) -> CPU {
        CPU {
            reg: Registers::new(),
            interupt_reg: InteruptReg::new(Rc::clone(&bus)),
            bus,
            cycles_to_wait: 0,
            opcode: 0x00,
            cycles_since_last_cmd: 0,
            total_tick: 0,
            instruction_dict_notprefixed: load_json("opcodes_nopre.json").unwrap(),
            instruction_dict_prefixed: load_json("opcodes_pre.json").unwrap(),
            ime: false,
            next_pc: 0x00,
            log_buffer: None,
            current_interupt: None,
            interupt_stage: 0,
            interupt_happened: false,
        }
    }
    pub fn new_doctor(bus: Rc<RefCell<Bus>>) -> CPU {
        CPU {
            reg: Registers::new_doctor(),
            interupt_reg: InteruptReg::new(Rc::clone(&bus)),
            bus,
            cycles_to_wait: 0,
            cycles_since_last_cmd: 0,
            total_tick: 0,
            opcode: 0x00,
            instruction_dict_notprefixed: load_json("opcodes_nopre.json").unwrap(),
            instruction_dict_prefixed: load_json("opcodes_pre.json").unwrap(),
            ime: false,
            next_pc: 0x00,
            log_buffer: None,
            current_interupt: None,
            interupt_stage: 0,
            interupt_happened: false,
        }
    }
    pub fn init_with_log(&mut self) {
        self.init_log_file("log/log_file.txt");
    }
    pub fn stop(&mut self) {
        // TODO: what should actually happen is a way to stop the timer from ticking
        self.bus.borrow_mut().write_byte_as_cpu(0xFF04, 0x00);
    }
    // Check if I should do stuff and wait the proper amount of cycle
    // or wait and then do stuff
    pub fn next_tick(&mut self) {
        self.cycles_since_last_cmd += 1;
        if self.cycles_since_last_cmd >= self.cycles_to_wait as u64 {
            self.total_tick += 1;
            // Run next command
            self.opcode = self.bus.borrow().read_byte_as_cpu(self.reg.pc);
            self.check_for_interupt();

            // if interupt update opcode
            if self.interupt_happened {
                self.interupt_happened = false;
                self.reg.pc = self.next_pc;
                self.opcode = self.bus.borrow().read_byte_as_cpu(self.reg.pc);
            }
            if self.log_buffer.is_some() {
                self.log_state_to_file();
            }
            if self.total_tick % 1000 == 0 {
                // println!(
                //     "Running code: {:#04x}, cycle: {:02}, pc: {:#04x}, total ticks: {}",
                //     self.opcode, self.cycles_since_last_cmd, self.reg.pc, self.total_tick
                // );
                let reg = &self.reg;
                let bus = self.bus.borrow();
                let mut text = format!(
                    "A:{:#04x} F:{:#04x} B:{:#04x} C:{:#04x} D:{:#04x} E:{:#04x} H:{:#04x} L:{:#04x} SP:{:#06x} PC:{:#06x} PCMEM:{:#04x},{:#04x},{:#04x},{:#04x}, ime: {}, ticks: {}",
                    reg.get_a(),
                    reg.get_f(),
                    reg.get_b(),
                    reg.get_c(),
                    reg.get_d(),
                    reg.get_e(),
                    reg.get_h(),
                    reg.get_l(),
                    reg.sp,
                    reg.pc,
                    bus.read_byte_as_cpu(reg.pc),
                    bus.read_byte_as_cpu(reg.pc.wrapping_add(1)),
                    bus.read_byte_as_cpu(reg.pc.wrapping_add(2)),
                    bus.read_byte_as_cpu(reg.pc.wrapping_add(3)),
                    self.ime,
                    self.total_tick
                );
                println!("{}",text);
            }
            // set cycle timing
            self.cycles_since_last_cmd = 0;
            // Run opcode
            self.tick(self.opcode);
            // Applying the next_pc mem that might have been altered by a jump operation
            // println!("pc: {}, next pc: {}", self.reg.pc, self.next_pc);
            self.reg.pc = self.next_pc;
        }
    }
    pub fn tick(&mut self, opcode: u8) {
        // Set next_pc mem according to instruction byte length might be assigned again by a jump
        self.next_pc = self.get_next_pc(opcode);
        self.cycles_to_wait = self.get_cycles_to_wait(opcode);
        if opcode == 0xCB {
            // Set opcode to next byte
            let opcode = self.bus.borrow().read_byte_as_cpu(self.reg.get_pc_next());
            self.opcode_prefixed_tick(self.instruction_dict_prefixed[opcode as usize].clone());
        } else {
            self.opcode_noprefix_tick(
                self.instruction_dict_notprefixed[opcode as usize].clone(),
                opcode,
            );
        };
    }
    fn get_cycles_to_wait(&self, opcode: u8) -> u8 {
        let dict;
        if opcode == 0xCB {
            dict = &self.instruction_dict_prefixed;
        } else {
            dict = &self.instruction_dict_notprefixed;
        };
        let instruction = &dict[opcode as usize];
        instruction.cycles[0] as u8
    }

    fn get_next_pc(&self, opcode: u8) -> u16 {
        let dict;
        if opcode == 0xCB {
            dict = &self.instruction_dict_prefixed;
        } else {
            dict = &self.instruction_dict_notprefixed;
        };
        let instruction = &dict[opcode as usize];
        // println!("instruction byte:{}, opcode:{}", instruction.bytes, &opcode);
        // println!("opcode:{}, opcode usize:{}", &opcode, opcode as usize);
        // println!("instruction {}", instruction);
        // println!("instruction 0x01 {}", &dict[0x01]);
        // println!("instruction 1 {}", &dict[1]);
        return self.reg.pc + instruction.bytes as u16;
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
            | NopreOpcodeMnemonics::SCF => self.op_arith(instruction, opcode),
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
            NopreOpcodeMnemonics::RLCA => {
                let (res, z, n, h, c) = rotate_left(reg.get_a());
                reg.set_a(res);
                reg.set_flags(false, n, h, c);
            }
            NopreOpcodeMnemonics::RLA => {
                let (res, z, n, h, c) = rotate_left_carry(reg.get_a(), reg.flag_c());
                reg.set_a(res);
                reg.set_flags(false, n, h, c);
            }
            NopreOpcodeMnemonics::RRCA => {
                let (res, z, n, h, c) = rotate_right(reg.get_a());
                reg.set_a(res);
                reg.set_flags(false, n, h, c);
            }
            NopreOpcodeMnemonics::RRA => {
                let (res, z, n, h, c) = rotate_right_carry(reg.get_a(), reg.flag_c());
                reg.set_a(res);
                reg.set_flags(false, n, h, c);
            }

            NopreOpcodeMnemonics::JP => self.op_jump(instruction),
            NopreOpcodeMnemonics::JR => self.op_jump_rel(instruction),
            NopreOpcodeMnemonics::CALL => self.op_call(instruction),
            NopreOpcodeMnemonics::RET => self.op_ret(instruction),
            NopreOpcodeMnemonics::RETI => {
                self.ime = false;
                self.op_ret(instruction);
            }
            NopreOpcodeMnemonics::RST => self.op_rst(instruction),

            NopreOpcodeMnemonics::POP => self.op_pop(instruction),
            NopreOpcodeMnemonics::PUSH => self.op_push(instruction),

            NopreOpcodeMnemonics::NOP => (),
            NopreOpcodeMnemonics::DI => self.ime = false,
            NopreOpcodeMnemonics::EI => self.ime = true,
            NopreOpcodeMnemonics::HALT => self.stop(),
            NopreOpcodeMnemonics::STOP => self.stop(),
            NopreOpcodeMnemonics::INVALID => panic!("Illegal invalid instruction INVALID"),
        }
    }
    fn opcode_prefixed_tick(&mut self, instruction: Instruction) {
        let mnemonic: PrefixOpcodeMnemonics = instruction.mnemonic.clone().into();
        match mnemonic {
            PrefixOpcodeMnemonics::RLC => self.op_pre_1arg_carryless(instruction, rotate_right),
            PrefixOpcodeMnemonics::RRC => self.op_pre_1arg_carryless(instruction, rotate_left),
            PrefixOpcodeMnemonics::RL => {
                self.op_pre_1arg_with_carry(instruction, rotate_left_carry)
            }
            PrefixOpcodeMnemonics::RR => {
                self.op_pre_1arg_with_carry(instruction, rotate_right_carry)
            }
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

    fn op_dec_16bit(&mut self, instruction: Instruction) {
        let target_operand = instruction.operands[0].name.clone().into();
        let reg = &mut self.reg;
        let new_value = match target_operand {
            NopreOperands::SP => dec_16bit(reg.sp),
            NopreOperands::BC => dec_16bit(reg.bc),
            NopreOperands::DE => dec_16bit(reg.de),
            NopreOperands::HL => dec_16bit(reg.hl),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
        match target_operand {
            NopreOperands::SP => reg.sp = new_value,
            NopreOperands::BC => reg.bc = new_value,
            NopreOperands::DE => reg.de = new_value,
            NopreOperands::HL => reg.hl = new_value,
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
    }
    fn op_inc_16bit(&mut self, instruction: Instruction) {
        let target_operand = instruction.operands[0].name.clone().into();
        let reg = &mut self.reg;
        let new_value = match target_operand {
            NopreOperands::SP => inc_16bit(reg.sp),
            NopreOperands::BC => inc_16bit(reg.bc),
            NopreOperands::DE => inc_16bit(reg.de),
            NopreOperands::HL => inc_16bit(reg.hl),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
        match target_operand {
            NopreOperands::SP => reg.sp = new_value,
            NopreOperands::BC => reg.bc = new_value,
            NopreOperands::DE => reg.de = new_value,
            NopreOperands::HL => reg.hl = new_value,
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
    }
    fn op_add_16bit(&mut self, instruction: Instruction) {
        let target_operand = instruction.operands[1].name.clone().into();
        let reg = &mut self.reg;
        let (result, n, h, c) = match target_operand {
            NopreOperands::SP => addition_16bit(reg.hl, reg.sp),
            NopreOperands::BC => addition_16bit(reg.hl, reg.bc),
            NopreOperands::DE => addition_16bit(reg.hl, reg.de),
            NopreOperands::HL => addition_16bit(reg.hl, reg.hl),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
        reg.hl = result;
        reg.set_flag_n(n);
        reg.set_flag_h(h);
        reg.set_flag_c(c);
    }
    fn op_inc_dec_8bit<F>(&mut self, instruction: Instruction, f: F)
    where
        F: Fn(u8) -> (u8, bool, bool, bool),
    {
        let target_operand = instruction.operands[0].name.clone().into();
        let mut bus = self.bus.borrow_mut();
        let reg = &mut self.reg;
        let result = match target_operand {
            NopreOperands::A
            | NopreOperands::B
            | NopreOperands::C
            | NopreOperands::D
            | NopreOperands::E
            | NopreOperands::L
            | NopreOperands::H => f(reg.get(&target_operand)),
            NopreOperands::HL => f(bus.read_byte_as_cpu(reg.hl)),
            NopreOperands::n8 => f(bus.read_next_byte(reg.pc)),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
        let (new_value, z, n, h) = result;
        reg.set_flag_z(z);
        reg.set_flag_n(n);
        reg.set_flag_h(h);
        match target_operand {
            NopreOperands::A
            | NopreOperands::B
            | NopreOperands::C
            | NopreOperands::D
            | NopreOperands::E
            | NopreOperands::L
            | NopreOperands::H => reg.set_byte_reg(&target_operand, new_value),
            NopreOperands::HL => bus.write_byte_as_cpu(reg.hl, new_value),
            NopreOperands::n8 => bus.write_next_byte(reg.pc, new_value),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
    }

    //push
    fn op_push(&mut self, instruction: Instruction) {
        let mut bus = self.bus.borrow_mut();
        let reg = &mut self.reg;
        let address_operand: NopreOperands = instruction.operands[0].name.clone().into();
        let target_reg = match address_operand {
            NopreOperands::AF => reg.af,
            NopreOperands::BC => reg.bc,
            NopreOperands::DE => reg.de,
            NopreOperands::HL => reg.hl,
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Error push"),
        };
        reg.sp = reg.sp - 2;
        bus.write_2_bytes_little_endian(reg.sp, target_reg);
    }
    //pop
    fn op_pop(&mut self, instruction: Instruction) {
        let bus = self.bus.borrow();
        let reg = &mut self.reg;
        let address_operand: NopreOperands = instruction.operands[0].name.clone().into();
        let stack_value = bus.read_2_bytes_little_endian(reg.sp);
        match address_operand {
            NopreOperands::AF => reg.af = stack_value,
            NopreOperands::BC => reg.bc = stack_value,
            NopreOperands::DE => reg.de = stack_value,
            NopreOperands::HL => reg.hl = stack_value,
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Error pop"),
        };
        reg.sp = reg.sp + 2;
    }
    //rst
    fn op_rst(&mut self, instruction: Instruction) {
        let mut bus = self.bus.borrow_mut();
        let reg = &mut self.reg;
        let address_operand: NopreOperands = instruction.operands[0].name.clone().into();
        let address = match address_operand {
            NopreOperands::X00 => 0x0000,
            NopreOperands::X08 => 0x0008,
            NopreOperands::X10 => 0x0010,
            NopreOperands::X18 => 0x0018,
            NopreOperands::X20 => 0x0028,
            NopreOperands::X28 => 0x0028,
            NopreOperands::X30 => 0x0030,
            NopreOperands::X38 => 0x0038,
            NopreOperands::INVALID => panic!(""),
            _ => panic!(""),
        };
        //SP=SP-2 the stack goes 2 addresses down
        //it's 2 because the stack is used to store 16bit value/address
        //and the memory is 8bit
        //call to nn, SP=SP-2, (SP)=PC, PC=nn
        //TODO: CHECK HOW TO WRITE THE 2BYTES which endian to use?
        reg.sp = reg.sp - 2;
        bus.write_2_bytes_little_endian(address, reg.pc);
        self.next_pc = address;
    }
    //call
    fn op_call(&mut self, instruction: Instruction) {
        let mut bus = self.bus.borrow_mut();
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
            };
            if !condition {
                self.cycles_to_wait = instruction.cycles[1] as u8;
            }
        } //SP=SP-2 the stack goes 2 addresses down
          //it's 2 because the stack is used to store 16bit value/address
          //and the memory is 8bit
          //call to nn, SP=SP-2, (SP)=PC, PC=nn
          //TODO: CHECK HOW TO WRITE THE 2BYTES which endian to use?
        if condition {
            // moving stack pointer
            // OK !
            let instruction_byte_size = instruction.bytes;
            reg.sp = reg.sp - 2;
            bus.write_2_bytes_little_endian(reg.sp, reg.pc + instruction_byte_size as u16);
            self.next_pc = bus.get_a16_address(reg.pc);
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
            };
            if !conditional {
                self.cycles_to_wait = instruction.cycles[1] as u8;
            }
        }
        if condition {
            let next_byte = bus.read_next_byte(reg.pc);
            let (next_pc, _, _) = signed_addition(reg.pc, next_byte);
            //TODO: I think its adding not from the pc position at the begining of the opreation
            // but bytes + after
            self.next_pc = next_pc + instruction.bytes as u16;
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
            };
            if !condition {
                self.cycles_to_wait = instruction.cycles[1] as u8;
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
            self.next_pc = new_pc;
        }
    }
    fn op_ret(&mut self, instruction: Instruction) {
        let bus = self.bus.borrow_mut();
        let reg = &mut self.reg;
        let operand = instruction.operands.get(0);
        let conditional = instruction.cycles.len() == 2;

        let condition = match operand {
            Some(operand_type) => match operand_type.name.clone().into() {
                NopreOperands::Z => reg.flag_z(),
                NopreOperands::NZ => !reg.flag_z(),
                NopreOperands::C => reg.flag_c(),
                NopreOperands::NC => !reg.flag_c(),
                NopreOperands::INVALID => panic!("Invalid operand"),
                _ => panic!("Missing operand for add 8bit?"),
            },
            None => true,
        };
        if condition {
            self.next_pc = bus.read_2_bytes_little_endian(reg.sp);
            // OK
            reg.sp += 2;
        }
        if conditional && !condition {
            self.cycles_to_wait = instruction.cycles[1] as u8;
        }
    }
    fn op_cp_8bit(&mut self, instruction: Instruction) {
        let target_operand = instruction.operands[1].name.clone().into();
        let bus = self.bus.borrow_mut();
        let reg = &mut self.reg;
        let a = reg.get_a();
        let instruction_byte_size = instruction.bytes as u16;
        let result = match target_operand {
            NopreOperands::A
            | NopreOperands::B
            | NopreOperands::C
            | NopreOperands::D
            | NopreOperands::E
            | NopreOperands::H => compare(a, reg.get(&target_operand)),
            NopreOperands::HL => compare(a, bus.read_byte_as_cpu(reg.hl)),
            NopreOperands::n8 => compare(a, bus.read_next_byte(reg.pc)),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
        let (z, n, h, c) = result;
        reg.set_flags(z, n, h, c);
    }
    fn op_adc_sbc_8bit<F>(&mut self, instruction: Instruction, f: F)
    where
        F: Fn(u8, u8, bool) -> (u8, bool, bool, bool, bool),
    {
        let target_operand = instruction.operands[1].name.clone().into();
        let bus = self.bus.borrow_mut();
        let reg = &mut self.reg;
        let a = reg.get_a();
        let carry = reg.flag_c();
        let result = match target_operand {
            NopreOperands::A
            | NopreOperands::B
            | NopreOperands::C
            | NopreOperands::D
            | NopreOperands::E
            | NopreOperands::L
            | NopreOperands::H => f(a, reg.get(&target_operand), carry),
            NopreOperands::HL => f(a, bus.read_byte_as_cpu(reg.hl), carry),
            NopreOperands::n8 => f(a, bus.read_next_byte(reg.pc), carry),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
        let (new_a, z, n, h, c) = result;
        reg.set_flags(z, n, h, c);
        reg.set_a(new_a);
    }

    fn op_add_sub_bit_8bit<F>(&mut self, instruction: Instruction, f: F)
    where
        F: Fn(u8, u8) -> (u8, bool, bool, bool, bool),
    {
        // println!("Instruction: {:?}", instruction);
        let target_operand = instruction.operands[1].name.clone().into();
        let bus = self.bus.borrow_mut();
        let reg = &mut self.reg;
        let a = reg.get_a();
        // println!("(HL): {:#04x}",bus.read_byte_as_cpu(reg.hl));
        let result = match target_operand {
            NopreOperands::A
            | NopreOperands::B
            | NopreOperands::C
            | NopreOperands::D
            | NopreOperands::E
            | NopreOperands::L
            | NopreOperands::H => f(a, reg.get(&target_operand)),
            NopreOperands::HL => f(a, bus.read_byte_as_cpu(reg.hl)),
            NopreOperands::n8 => f(a, bus.read_next_byte(reg.pc)),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
        let (new_a, z, n, h, c) = result;
        // println!("New a: {:#04x}", new_a);
        reg.set_flags(z, n, h, c);
        reg.set_a(new_a);
    }

    // Filter ADD INC DEC that have both 8 and 16 bit version
    fn op_arith(&mut self, instruction: Instruction, opcode: u8) {
        //Dispatch 8bit and 16bit load instruction
        match opcode {
            0x03 | 0x13 | 0x23 | 0x33 | 0x09 | 0x19 | 0x29 | 0x39 | 0x0b | 0x1b | 0x2b | 0x3b => {
                self.op_arith_16bit(instruction)
            }
            0xe8 => {
                let reg = &mut self.reg;
                let bus = self.bus.borrow();
                let (result, h, c) = signed_addition(reg.sp, bus.read_next_byte(reg.pc));
                reg.sp = result;
                reg.set_flag_z(false);
                reg.set_flag_n(false);
                reg.set_flag_h(h);
                reg.set_flag_c(c);
            }
            _ => self.op_arith_8bit(instruction),
        }
    }

    fn op_arith_16bit(&mut self, instruction: Instruction) {
        let mnemonic: NopreOpcodeMnemonics = instruction.mnemonic.clone().into();
        match mnemonic {
            NopreOpcodeMnemonics::ADD => self.op_add_16bit(instruction),
            NopreOpcodeMnemonics::INC => self.op_inc_16bit(instruction),
            NopreOpcodeMnemonics::DEC => self.op_dec_16bit(instruction),
            NopreOpcodeMnemonics::INVALID => panic!("INVALID operand arith_16bit"),
            _ => panic!("No operand match"),
        };
    }

    fn op_arith_8bit(&mut self, instruction: Instruction) {
        let mnemonic: NopreOpcodeMnemonics = instruction.mnemonic.clone().into();
        let reg = &mut self.reg;

        match mnemonic {
            NopreOpcodeMnemonics::AND => self.op_add_sub_bit_8bit(instruction, and),
            NopreOpcodeMnemonics::OR => self.op_add_sub_bit_8bit(instruction, or),
            NopreOpcodeMnemonics::XOR => self.op_add_sub_bit_8bit(instruction, xor),
            NopreOpcodeMnemonics::ADD => self.op_add_sub_bit_8bit(instruction, addition),
            NopreOpcodeMnemonics::SUB => self.op_add_sub_bit_8bit(instruction, subtraction),
            NopreOpcodeMnemonics::ADC => self.op_adc_sbc_8bit(instruction, adc),
            NopreOpcodeMnemonics::SBC => self.op_adc_sbc_8bit(instruction, sbc),
            NopreOpcodeMnemonics::DEC => self.op_inc_dec_8bit(instruction, dec),
            NopreOpcodeMnemonics::INC => self.op_inc_dec_8bit(instruction, inc),
            NopreOpcodeMnemonics::CP => self.op_cp_8bit(instruction),
            NopreOpcodeMnemonics::DAA => {
                let (res, z, c) = daa(reg.get_a(), reg.flag_h(), reg.flag_c());
                reg.set_a(res);
                reg.set_flag_z(z);
                reg.set_flag_c(c);
            }
            NopreOpcodeMnemonics::CPL => {
                reg.set_a(complement(reg.get_a()));
                reg.set_flag_n(true);
                reg.set_flag_h(true);
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
        let mut bus = self.bus.borrow_mut();
        let reg = &mut self.reg;

        let value = match from_type {
            NopreOperands::INVALID => panic!("Invalid LD operands"),
            NopreOperands::HL => reg.hl,
            NopreOperands::SP => {
                match opcode {
                    0xF8 => {
                        // TODO: flags h check
                        let (result, h, c) = signed_addition(reg.sp, bus.read_next_byte(reg.pc));
                        reg.set_flag_z(false);
                        reg.set_flag_n(false);
                        reg.set_flag_h(h);
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
            // WARN: not used at least before cycle 32535
            NopreOperands::a16 => bus.write_a16(reg.pc, value),

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
        let instruction_byte_size = instruction.bytes as u16;

        let value = match from_type {
            NopreOperands::A
            | NopreOperands::B
            | NopreOperands::D
            | NopreOperands::E
            | NopreOperands::H
            | NopreOperands::L => reg.get(&from_type),
            // NopreOperands::a8 => bus.read_next_byte(reg.pc + instruction_byte_size),
            // WARN: check a8 not what is described
            NopreOperands::a8 => bus.read_a8(bus.read_next_byte(reg.pc)),
            NopreOperands::BC => bus.read_byte_as_cpu(reg.bc),
            //check if c or [c] with bytes?,
            NopreOperands::C => {
                if from.immediate {
                    reg.get_c()
                } else {
                    bus.read_byte_as_cpu(0xFF00 + reg.get_c() as u16)
                }
            }
            NopreOperands::DE => bus.read_byte_as_cpu(reg.de),
            NopreOperands::HL => {
                let hl_mem = reg.hl.clone();
                if from.increment.is_some() {
                    reg.hl_plus();
                } else if from.decrement.is_some() {
                    reg.hl_minus();
                }
                bus.read_byte_as_cpu(hl_mem)
            }
            NopreOperands::n8 => bus.read_next_byte(reg.pc),
            NopreOperands::a16 => bus.get_a16_value(reg.pc),
            NopreOperands::INVALID => panic!("Invalid LD operands"),
            _ => panic!("Missing operand"),
        };
        match into_type {
            NopreOperands::A => reg.set_a(value),
            // Ok!
            NopreOperands::a16 => {
                let a16 = bus.get_a16_address(reg.pc);
                bus.write_byte_as_cpu(a16, value)
            }
            // Ok!
            NopreOperands::a8 => {
                let a8_add = get_a8_address(bus.read_next_byte(reg.pc));
                bus.write_byte_as_cpu(a8_add, value)
            }
            NopreOperands::B => reg.set_b(value),
            //check if c or [c] with bytes?,
            NopreOperands::C => {
                if into.immediate {
                    reg.set_c(value)
                } else {
                    bus.write_byte_as_cpu(0xFF00 + reg.get_c() as u16, value)
                }
            }
            NopreOperands::D => reg.set_d(value),
            NopreOperands::E => reg.set_e(value),
            NopreOperands::H => reg.set_h(value),
            NopreOperands::HL => {
                // TODO: extract logic
                bus.write_byte_as_cpu(reg.hl, value);
                if into.increment.is_some() {
                    reg.hl_plus();
                } else if into.decrement.is_some() {
                    reg.hl_minus();
                }
            }
            NopreOperands::BC => bus.write_byte_as_cpu(reg.bc, value),
            NopreOperands::DE => bus.write_byte_as_cpu(reg.de, value),
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
            NopreOperands::HL => f(bus.read_byte_as_cpu(reg.hl), bit_number),
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
            NopreOperands::HL => f(bus.read_byte_as_cpu(reg.hl), bit_number),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for res/set ?"),
        };
    }
    fn op_pre_1arg_with_carry<F>(&mut self, instruction: Instruction, f: F)
    where
        F: Fn(u8, bool) -> (u8, bool, bool, bool, bool),
    {
        let operand_type = instruction.operands[0].name.clone().into();
        let mut bus = self.bus.borrow_mut();
        let reg = &mut self.reg;
        let carry = reg.flag_c();
        let result = match operand_type {
            NopreOperands::A => f(reg.get_a(), carry),
            NopreOperands::B => f(reg.get_b(), carry),
            NopreOperands::C => f(reg.get_c(), carry),
            NopreOperands::D => f(reg.get_d(), carry),
            NopreOperands::E => f(reg.get_e(), carry),
            NopreOperands::H => f(reg.get_h(), carry),
            NopreOperands::L => f(reg.get_l(), carry),
            NopreOperands::HL => f(bus.read_byte_as_cpu(reg.hl), carry),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
        let (new_value, z, n, h, c) = result;
        reg.set_flags(z, n, h, c);
        match operand_type {
            NopreOperands::A => reg.set_a(new_value),
            NopreOperands::B => reg.set_b(new_value),
            NopreOperands::C => reg.set_c(new_value),
            NopreOperands::D => reg.set_d(new_value),
            NopreOperands::E => reg.set_e(new_value),
            NopreOperands::H => reg.set_h(new_value),
            NopreOperands::L => reg.set_l(new_value),
            NopreOperands::HL => bus.write_byte_as_cpu(reg.hl, new_value),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
    }

    // prefixed instruction with only one arg (RLC, RRC...)
    fn op_pre_1arg_carryless<F>(&mut self, instruction: Instruction, f: F)
    where
        F: Fn(u8) -> (u8, bool, bool, bool, bool),
    {
        let operand_type = instruction.operands[0].name.clone().into();
        let mut bus = self.bus.borrow_mut();
        let reg = &mut self.reg;
        let result = match operand_type {
            NopreOperands::A => f(reg.get_a()),
            NopreOperands::B => f(reg.get_b()),
            NopreOperands::C => f(reg.get_c()),
            NopreOperands::D => f(reg.get_d()),
            NopreOperands::E => f(reg.get_e()),
            NopreOperands::H => f(reg.get_h()),
            NopreOperands::L => f(reg.get_l()),
            NopreOperands::HL => f(bus.read_byte_as_cpu(reg.hl)),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
        let (new_value, z, n, h, c) = result;
        reg.set_flags(z, n, h, c);
        match operand_type {
            NopreOperands::A => reg.set_a(new_value),
            NopreOperands::B => reg.set_b(new_value),
            NopreOperands::C => reg.set_c(new_value),
            NopreOperands::D => reg.set_d(new_value),
            NopreOperands::E => reg.set_e(new_value),
            NopreOperands::H => reg.set_h(new_value),
            NopreOperands::L => reg.set_l(new_value),
            NopreOperands::HL => bus.write_byte_as_cpu(reg.hl, new_value),
            NopreOperands::INVALID => panic!("Invalid operand"),
            _ => panic!("Missing operand for add 8bit?"),
        };
    }
    fn check_for_interupt(&mut self) {
        // println!(
        //     "Check interupt current interupt: {}, ime: {}, interupt flag: {}, interupt enable: {}",
        //     self.current_interupt.is_none(),
        //     self.ime,
        //     self.interupt_reg.get_interupt_flag(),
        //     self.interupt_reg.get_interupt_enable()
        // );
        if self.current_interupt.is_none() {
            if self.ime {
                self.current_interupt = self.interupt_reg.query_interupts_flag_enable();
            }
        }
        if self.current_interupt.is_some() {
            println!(
                "LET'S GOO INTERUPT : {:?}",
                self.current_interupt.as_ref().unwrap()
            );
            // push current pc on stack
            let reg = &mut self.reg;

            reg.sp = reg.sp - 2;
            self.bus
                .borrow_mut()
                .write_2_bytes_little_endian(reg.sp, reg.pc);

            let current_interupt = self.current_interupt.as_ref().unwrap();
            // set pc to correct address
            self.next_pc = match current_interupt {
                InteruptType::VBlank => 0x0040,
                InteruptType::LCD => 0x0048,
                InteruptType::Timer => 0x0050,
                InteruptType::Serial => 0x0058,
                InteruptType::Joypad => 0x0060,
            };
            // TODO: I don't understand but apparenty I need this?
            // self.next_pc += 1;

            // println!("next_pc in check interupt : {:#04x}", self.next_pc);
            self.ime = false;
            self.interupt_reg.reset_flag(current_interupt);
            self.interupt_stage = 0;
            self.current_interupt = None;
            self.interupt_happened = true;

            // TODO: for now removed, do all in one step as it seems to be like this in the
            // the test interupt file

            // self.interupt_stage += 1;
            //
            // Set a "interupt step x var"
            // step 1 nop
            // step 2 nop
            // step 3 push
            // step 4 pc = interupt address
            // Clear flag?
            //
            // match self.interupt_stage {
            //     1 => {
            //         self.ime = false;
            //         self.opcode = 0x00;
            //     }
            //     2 => {
            //         self.opcode = 0x00;
            //     }
            //     3 => {
            //         // HACK: A third nop that shouldn't happen to allow for 2 cycles to pass
            //         // While pushing manually
            //         // Idea: use unused opcode to add one that pushes pc to sp
            //
            //         // self.opcode = 0x00;
            //
            //         // push current
            //         let reg = &mut self.reg;
            //         let mut bus = self.bus.borrow_mut();
            //         reg.sp = reg.sp - 2;
            //         bus.write_2_bytes_little_endian(reg.sp, reg.pc);
            //     }
            //     4 => {
            //         self.next_pc = match self.current_interupt.as_ref().unwrap() {
            //             InteruptType::VBlank => 0x0040,
            //             InteruptType::LCD => 0x0048,
            //             InteruptType::Timer => 0x0050,
            //             InteruptType::Serial => 0x0058,
            //             InteruptType::Joypad => 0x0060,
            //         };
            //         self.interupt_stage = 0;
            //         self.current_interupt = None;
            //     }
            //     _ => panic!("Interupt stage >4 should be impossible"),
            // }
        }
    }
    fn init_log_file(&mut self, file_path: &str) {
        // Open the file with append mode
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true) // Create the file if it doesn't exist
            .open(file_path)
            .unwrap();
        // Truncate the file to zero length, effectively erasing its contents
        file.set_len(0).unwrap();
        // Wrap the file in a BufWriter for better performance
        self.log_buffer = Some(io::BufWriter::new(file));
    }
    fn log_state_to_file(&mut self) {
        match &mut self.log_buffer {
            Some(buf_writer) => {
                let reg = &self.reg;
                let bus = self.bus.borrow();
                let mut text = format!(
                    "A:{:#04x} F:{:#04x} B:{:#04x} C:{:#04x} D:{:#04x} E:{:#04x} H:{:#04x} L:{:#04x} SP:{:#06x} PC:{:#06x} PCMEM:{:#04x},{:#04x},{:#04x},{:#04x}",
                    reg.get_a(),
                    reg.get_f(),
                    reg.get_b(),
                    reg.get_c(),
                    reg.get_d(),
                    reg.get_e(),
                    reg.get_h(),
                    reg.get_l(),
                    reg.sp,
                    reg.pc,
                    bus.read_byte_as_cpu(reg.pc),
                    bus.read_byte_as_cpu(reg.pc.wrapping_add(1)),
                    bus.read_byte_as_cpu(reg.pc.wrapping_add(2)),
                    bus.read_byte_as_cpu(reg.pc.wrapping_add(3)),
                );
                text = text.to_string().replace("0x", "");
                text = text.to_uppercase();
                // Append the line to the file
                writeln!(buf_writer, "{}", text).unwrap();
                // Ensure the data is written to disk
                let _ = buf_writer.flush();
            }
            None => (),
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
            cpu2.bus.borrow_mut().write_byte_as_cpu(0xFF00, 7);
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
