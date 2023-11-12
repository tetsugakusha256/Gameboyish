use std::{cell::RefCell, rc::Rc};

use crate::{bus::Bus, register::Registers};

/// 8bit op target
pub enum RegTarget {
    A,
    B,
    C,
    Cv,
    D,
    E,
    H,
    L,
    BCv,
    DEv,
    HLv,
    HLp,
    HLm,
    N8,
    A8,
    A16,
}
pub struct CPU {
    pub reg: Registers,
    pub bus: Rc<RefCell<Bus>>,
    cycles_since_last_cmd: u64,
    cycles_to_wait: u64,
}
impl CPU {
    pub fn new(bus: Rc<RefCell<Bus>>) -> CPU {
        CPU {
            reg: Registers::new(),
            bus, 
            cycles_to_wait: 0,
            cycles_since_last_cmd: 0,
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
    pub fn op_add_8bit(&mut self, left: RegTarget, right: RegTarget) {}
    pub fn op_load_reg(&mut self, into: RegTarget, from: RegTarget) {
        let from = match from {
            RegTarget::A => self.reg.get_a(),
            RegTarget::B => self.reg.get_b(),
            RegTarget::C => self.reg.get_c(),
            RegTarget::D => self.reg.get_d(),
            RegTarget::E => self.reg.get_e(),
            RegTarget::H => self.reg.get_h(),
            RegTarget::L => self.reg.get_l(),
            RegTarget::BCv => todo!(),
            RegTarget::DEv => todo!(),
            RegTarget::HLv => todo!(),
            RegTarget::N8 => todo!(),
            RegTarget::A8 => todo!(),
            RegTarget::A16 => todo!(),
            RegTarget::Cv => todo!(),
            RegTarget::HLp => todo!(),
            RegTarget::HLm => todo!(),
        };
        match into {
            RegTarget::A => self.reg.set_a(from),
            RegTarget::B => self.reg.set_b(from),
            RegTarget::C => self.reg.set_c(from),
            RegTarget::D => self.reg.set_d(from),
            RegTarget::E => self.reg.set_e(from),
            RegTarget::H => self.reg.set_h(from),
            RegTarget::L => self.reg.set_l(from),
            RegTarget::BCv => todo!(),
            RegTarget::DEv => todo!(),
            RegTarget::HLv => todo!(),
            RegTarget::N8 => todo!(),
            RegTarget::A8 => todo!(),
            RegTarget::A16 => todo!(),
            RegTarget::Cv => todo!(),
            RegTarget::HLp => todo!(),
            RegTarget::HLm => todo!(),
        }
    }
    pub fn tick(&mut self, opcode: u8) {
        let af = self.reg.af;
        let de = self.reg.de;
        let bc = self.reg.bc;
        let hl = self.reg.hl;
        let sp = self.reg.sp;
        let pc = self.reg.pc;
        match opcode {
            0x00 => {}

            0x02 => {
                self.op_load_reg(RegTarget::BCv, RegTarget::A);
            }
            0x06 => {
                self.op_load_reg(RegTarget::B, RegTarget::N8);
            }
            0x0a => {
                self.op_load_reg(RegTarget::A, RegTarget::BCv);
            }
            0x0e => {
                self.op_load_reg(RegTarget::C, RegTarget::N8);
            }
            0x12 => {
                self.op_load_reg(RegTarget::DEv, RegTarget::A);
            }
            0x16 => {
                self.op_load_reg(RegTarget::D, RegTarget::N8);
            }
            0x1a => {
                self.op_load_reg(RegTarget::A, RegTarget::DEv);
            }
            0x1e => {
                self.op_load_reg(RegTarget::E, RegTarget::N8);
            }
            0x22 => {
                self.op_load_reg(RegTarget::HLp, RegTarget::A);
            }
            0x26 => {
                self.op_load_reg(RegTarget::H, RegTarget::N8);
            }
            0x2a => {
                self.op_load_reg(RegTarget::A, RegTarget::HLp);
            }
            0x2e => {
                self.op_load_reg(RegTarget::L, RegTarget::N8);
            }
            0x32 => {
                self.op_load_reg(RegTarget::HLm, RegTarget::A);
            }
            0x36 => {
                self.op_load_reg(RegTarget::HLv, RegTarget::N8);
            }
            0x3a => {
                self.op_load_reg(RegTarget::A, RegTarget::HLm);
            }
            0x3e => {
                self.op_load_reg(RegTarget::A, RegTarget::N8);
            }

            0x40 => {
                self.op_load_reg(RegTarget::B, RegTarget::B);
            }
            0x41 => {
                self.op_load_reg(RegTarget::B, RegTarget::C);
            }
            0x42 => {
                self.op_load_reg(RegTarget::B, RegTarget::D);
            }
            0x43 => {
                self.op_load_reg(RegTarget::B, RegTarget::E);
            }
            0x44 => {
                self.op_load_reg(RegTarget::B, RegTarget::H);
            }
            0x45 => {
                self.op_load_reg(RegTarget::B, RegTarget::L);
            }
            0x46 => {
                self.op_load_reg(RegTarget::B, RegTarget::HLv);
            }
            0x47 => {
                self.op_load_reg(RegTarget::B, RegTarget::A);
            }
            0x48 => {
                self.op_load_reg(RegTarget::C, RegTarget::B);
            }
            0x49 => {
                self.op_load_reg(RegTarget::C, RegTarget::C);
            }
            0x4a => {
                self.op_load_reg(RegTarget::C, RegTarget::D);
            }
            0x4b => {
                self.op_load_reg(RegTarget::C, RegTarget::E);
            }
            0x4c => {
                self.op_load_reg(RegTarget::C, RegTarget::H);
            }
            0x4d => {
                self.op_load_reg(RegTarget::C, RegTarget::L);
            }
            0x4e => {
                self.op_load_reg(RegTarget::C, RegTarget::HLv);
            }
            0x4f => {
                self.op_load_reg(RegTarget::C, RegTarget::A);
            }
            0x50 => {
                self.op_load_reg(RegTarget::D, RegTarget::B);
            }
            0x51 => {
                self.op_load_reg(RegTarget::D, RegTarget::C);
            }
            0x52 => {
                self.op_load_reg(RegTarget::D, RegTarget::D);
            }
            0x53 => {
                self.op_load_reg(RegTarget::D, RegTarget::E);
            }
            0x54 => {
                self.op_load_reg(RegTarget::D, RegTarget::H);
            }
            0x55 => {
                self.op_load_reg(RegTarget::D, RegTarget::L);
            }
            0x56 => {
                self.op_load_reg(RegTarget::D, RegTarget::HLv);
            }
            0x57 => {
                self.op_load_reg(RegTarget::D, RegTarget::A);
            }
            0x58 => {
                self.op_load_reg(RegTarget::E, RegTarget::B);
            }
            0x59 => {
                self.op_load_reg(RegTarget::E, RegTarget::C);
            }
            0x5a => {
                self.op_load_reg(RegTarget::E, RegTarget::D);
            }
            0x5b => {
                self.op_load_reg(RegTarget::E, RegTarget::E);
            }
            0x5c => {
                self.op_load_reg(RegTarget::E, RegTarget::H);
            }
            0x5d => {
                self.op_load_reg(RegTarget::E, RegTarget::L);
            }
            0x5e => {
                self.op_load_reg(RegTarget::E, RegTarget::HLv);
            }
            0x5f => {
                self.op_load_reg(RegTarget::E, RegTarget::A);
            }
            0x60 => {
                self.op_load_reg(RegTarget::H, RegTarget::B);
            }
            0x61 => {
                self.op_load_reg(RegTarget::H, RegTarget::C);
            }
            0x62 => {
                self.op_load_reg(RegTarget::H, RegTarget::D);
            }
            0x63 => {
                self.op_load_reg(RegTarget::H, RegTarget::E);
            }
            0x64 => {
                self.op_load_reg(RegTarget::H, RegTarget::H);
            }
            0x65 => {
                self.op_load_reg(RegTarget::H, RegTarget::L);
            }
            0x66 => {
                self.op_load_reg(RegTarget::H, RegTarget::HLv);
            }
            0x67 => {
                self.op_load_reg(RegTarget::H, RegTarget::A);
            }
            0x68 => {
                self.op_load_reg(RegTarget::L, RegTarget::B);
            }
            0x69 => {
                self.op_load_reg(RegTarget::L, RegTarget::C);
            }
            0x6a => {
                self.op_load_reg(RegTarget::L, RegTarget::D);
            }
            0x6b => {
                self.op_load_reg(RegTarget::L, RegTarget::E);
            }
            0x6c => {
                self.op_load_reg(RegTarget::L, RegTarget::H);
            }
            0x6d => {
                self.op_load_reg(RegTarget::L, RegTarget::L);
            }
            0x6e => {
                self.op_load_reg(RegTarget::L, RegTarget::HLv);
            }
            0x6f => {
                self.op_load_reg(RegTarget::L, RegTarget::A);
            }
            0x70 => {
                self.op_load_reg(RegTarget::HLv, RegTarget::B);
            }
            0x71 => {
                self.op_load_reg(RegTarget::HLv, RegTarget::C);
            }
            0x72 => {
                self.op_load_reg(RegTarget::HLv, RegTarget::D);
            }
            0x73 => {
                self.op_load_reg(RegTarget::HLv, RegTarget::E);
            }
            0x74 => {
                self.op_load_reg(RegTarget::HLv, RegTarget::H);
            }
            0x75 => {
                self.op_load_reg(RegTarget::HLv, RegTarget::L);
            }
            //Halt
            0x76 => {
                todo!();
            }
            0x77 => {
                self.op_load_reg(RegTarget::HLv, RegTarget::A);
            }
            0x78 => {
                self.op_load_reg(RegTarget::A, RegTarget::B);
            }
            0x79 => {
                self.op_load_reg(RegTarget::A, RegTarget::C);
            }
            0x7a => {
                self.op_load_reg(RegTarget::A, RegTarget::D);
            }
            0x7b => {
                self.op_load_reg(RegTarget::A, RegTarget::E);
            }
            0x7c => {
                self.op_load_reg(RegTarget::A, RegTarget::H);
            }
            0x7d => {
                self.op_load_reg(RegTarget::A, RegTarget::L);
            }
            0x7e => {
                self.op_load_reg(RegTarget::A, RegTarget::HLv);
            }
            0x7f => {
                self.op_load_reg(RegTarget::A, RegTarget::A);
            }

            0xe0 => {
                self.op_load_reg(RegTarget::A8, RegTarget::A);
            }
            0xf0 => {
                self.op_load_reg(RegTarget::A, RegTarget::A8);
            }

            0xe2 => {
                self.op_load_reg(RegTarget::Cv, RegTarget::A);
            }
            0xf2 => {
                self.op_load_reg(RegTarget::A, RegTarget::Cv);
            }

            0xea => {
                self.op_load_reg(RegTarget::A16, RegTarget::A);
            }
            0xfa => {
                self.op_load_reg(RegTarget::A, RegTarget::A16);
            }
            // 0x02 => {self.op_load(from, into)}
            _ => unreachable!(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::CPU;
    use crate::{bus::Bus, register::Registers};
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn bus_access() {
        let bus = Rc::new(RefCell::new(Bus::new()));
        let cpu2 = CPU {
            reg: Registers::new(),
            bus: Rc::clone(&bus),
            cycles_since_last_cmd: 0,
            cycles_to_wait:0,
        };
        let cpu = CPU {
            reg: Registers::new(),
            bus: Rc::clone(&bus),
            cycles_since_last_cmd: 0,
            cycles_to_wait:0,
        };
        {
            cpu.bus.borrow_mut().write_slice(0x0010, &[1, 2, 3]);
            let binding = cpu.bus.borrow();
            let slice = binding.read_bytes_range(0x0010, 3);
            assert_eq!(slice, &[1, 2, 3]);
        }
        {
            cpu2.bus.borrow_mut().write_bytes(0xFF00, 7);
            let binding = cpu2.bus.borrow();
            let slice = binding.read_bytes_range(0xFF00, 3);
            assert_eq!(slice, &[7, 0, 0]);
        }
    }
}
