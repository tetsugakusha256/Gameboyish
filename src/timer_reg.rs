use crate::{
    bus::Bus,
    util::u8_traits::{Bit, NibblesU16, NibblesU8},
};
use std::{cell::RefCell, rc::Rc};

pub struct TimerReg {
    bus: Rc<RefCell<Bus>>,
}
impl TimerReg {
    pub fn new(bus: Rc<RefCell<Bus>>) -> TimerReg {
        TimerReg { bus }
    }
    pub fn next_tick(&mut self) {
        {
            let mut bus = self.bus.borrow_mut();
            bus.timer_div_intern = bus.timer_div_intern.wrapping_add(1);
        }
        self.div_tick();
        self.tima_tick();
    }
    // TODO: refactor
    fn tima_tick(&mut self) {
        // If tima is enable
        if self.get_tac_ff07().get_bit(2) {
            let tima_rate = self.get_tima_rate();
            let mut bus = self.bus.borrow_mut();
            bus.timer_tima_intern = bus.timer_tima_intern.wrapping_add(1);
            println!(
                "tima_rate {}, tima_intern {}",
                tima_rate, bus.timer_tima_intern
            );

            if bus.timer_tima_intern >= tima_rate {
                let mut tima = bus.read_byte(0xFF05);
                if tima != 0 {
                    tima = tima + 1;
                }

                // When overflow reset to value of ff06
                let tima = match tima {
                    255 => 0,
                    0 => {
                        // Setting flag
                        bus.write_bit(0xff0f, 2, true);
                        let a = bus.read_byte(0xff06);
                        if a == 0 {
                            1
                        } else {
                            a
                        }
                    }
                    _ => tima,
                };
                bus.write_byte(0xff05, tima);

                bus.timer_tima_intern -= tima_rate
            }
        }
    }
    fn div_tick(&mut self) {
        let timer_inter = self.bus.borrow().timer_div_intern;
        self.bus
            .borrow_mut()
            .write_byte(0xff04, timer_inter.high_8nibble());
    }
    fn get_tima_rate(&self) -> u16 {
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
