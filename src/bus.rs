use crate::{
    ppu::VideoMemBlock,
    util::{
        cartridge_util::load,
        error_type::Errors,
        u8_traits::{Bit, NibblesU16},
    },
};
use std::{cell::RefCell, rc::Rc};

pub struct VRAM {
    bus: Rc<RefCell<Bus>>,
}
impl VRAM {
    pub fn new(bus: Rc<RefCell<Bus>>) -> VRAM {
        VRAM { bus }
    }
    pub fn get_background(&self) -> BackgroundReg {
        let bus = self.bus.borrow();
        BackgroundReg {
            scy: bus.read_byte(0xFF42),
            scx: bus.read_byte(0xFF43),
        }
    }
    pub fn get_window(&self) -> WindowReg {
        let bus = self.bus.borrow();
        WindowReg {
            wy: bus.read_byte(0xFF4A),
            wx: bus.read_byte(0xFF4B),
        }
    }
    pub fn get_lcd_control(&self) -> LCDControlReg {
        LCDControlReg {
            byte: self.bus.borrow().read_byte(0xFF40),
        }
    }
    pub fn set_ly(&mut self, value: u8) {
        self.bus.borrow_mut().write_byte(0xFF44, value)
    }
    pub fn get_ly(&self) -> u8 {
        self.bus.borrow().read_byte(0xFF44)
    }
    pub fn get_oam_sprites_vec(&self) -> Vec<OAMSprite> {
        let mut oam_vec = vec![];
        let oam_mem_start = 0xFE00;
        let bus = self.bus.borrow();
        for i in 0x00..=0x40 {
            oam_vec.push(OAMSprite {
                y: bus.read_byte(oam_mem_start + i * 4),
                x: bus.read_byte(oam_mem_start + i * 4 + 1),
                tile_number: bus.read_byte(oam_mem_start + i * 4 + 2),
                flags: bus.read_byte(oam_mem_start + i * 4 + 3),
            })
        }
        oam_vec
    }
    pub fn win_tile_map_vec(&self, bus: &Bus) -> Vec<u8> {
        let address;
        if self.get_lcd_control().win_tile_map() {
            address = 0x9C00u16;
        } else {
            address = 0x9800u16;
        }
        bus.read_bytes_range(address, 1024).to_vec()
    }
    pub fn bg_tile_map_vec(&self, bus: &Bus) -> Vec<u8> {
        let address;
        if self.get_lcd_control().bg_tile_map() {
            address = 0x9C00u16;
        } else {
            address = 0x9800u16;
        }
        bus.read_bytes_range(address, 1024).to_vec()
    }
    // Return active Background tile memory
    pub fn bg_win_tile_memory_vec(&self, bus: &Bus) -> Vec<u8> {
        let address;
        if self.get_lcd_control().bg_win_tiles() {
            address = 0x8000u16;
        } else {
            address = 0x8800u16;
        }
        bus.read_bytes_range(address, 2 * 2048).to_vec()
    }
}
pub struct BackgroundReg {
    scy: u8,
    scx: u8,
}
pub struct WindowReg {
    wy: u8,
    wx: u8,
}
#[derive(Debug)]
pub enum InteruptType {
    VBlank,
    LCD,
    Timer,
    Serial,
    Joypad,
}
impl From<u8> for InteruptType {
    fn from(value: u8) -> Self {
        match value {
            0 => InteruptType::VBlank,
            1 => InteruptType::LCD,
            2 => InteruptType::Timer,
            3 => InteruptType::Serial,
            4 => InteruptType::Joypad,
            _ => panic!("Invalid Interupt Type"),
        }
    }
}
// Interupt flags and enabled status
pub struct InteruptReg {
    bus: Rc<RefCell<Bus>>,
}
impl InteruptReg {
    pub fn new(bus: Rc<RefCell<Bus>>) -> InteruptReg {
        InteruptReg { bus }
    }
    // Return the hightest priority interupt type that has it's flag set
    pub fn query_interupts_flag(&self) -> Option<InteruptType> {
        let is_interupts = self.get_interupt_flag();
        for i in 0..=4 {
            if is_interupts.get_bit(i) {
                return Some(i.into());
            }
        }
        None
    }
    // Return the hightest priority interupts that has it's flag and is enable
    pub fn query_interupts_flag_enable(&self) -> Option<InteruptType> {
        let is_interupts = self.get_interupt_flag() & self.get_interupt_enable();
        for i in 0..=4 {
            if is_interupts.get_bit(i) {
                return Some(i.into());
            }
        }
        None
    }
    // Reset flag of given InteruptType
    pub fn reset_flag(&mut self, interupt_type: &InteruptType) {
        match interupt_type {
            InteruptType::VBlank => self.set_vblank_flag(false),
            InteruptType::LCD => self.set_lcd_flag(false),
            InteruptType::Timer => self.set_timer_flag(false),
            InteruptType::Serial => self.set_serial_flag(false),
            InteruptType::Joypad => self.set_joypad_flag(false),
        }
    }

    pub fn set_joypad_flag(&mut self, value: bool) {
        let mut bus = self.bus.borrow_mut();
        let mut mem = bus.read_byte(0xFFFF);
        mem.set_bit(4, value);
        bus.write_byte(0xFFFF, mem);
    }
    pub fn set_serial_flag(&mut self, value: bool) {
        let mut bus = self.bus.borrow_mut();
        let mut mem = bus.read_byte(0xFFFF);
        mem.set_bit(3, value);
        bus.write_byte(0xFFFF, mem);
    }
    pub fn set_timer_flag(&mut self, value: bool) {
        let mut bus = self.bus.borrow_mut();
        let mut mem = bus.read_byte(0xFFFF);
        mem.set_bit(2, value);
        bus.write_byte(0xFFFF, mem);
    }
    pub fn set_lcd_flag(&mut self, value: bool) {
        let mut bus = self.bus.borrow_mut();
        let mut mem = bus.read_byte(0xFFFF);
        mem.set_bit(1, value);
        bus.write_byte(0xFFFF, mem);
    }
    pub fn set_vblank_flag(&mut self, value: bool) {
        let mut bus = self.bus.borrow_mut();
        let mut mem = bus.read_byte(0xFFFF);
        mem.set_bit(0, value);
        println!("mem: {}", mem);
        bus.write_byte(0xFFFF, mem);
    }
    pub fn get_joypad_flag(&self) -> bool {
        self.get_interupt_flag().get_bit(4)
    }
    pub fn get_serial_flag(&self) -> bool {
        self.get_interupt_flag().get_bit(3)
    }
    pub fn get_timer_flag(&self) -> bool {
        self.get_interupt_flag().get_bit(2)
    }
    pub fn get_lcd_flag(&self) -> bool {
        self.get_interupt_flag().get_bit(1)
    }
    pub fn get_vblank_flag(&self) -> bool {
        self.get_interupt_flag().get_bit(0)
    }
    pub fn get_interupt_enable(&self) -> u8 {
        self.bus.borrow().read_byte(0xFF0F)
    }
    pub fn get_interupt_flag(&self) -> u8 {
        self.bus.borrow().read_byte(0xFFFF)
    }
    pub fn is_joypad_enable(&self) -> bool {
        self.get_interupt_enable().get_bit(4)
    }
    pub fn is_serial_enable(&self) -> bool {
        self.get_interupt_enable().get_bit(3)
    }
    pub fn is_timer_enable(&self) -> bool {
        self.get_interupt_enable().get_bit(2)
    }
    pub fn is_lcd_enable(&self) -> bool {
        self.get_interupt_enable().get_bit(1)
    }
    pub fn is_vblank_enable(&self) -> bool {
        self.get_interupt_enable().get_bit(0)
    }
}
pub struct LCDControlReg {
    byte: u8,
}
impl LCDControlReg {
    pub fn lcd_ppu_enable(&self) -> bool {
        self.byte.get_bit(7)
    }
    pub fn win_tile_map(&self) -> bool {
        self.byte.get_bit(6)
    }
    pub fn win_enable(&self) -> bool {
        self.byte.get_bit(5)
    }
    pub fn bg_win_tiles(&self) -> bool {
        self.byte.get_bit(4)
    }
    pub fn bg_tile_map(&self) -> bool {
        self.byte.get_bit(3)
    }
    pub fn obj_size(&self) -> bool {
        self.byte.get_bit(2)
    }
    pub fn obj_enable(&self) -> bool {
        self.byte.get_bit(1)
    }
    pub fn bg_win_enable_priority(&self) -> bool {
        self.byte.get_bit(0)
    }
}
pub struct OAMSprite {
    pub y: u8,
    pub x: u8,
    pub tile_number: u8,
    pub flags: u8,
}
impl OAMSprite {
    pub fn render_priority(&self) -> bool {
        self.flags.get_bit(7)
    }
    pub fn y_flip(&self) -> bool {
        self.flags.get_bit(6)
    }
    pub fn x_flip(&self) -> bool {
        self.flags.get_bit(5)
    }
    pub fn palette_number(&self) -> bool {
        self.flags.get_bit(4)
    }
}

pub struct Bus {
    pub data: [u8; 0x1_0000],
    pub video_mem_block: VideoMemBlock,
}
impl std::fmt::Display for Bus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = "".to_string();
        for (i, num) in self.data.iter().enumerate() {
            let text_num = format!("{:#04x} ", num).replace("0x", "").to_uppercase();
            text += format!("{} ", text_num).as_str();
            if i % 16 == 0 {
                text += "\n";
            }
        }
        write!(f, "Bus: \n Data: {}", text)
    }
}
impl Bus {
    pub fn print_slice(&self, a: u16, b: u16) {
        let mut text = "".to_string();
        for (i, num) in self.data.iter().enumerate() {
            if i >= a as usize {
                if i % 16 == 0 {
                    text += format!("\n {:#06x} : ", i).as_str();
                }
                let text_num = format!("{:#04x} ", num).replace("0x", "").to_uppercase();
                text += format!("{} ", text_num).as_str();
                if i > b as usize {
                    break;
                }
            }
        }
        println!("Bus \n {}", text);
    }
    pub fn new() -> Bus {
        Bus {
            data: [0x00; 0x1_0000],
            video_mem_block: VideoMemBlock::None,
        }
    }
    pub fn init(&mut self) {
        self.load_boot_rom().unwrap();
    }
    pub fn load_boot_rom(&mut self) -> Result<(), Errors> {
        self.load_file("boot_roms/dmg_boot.bin", 0x0000)
    }
    // TODO: make it work for more complexe cartridge
    pub fn load_cartridge(&mut self, path: &str) -> Result<(), Errors> {
        self.load_file(path, 0x0000)
    }
    fn load_file(&mut self, path: &str, address: u16) -> Result<(), Errors> {
        let data = load(path)?;
        self.write_slice(address, data.0.as_slice());
        Ok(())
    }
    pub fn read_a8(&self, offset: u8) -> u8 {
        return self.read_byte(0xFF00 + offset as u16);
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        // temporary gamedoctor thing
        if address == 0xFF44 {
            return 0x90;
        }
        let add = address as usize;
        return self.data[add];
    }
    pub fn read_2_bytes_from_little_endian_address(&self, address: u16) -> u8 {
        let high = address.high_8nibble();
        let low = address.high_8nibble();
        let small_endian_address = ((low as u16) << 8) + (high as u16);
        // Doctor
        if small_endian_address == 0xFF44 {
            return 0x90;
        }
        self.read_byte(small_endian_address)
    }
    pub fn read_2_bytes_little_endian(&self, address: u16) -> u16 {
        let low = self.read_byte(address);
        let high = self.read_byte(address + 1);
        ((high as u16) << 8) + (low as u16)
    }

    pub fn read_bytes_range(&self, address: u16, length: u16) -> &[u8] {
        let add = address as usize;
        let len = length as usize;
        let add_end = add + len;
        if add_end > self.data.len() {
            panic!("Trying to read out of bus memory");
        }
        return &self.data[add..add_end];
    }
    /// Read the byte at address + 0x0001
    pub fn read_next_byte(&self, address: u16) -> u8 {
        if address == 0xFFFF {
            panic!()
        }
        self.read_byte(address + 0x0001)
    }
    /// TODO: check that I understand correctly the little endian here
    pub fn get_a16_address(&self, pc: u16) -> u16 {
        self.read_2_bytes_little_endian(pc + 1)
        // let next_byte = self.read_byte(pc + 0x0001);
        // let second_byte = self.read_byte(pc + 0x0002);
        // ((second_byte as u16) << 8) + (next_byte as u16)
    }
    /// TODO: check that I understand correctly the little endian here
    pub fn get_a16_value(&self, pc: u16) -> u8 {
        self.read_byte(self.get_a16_address(pc))
    }
    pub fn write_a16(&mut self, pc: u16, value: u16) {
        self.write_2_bytes_little_endian(self.get_a16_address(pc), value)
    }
    /// write the byte at address + 0x0001
    pub fn write_next_byte(&mut self, address: u16, value: u8) {
        if address == 0xFFFF {
            panic!("trying to read out of bus");
        }
        self.write_byte(address + 0x0001, value);
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
    //TODO: should the write happen in little endian?
    //I feel like it should be the same as the load method
    //used to retrieve them
    pub fn write_2_bytes_little_endian(&mut self, address: u16, value: u16) {
        let high = value.high_8nibble();
        let low = value.low_8nibble();
        self.write_slice(address, &[low, high]);
    }
    pub fn write_2_bytes_big_endian(&mut self, address: u16, value: u16) {
        let high = value.high_8nibble();
        let low = value.low_8nibble();
        self.write_slice(address, &[high, low]);
    }
    pub fn write_slice(&mut self, address: u16, slice: &[u8]) {
        let add = address as usize;
        let add_end = add + slice.len();
        if add_end > self.data.len() {
            panic!("Trying to write out of bus memory");
        }
        let data_slice = &mut self.data[add..add_end];
        data_slice.copy_from_slice(slice);
    }

    // 0-7 line
    fn get_tile_x_line_2bytes(&self, address: u16, line: u8) -> (u8, u8) {
        if line > 7 {
            panic!("Error");
        }
        let i = (line * 2) as u16;
        (self.read_byte(address + i), self.read_byte(address + i + 1))
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::bus::{Bus, InteruptReg, InteruptType};
    #[test]
    fn test_read() {
        let mut bus = Bus::new();
        bus.data[0x0003] = 0xFF;
        bus.data[0xF003] = 0xFC;
        bus.data[0xFFFF] = 0xFC;
        assert_eq!(bus.read_byte(0x0003), 0xFF);
        assert_eq!(bus.read_byte(0x0001), 0x00);
        assert_eq!(bus.read_byte(0xF003), 0xFC);
        assert_eq!(bus.read_byte(0xFFFF), 0xFC);

        assert_ne!(bus.read_byte(0x0000), 0xA0);
    }
    #[test]
    fn test_write() {
        let mut bus = Bus::new();
        assert_ne!(bus.read_byte(0x0010), 0x12);
        bus.write_byte(0x0010, 0x12);
        assert_eq!(bus.read_byte(0x0010), 0x12);
    }
    #[test]
    fn read_slice() {
        let mut bus = Bus::new();
        bus.write_slice(0x00F0, &[1, 2, 3, 4, 5]);
        assert_eq!(bus.data[0x00F0], 1);
        assert_eq!(bus.data[0x00F1], 2);
        assert_eq!(bus.data[0x00F2], 3);
    }
    #[test]
    fn write_slice() {
        let mut bus = Bus::new();
        bus.write_slice(0x0000, &[1, 2, 3, 4]);
        assert_eq!(bus.data[0x0002], 3);
        bus.write_slice(0xFFFF, &[4]);
        assert_eq!(bus.data[0xFFFF], 4);
    }
    #[test]
    #[should_panic(expected = "Trying to read out of bus memory")]
    fn read_slice_panic() {
        let bus = Bus::new();
        bus.read_bytes_range(0xFFFF, 2);
    }
    #[test]
    #[should_panic(expected = "Trying to write out of bus memory")]
    fn write_slice_panic() {
        let mut bus = Bus::new();
        bus.write_slice(0xFFFF, &[1, 2]);
    }
    #[test]
    fn load_boot_loader() {
        let mut bus = Bus::new();
        let _ = bus.load_boot_rom();
        assert_eq!(
            bus.read_bytes_range(0x0000, 5),
            &[0x31, 0xfe, 0xff, 0xaf, 0x21]
        );
        assert_eq!(bus.read_byte(0x00FF), 0x50);
    }
    // #[test]
    // fn get_tile_x_line_2bytes_test() {
    //     let mut bus = Bus::new();
    //     bus.write_slice(0x1111, &[0x00, 0x10, 0x01, 0x00, 0x00, 0x33, 0x44]);
    //     assert_eq!(bus.get_tile_x_line_2bytes(0x1111, 2), (0x00, 0x33));
    // }
    #[test]
    fn get_a16_address_test() {
        let mut bus = Bus::new();
        bus.write_slice(0x0010, &[0x00, 0x10, 0x01]);
        assert_eq!(bus.get_a16_address(0x0010), 0x0110);
    }
    #[test]
    fn reset_flag_test() {
        let bus_rc = Rc::new(RefCell::new(Bus::new()));
        let mut interupt_reg = InteruptReg::new(Rc::clone(&bus_rc));
        {
            let mut bus = bus_rc.borrow_mut();
            println!("0xFFFF: {}", bus.read_byte(0xFFFF));
            bus.write_byte(0xFFFF, 0b0000_0001);
            println!("0xFFFF: {}", bus.read_byte(0xFFFF));
        }
        interupt_reg.reset_flag(&InteruptType::VBlank);
        {
            let mut bus = bus_rc.borrow_mut();
            println!("0xFFFF: {}", bus.read_byte(0xFFFF));
            assert_eq!(bus.read_byte(0xFFFF), 0b0000_0000);
        }
    }
}
