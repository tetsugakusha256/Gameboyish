use std::{cell::RefCell, rc::Rc, time::Instant};

use crate::{bus::Bus, cpu::CPU, io_handler::IOHandler, ppu::PPU, screen::Screen, timer::Timer};
#[derive(PartialEq, Eq)]
pub enum EmulatorState {
    Running,
    Paused,
    Stopped,
}
pub struct Emulator {
    pub cpu: CPU,
    pub ppu: PPU,
    pub io_handler: IOHandler,
    pub bus: Rc<RefCell<Bus>>,
    pub timer: Timer,
    pub state: EmulatorState,
    pub cycles: u64,
    pub screen: Screen,
}
impl Emulator {
    pub fn init(&mut self) {
        self.screen.init();
        self.bus.borrow_mut().init();
        self.bus
            .borrow_mut()
            .load_cartridge("/home/anon/Documents/Code/GameBoyish/roms/Tetris (JUE) (V1.1) [!].gb")
            .unwrap();
        self.cpu.init_with_log();
        self.start();
    }
    pub fn start(&mut self) {
        self.state = EmulatorState::Running;
        self.main_loop();
    }

    pub fn stop(&mut self) {
        self.state = EmulatorState::Stopped;
    }

    pub fn pause_resume() {}
    // main emulator loop
    fn main_loop(&mut self) {
        if self.state == EmulatorState::Running {
            loop {
                self.update_emulator_state();
                if self.state != EmulatorState::Running {
                    break;
                }
            }
        }
    }
    // This function make calls every clock tick
    fn update_emulator_state(&mut self) {
        if self.timer.check_next_tick() {
            self.timer.next_tick();
            self.cycles += 1;
            // TODO: Think where to put this because reading button is made in 2 step
            // put a bit to set if we want to check direction or buttons
            // then read the value (How many cycles in between those?)
            self.io_handler.next_tick();
            self.cpu.next_tick();
            self.ppu.next_tick();
            self.screen.next_tick();
        }
        if self.cycles > 10000{
            self.stop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Emulator, EmulatorState, CPU};
    use crate::{
        bus::Bus, io_handler::IOHandler, ppu::PPU, register::Registers, screen::Screen,
        timer::Timer,
    };
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn multiple_bus_access() {
        let bus = Rc::new(RefCell::new(Bus::new()));
        let emu = Emulator {
            cpu: CPU::new(Rc::clone(&bus)),
            ppu: PPU::new(Rc::clone(&bus)),
            io_handler: IOHandler {
                bus: Rc::clone(&bus),
            },
            bus,
            timer: Timer::new(),
            state: EmulatorState::Running,
            cycles: 0,
            screen: Screen::new(400, 400),
        };
        {
            emu.cpu.bus.borrow_mut().write_slice(0x0010, &[1, 2, 3]);
            let binding = emu.cpu.bus.borrow();
            let slice = binding.read_bytes_range(0x0010, 3);
            assert_eq!(slice, &[1, 2, 3]);
        }
        {
            emu.ppu.bus.borrow_mut().write_slice(0x0010, &[1, 2, 3]);
            let binding = emu.ppu.bus.borrow();
            let slice = binding.read_bytes_range(0x0010, 3);
            assert_eq!(slice, &[1, 2, 3]);
        }
        {
            emu.ppu.bus.borrow_mut().write_byte(0x00A0, 5);
            let binding = emu.cpu.bus.borrow();
            let val = binding.read_byte(0x00A0);
            assert_eq!(val, 5);
        }
    }
}
