use std::{cell::RefCell, rc::Rc, time::Instant};

use crate::{
    bus::Bus,
    cpu::CPU,
    io_handler::IOHandler,
    ppu::PPU,
    quartz::Quartz,
    timer_reg::TimerReg,
    util::tiles_util::{vram_to_screen, ScreenVector},
    windows::game_window::GameWindow,
};
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
    pub quartz: Quartz,
    pub timer: TimerReg,
    pub state: EmulatorState,
    pub cycles: u64,
    pub screen: GameWindow,
    pub debug_screen: GameWindow,
}
impl Emulator {
    pub fn init(&mut self) {
        self.screen.init("Main", true);
        self.debug_screen.init("Debug",false);
        self.bus
            .borrow_mut()
            .load_cartridge("/home/anon/Documents/Code/GameBoyish/roms/test_roms/mts-20221022-1430-8d742b9/acceptance/bits/reg_f.gb")
            .unwrap();
        // Load boot rom
        self.bus.borrow_mut().init();

        // Activate logging
        // self.cpu.init_with_log();

        self.start();
    }
    pub fn start(&mut self) {
        self.state = EmulatorState::Running;
        self.main_loop();
    }

    pub fn stop(&mut self) {
        self.state = EmulatorState::Stopped;
    }

    pub fn pause_resume(&mut self) {
        let state = &self.state;
        self.state = match state {
            EmulatorState::Running => EmulatorState::Paused,
            EmulatorState::Paused => EmulatorState::Running,
            EmulatorState::Stopped => EmulatorState::Stopped,
        }
    }
    // main emulator loop
    fn main_loop(&mut self) {
        if self.state == EmulatorState::Running {
            loop {
                self.quartz.wait_till_next_tick();
                if self.state == EmulatorState::Running {
                    self.update_emulator_state();
                }
                if self.state == EmulatorState::Stopped {
                    break;
                }
            }
        }
    }
    // This function make calls every clock tick
    fn update_emulator_state(&mut self) {
        self.quartz.next_tick();
        self.cycles += 1;
        // TODO: Think where to put this because reading button is made in 2 step
        // put a bit to set if we want to check direction or buttons
        // then read the value (How many cycles in between those?)
        // self.io_handler.next_tick();
        self.timer.next_tick();
        self.cpu.next_tick();
        // println!("cycle : {}", self.cycles);
        self.ppu.next_tick();
        // self.bus.borrow_mut().write_slice(0x8000, &[0x56u8;8192]);

        if self.cycles % 70224 == 0 {
            self.debug_screen.next_tick(&vram_to_screen(
                Vec::from(self.bus.borrow().read_bytes_range(0x8000, 8192)),
                16,
            ));
            // println!("Screen_array: {:?}", &self.ppu.screen_array);
            self.screen.next_tick(&self.ppu.screen_array);
            // println!("Bg map : {:?}", self.bus.borrow().read_bytes_range(0x9800, 1024));
        }

        if self.cycles > 50000*50{
            self.pause_resume();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Emulator, EmulatorState, CPU};
    use crate::{
        bus::{Bus, VRAM},
        io_handler::IOHandler,
        ppu::PPU,
        quartz::Quartz,
        register::Registers,
        timer_reg::TimerReg,
        windows::game_window::GameWindow,
    };
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn multiple_bus_access() {
        let bus = Rc::new(RefCell::new(Bus::new()));
        let emu = Emulator {
            cpu: CPU::new(Rc::clone(&bus)),
            ppu: PPU::new(Rc::clone(&bus)),
            io_handler: IOHandler::new(Rc::clone(&bus)),
            timer: TimerReg::new(Rc::clone(&bus)),
            bus: Rc::clone(&bus),
            quartz: Quartz::new(),
            state: EmulatorState::Running,
            cycles: 0,
            screen: GameWindow::new(400, 400),
            debug_screen: GameWindow::new(500, 500),
        };
        {
            bus.borrow_mut().write_slice(0x0010, &[1, 2, 3]);
            let binding = emu.cpu.bus.borrow();
            let slice = binding.read_bytes_range(0x0010, 3);
            assert_eq!(slice, &[1, 2, 3]);
        }
        {
            bus.borrow_mut().write_slice(0x0010, &[1, 2, 3]);
            let binding = bus.borrow();
            let slice = binding.read_bytes_range(0x0010, 3);
            assert_eq!(slice, &[1, 2, 3]);
        }
        {
            bus.borrow_mut().write_byte_as_cpu(0x00A0, 5);
            let binding = bus.borrow();
            let val = binding.read_byte_as_cpu(0x00A0);
            assert_eq!(val, 5);
        }

        bus.borrow_mut().write_slice(0x8000, &[2u8; 8192]);
        let binding = bus.borrow();
        let val = binding.read_byte_as_cpu(0x8222);
        assert_eq!(val, 2);
    }
}
