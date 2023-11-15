use std::{cell::RefCell, rc::Rc, time::Instant};

use crate::{
    bus::Bus,
    screen::{Screen, GAMEBOY_SCREEN_HEIGHT, GAMEBOY_SCREEN_WIDTH},
};

pub enum PPUModes {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
}
pub enum VideoMemBlock {
    All,
    OAM,
    None,
}

#[warn(dead_code)]
const MODE_2_DOTS: usize = 80usize;
const MODE_3_DOTS_MIN: usize = 172usize;
const MODE_3_DOTS_MAX: usize = 289usize;
const MODE_0_DOTS_MIN: usize = 87usize;
const MODE_0_DOTS_MAX: usize = 204usize;
const MODE_1_DOTS: usize = 4560usize;

pub struct PPU {
    pub bus: Rc<RefCell<Bus>>,
    pub screen_array: [[u8; GAMEBOY_SCREEN_WIDTH]; GAMEBOY_SCREEN_HEIGHT],
    pub current_mode: PPUModes,
    pub dots_counter: usize,
}
impl PPU {
    pub fn new(bus: Rc<RefCell<Bus>>) -> PPU {
        PPU {
            bus,
            screen_array: [[0; GAMEBOY_SCREEN_WIDTH]; GAMEBOY_SCREEN_HEIGHT],
            current_mode: PPUModes::Mode0,
            dots_counter: 0,
        }
    }
    pub fn next_tick(&mut self) {
        self.dots_counter += 4;
    }
    fn tick_mode(&self) {
        match self.current_mode {
            PPUModes::Mode2 => todo!(),
            PPUModes::Mode3 => todo!(),
            PPUModes::Mode0 => todo!(),
            PPUModes::Mode1 => todo!(),
        }
    }
    fn block_memory(&self) {}

    fn draw() {}
}
