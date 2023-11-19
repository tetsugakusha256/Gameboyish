use std::{cell::RefCell, rc::Rc, time::Instant};

use crate::{
    bus::{Bus, LCDControlReg, LCDStatusReg, WinBackPosReg, VRAM},
    util::tiles_util::{tile_fuse_byte_u8, ScreenVector},
    windows::game_window::{GameWindow, GAMEBOY_SCREEN_HEIGHT, GAMEBOY_SCREEN_WIDTH},
};

#[derive(Debug)]
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
    pub lcd_status: LCDStatusReg,
    pub lcd_control: LCDControlReg,
    pub win_back_mem: WinBackPosReg,

    pub screen_array: ScreenVector,
    pub current_mode: PPUModes,

    // total dots_counter
    dots_counter_frame: usize,
    dots_counter_line: usize,
    // current mode dots_counter
    dots_counter_mode: usize,

    line_pixels: [u8; 160],
    mode_3_last_dots_counter: usize,
    mode_3_pixel_counter: usize,
    ly: u8,
}
impl PPU {
    pub fn new(bus: Rc<RefCell<Bus>>) -> PPU {
        PPU {
            vram: VRAM::new(Rc::clone(&bus)),
            lcd_status: LCDStatusReg::new(Rc::clone(&bus)),
            lcd_control: LCDControlReg::new(Rc::clone(&bus)),
            win_back_mem: WinBackPosReg::new(Rc::clone(&bus)),
            screen_array: ScreenVector::new_with_screen_size(
                GAMEBOY_SCREEN_WIDTH,
                GAMEBOY_SCREEN_HEIGHT,
            ),
            current_mode: PPUModes::Mode2,
            dots_counter_frame: 0,
            dots_counter_line: 0,
            dots_counter_mode: 0,
            mode_3_last_dots_counter: 0,
            line_pixels: [0u8; 160],
            ly: 0,
            mode_3_pixel_counter: 0,
        }
    }
    pub fn next_tick(&mut self) {
        // 4 dots per cpu cycle
        for _ in 0..4 {
            self.dots_counter_frame += 1;
            self.dots_counter_line += 1;
            self.dots_counter_mode += 1;
            self.tick_mode();
        }
        if self.dots_counter_frame >= 70224 {
            self.dots_counter_frame = 0
        }
        // println!(
        //     "Current ppu mode: {:?}, dots frame: {}, dots mods: {}, dots line: {}, last mode 3 dots: {}",
        //     self.current_mode,
        //     self.dots_counter_frame,
        //     self.dots_counter_mode,
        //     self.dots_counter_line,
        //     self.mode_3_last_dots_counter
        // );
        // println!("LINE ly : {}", self.ly);
    }

    fn update_ly(&mut self) {
        self.ly += 1;
        if self.ly >= 154 {
            self.ly = 0
        }
        self.vram.set_ly(self.ly);
        // Check for stat interrupt
        self.lcd_status
            .set_lyc_ly(self.ly == self.lcd_status.get_lyc());
        self.lcd_status.set_ppu_mode(&self.current_mode);
        // TODO: stat interupt
    }

    fn tick_mode(&mut self) {
        match self.current_mode {
            PPUModes::Mode2 => self.mode2(),
            PPUModes::Mode3 => self.mode3(),
            PPUModes::Mode0 => self.mode0(),
            PPUModes::Mode1 => self.mode1(),
        }
    }

    fn insert_obj_vec_into_line_pixels(&mut self, vec: &Vec<u8>, pos_x: usize) {
        for i in 0..=7 {
            if pos_x + i < 160 {
                self.line_pixels[pos_x + i] = vec[i]
            };
        }
    }

    fn mode2(&mut self) {
        if self.dots_counter_mode == 1 {
            self.vram.lock_oam();
            self.vram.unlock_vram();
            let obj_tile_map = self.vram.get_oam_sprites_vec();
            let obj_height = if self.lcd_control.obj_size() { 8 } else { 16 };
            let ly_screen = self.ly + 16;
            let line_object = obj_tile_map.iter().filter(|obj| {
                obj.y < (144 + 16)
                    && obj.y + obj_height > 16
                    && (ly_screen >= obj.y && ly_screen < obj.y + obj_height)
            });
            for obj in line_object {
                println!(
                    "obj.y: {}, ly_screen: {}, height: {}",
                    obj.y, ly_screen, obj_height
                );
                let y_obj_offset = ly_screen - obj.y;
                println!(
                    "offset: {}, obj.y: {}, ly_screen: {}, height: {}",
                    y_obj_offset, obj.y, ly_screen, obj_height
                );
                let (l, h) = self.vram.get_tile_line(y_obj_offset, obj.tile_number, obj_height == 16);
                let obj_line_vec = tile_fuse_byte_u8(l, h);
                self.insert_obj_vec_into_line_pixels(&obj_line_vec, obj.x as usize);
            }
        }
        if self.dots_counter_line == 80 {
            self.current_mode = PPUModes::Mode3;
            self.dots_counter_mode = 0;
        }
    }
    fn mode3(&mut self) {
        if self.dots_counter_mode == 1 {
            self.vram.lock_vram();
            self.vram.lock_oam();
            println!("line_pixels {:?}", self.line_pixels);
        }
        // is window on this line?
        // get object on this line
        // draw background
        // draw object
        // draw window
        if self.dots_counter_mode < 160 {
            self.screen_array.set_x_y_gray(
                self.dots_counter_mode,
                self.ly as usize,
                self.line_pixels[self.dots_counter_mode],
            );
        }
        // let bg_tile_map = self.vram.bg_tile_map_vec();
        // let obj_tile_map = self.vram.get_oam_sprites_vec();
        // // How to know when line is done?
        // let screen_y = self.ly;

        // TODO: check the variable condition
        if self.mode_3_pixel_counter == 160 || self.dots_counter_mode >= MODE_3_DOTS_MAX {
            self.current_mode = PPUModes::Mode0;
            // println!("switch mode 3 to mode 0 : {:?}", self.current_mode);
            self.mode_3_last_dots_counter = self.dots_counter_mode;
            self.mode_3_pixel_counter = 0;
            self.dots_counter_mode = 0;
        }
    }
    fn mode0(&mut self) {
        if self.dots_counter_mode == 1 {
            self.vram.unlock_oam();
            self.vram.unlock_vram();
        }

        // TODO: check the variable condition
        if self.dots_counter_line == 456 {
            // println!(
            //     "line dots: {}, ly before: {}",
            //     self.dots_counter_line, self.ly
            // );
            // println!("SWITCH ly after: {}", self.ly);
            if self.ly == 143 {
                self.current_mode = PPUModes::Mode1;
            } else {
                self.current_mode = PPUModes::Mode2;
            }
            self.update_ly();
            self.dots_counter_mode = 0;
            self.dots_counter_line = 0;
        }
    }
    fn mode1(&mut self) {
        if self.dots_counter_mode % 456 == 1 {
            self.update_ly();
            self.dots_counter_line = 0;
        }
        if self.dots_counter_mode == MODE_1_DOTS {
            self.current_mode = PPUModes::Mode2;
            self.dots_counter_mode = 0;
            self.dots_counter_line = 0;
            // println!("New ppu frame");
        }
    }

    fn draw() {}
}
#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::bus::Bus;

    use super::PPU;

    #[test]
    fn insert_obj_vec_into_line_pixels_test() {
        let bus = Rc::new(RefCell::new(Bus::new()));
        let mut ppu = PPU::new(Rc::clone(&bus));
        let vec = vec![1u8, 3, 2, 3, 1, 1, 0, 0];
        ppu.insert_obj_vec_into_line_pixels(&vec, 0);
        assert_eq!(ppu.line_pixels[0], 1);
        assert_eq!(ppu.line_pixels[1], 3);
        assert_eq!(ppu.line_pixels[2], 2);
        assert_eq!(ppu.line_pixels[3], 3);
        ppu.insert_obj_vec_into_line_pixels(&vec, 158);
        assert_eq!(ppu.line_pixels[158], 1);
        assert_eq!(ppu.line_pixels[159], 3);
    }
}
