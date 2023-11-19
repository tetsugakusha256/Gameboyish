use std::{cell::RefCell, rc::Rc, time::Instant};

use crate::{
    bus::{Bus, VRAM},
    util::tiles_util::ScreenVector,
    windows::game_window::{GameWindow, GAMEBOY_SCREEN_HEIGHT, GAMEBOY_SCREEN_WIDTH},
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
    pub vram: VRAM,
    pub screen_array: ScreenVector,
    pub current_mode: PPUModes,

    dots_counter: usize,
    ly: u8,
}
impl PPU {
    pub fn new(vram: VRAM) -> PPU {
        PPU {
            vram,
            screen_array: ScreenVector::new_with_screen_size(
                GAMEBOY_SCREEN_WIDTH,
                GAMEBOY_SCREEN_HEIGHT,
            ),
            current_mode: PPUModes::Mode1,
            dots_counter: 0,
            ly: 0,
        }
    }
    pub fn next_tick(&mut self) {
        // 4 dots per cpu cycle
        for _ in 0..=4 {
            self.dots_counter += 1;
            self.tick_mode();
        }
        if self.dots_counter >= 70224 {
            self.dots_counter = 0
        }
        self.update_ly();
    }

    fn update_ly(&mut self) {
        self.ly += 1;
        if self.ly >= 153 {
            self.ly = 0
        }
        self.vram.set_ly(self.ly);
    }

    fn tick_mode(&self) {
        match self.current_mode {
            PPUModes::Mode2 => self.mode2(),
            PPUModes::Mode3 => self.mode3(),
            PPUModes::Mode0 => self.mode0(),
            PPUModes::Mode1 => self.mode1(),
        }
    }
    fn mode_all_at_once(&self) {
        // Draw background

        // Draw objects
        let oam_vec = self.vram.get_oam_sprites_vec();
        for object in oam_vec {}
    }

    fn mode2(&self) {}
    fn mode3(&self) {}
    fn mode0(&self) {}
    fn mode1(&self) {}
    fn block_memory(&self) {}

    fn draw() {}
}
