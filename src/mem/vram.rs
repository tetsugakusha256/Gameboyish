use std::{rc::Rc, cell::RefCell};

use crate::bus::{Bus, LCDControlReg, OAMSprite};

pub struct VRAM {
    bus: Rc<RefCell<Bus>>,
}
impl VRAM {
    pub fn new(bus: Rc<RefCell<Bus>>) -> VRAM {
        VRAM { bus }
    }
    // Return (scx, scy)
    pub fn get_background(&self) -> (u8, u8) {
        let bus = self.bus.borrow();
        (bus.read_byte(0xFF43), bus.read_byte(0xFF42))
    }
    // tile_number = the tile_number in the bg tile map(the map that stores id)
    // So it goes above 256 because the screen can show 20*18=360 tile on the screen
    pub fn get_background_tile_line(&self, y_offset: u8, tile_number: u16) -> (u8, u8) {
        // println!(
        //     "tile line : y_offset {}, tile_numbex: {}",
        //     y_offset, tile_number
        // );
        let address = match self.get_lcd_control().bg_tile_map() {
            true => 0x9C00,
            false => 0x9800,
        };
        self.get_tile_x_line_2bytes(address + tile_number as u16, y_offset)
    }
    // get the two bytes responsible for the x line tile color
    fn get_tile_x_line_2bytes(&self, tile_id_address: u16, line: u8) -> (u8, u8) {
        if line > 7 {
            panic!("Error");
        }
        let bus = self.bus.borrow();
        let tile_id = bus.read_byte_as_cpu(tile_id_address) as u16;
        // println!("tile id: {}, line: {}", tile_id, line);
        // Convert tile number to tile address
        let tile_address = match self.get_lcd_control().bg_win_tiles() {
            true => 0x8000u16 + tile_id * 16,
            false => match tile_id {
                0..=127 => 0x9000u16 + tile_id * 16,
                128..=255 => 0x8800u16 + tile_id * 16 - 128,
                _ => panic!("Impossible"),
            },
        };
        // println!("tile address: {:#06x}", tile_address);
        (
            bus.read_byte_as_cpu(tile_address + 2 * line as u16),
            bus.read_byte_as_cpu(tile_address + 2 * line as u16 + 1),
        )
    }
    pub fn get_tile_line(&self, y_offset: u8, tile_number: u8, size_16bit: bool) -> (u8, u8) {
        let (mut y_offset, mut tile_number) = (y_offset, tile_number);
        if size_16bit && y_offset > 7 {
            y_offset = y_offset - 8;
            tile_number += 1;
        }
        self.get_tile_x_line_2bytes(0x8000 + tile_number as u16, y_offset)
    }
    pub fn lock_oam(&mut self) {
        self.bus.borrow_mut().lock_oam();
    }
    pub fn unlock_oam(&mut self) {
        self.bus.borrow_mut().unlock_oam();
    }
    pub fn lock_vram(&mut self) {
        self.bus.borrow_mut().lock_vram();
    }
    pub fn unlock_vram(&mut self) {
        self.bus.borrow_mut().unlock_vram();
    }
    pub fn get_lcd_control(&self) -> LCDControlReg {
        LCDControlReg {
            bus: Rc::clone(&self.bus),
        }
    }
    pub fn set_ly(&mut self, value: u8) {
        self.bus.borrow_mut().write_byte(0xFF44, value)
    }
    pub fn get_ly(&self) -> u8 {
        self.bus.borrow().read_byte(0xFF44)
    }
    // get all 40 objects OAMSprite
    pub fn get_oam_sprites_vec(&self) -> Vec<OAMSprite> {
        let mut oam_vec = vec![];
        let oam_mem_start = 0xFE00;
        let bus = self.bus.borrow();
        for i in 0x00..0x40 {
            oam_vec.push(OAMSprite {
                y: bus.read_byte(oam_mem_start + i * 4),
                x: bus.read_byte(oam_mem_start + i * 4 + 1),
                tile_number: bus.read_byte(oam_mem_start + i * 4 + 2),
                flags: bus.read_byte(oam_mem_start + i * 4 + 3),
            })
        }
        oam_vec
    }
    pub fn win_tile_map_vec(&self) -> Vec<u8> {
        let address;
        if self.get_lcd_control().win_tile_map() {
            address = 0x9C00u16;
        } else {
            address = 0x9800u16;
        }
        self.bus.borrow().read_bytes_range(address, 1024).to_vec()
    }
    pub fn bg_tile_map_vec(&self) -> Vec<u8> {
        let address;
        if self.get_lcd_control().bg_tile_map() {
            address = 0x9C00u16;
        } else {
            address = 0x9800u16;
        }
        self.bus.borrow().read_bytes_range(address, 1024).to_vec()
    }
    // Return active Background tile memory
    pub fn bg_win_tile_memory_vec(&self) -> Vec<u8> {
        let address;
        if self.get_lcd_control().bg_win_tiles() {
            address = 0x8000u16;
        } else {
            address = 0x8800u16;
        }
        self.bus
            .borrow()
            .read_bytes_range(address, 2 * 2048)
            .to_vec()
    }
}
