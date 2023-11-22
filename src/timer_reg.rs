use crate::{bus::Bus, util::u8_traits::Bit};
use std::{cell::RefCell, rc::Rc};

pub struct TimerReg {
    bus: Rc<RefCell<Bus>>,
    tick_counter: u64,
}
impl TimerReg {
    pub fn new(bus: Rc<RefCell<Bus>>) -> TimerReg {
        TimerReg {
            bus,
            tick_counter: 0,
        }
    }
    pub fn next_tick(&mut self) {
        self.tick_counter = self.tick_counter.wrapping_add(1);
        self.div_tick();
        self.tima_tick();
    }
    fn tima_tick(&mut self) {
        if self.get_tac_ff07().get_bit(2) {
            if self.tick_counter % self.get_tima_rate() == 0 {
                let tima = self.get_tima_ff05();
                // When overflow reset to value of ff06
                let new_tima = if tima == 255 {
                    self.get_tma_ff06()
                } else {
                    tima + 1
                };
                // Request timer interrupt
                println!(
                    "TIMER REQUEST INTERRUPT, div {}, tima {}, tma {}, tac {}",
                    self.get_div_ff04(),
                    self.get_tima_ff05(),
                    self.get_tma_ff06(),
                    self.get_tac_ff07()
                );
                self.bus.borrow_mut().write_bit(0xff0f, 2, true);
                self.bus.borrow_mut().write_byte(0xff05, new_tima);
            }
        }
    }
    fn div_tick(&mut self) {
        if self.tick_counter % 256 == 0 {
            let new_div = self.get_div_ff04().wrapping_add(1);
            self.bus.borrow_mut().write_byte(0xff04, new_div);
        }
    }
    fn get_tima_rate(&self) -> u64 {
        let byte = self.get_tac_ff07();
        let bits = (byte.get_bit(1), byte.get_bit(0));
        match bits {
            (true, true) => 256,
            (true, false) => 64,
            (false, true) => 16,
            (false, false) => 1024,
        }
    }
    // divider
    pub fn get_div_ff04(&self) -> u8 {
        self.bus.borrow().read_byte(0xFF04)
    }
    // timer counter
    pub fn get_tima_ff05(&self) -> u8 {
        self.bus.borrow().read_byte(0xFF05)
    }
    // timer modulo
    pub fn get_tma_ff06(&self) -> u8 {
        self.bus.borrow().read_byte(0xFF06)
    }
    // timer control
    pub fn get_tac_ff07(&self) -> u8 {
        self.bus.borrow().read_byte(0xFF07)
    }
}
