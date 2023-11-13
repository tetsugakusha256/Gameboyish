use crate::{ppu::VideoMemBlock, cartridge::load, util::error_type::Errors};

pub struct Bus {
    pub data: [u8; 0x1_0000],
    pub video_mem_block: VideoMemBlock,
}
impl Bus {
    pub fn new() -> Bus {
        Bus {
            data: [0x00; 0x1_0000],
            video_mem_block: VideoMemBlock::None,
        }
    }
    pub fn init(&mut self){
        self.load_boot_rom().unwrap();
    }
    pub fn load_boot_rom(&mut self) -> Result<(),Errors>{
        self.load_file("boot_roms/dmg_boot.bin", 0x0000)
    }
    // TODO: make it work for more complexe cartridge
    pub fn load_cartridge(&mut self, path:&str) -> Result<(),Errors>{
        self.load_file(path, 0x0000)
    }
    fn load_file(&mut self, path:&str, address: u16)-> Result<(), Errors>{
        let data = load(path)?;
        self.write_slice(address, data.as_slice());
        Ok(())
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        let add = address as usize;
        return self.data[add];
    }

    /// Read the byte at address + 0x0001
    pub fn read_next_byte(&self, address:u16)->u8{
        if address == 0xFFFF {panic!()}
        self.read_byte(address + 0x0001)
    }
    /// TODO: check that I understand correctly the little endian here
    pub fn get_a16_address(&self, pc: u16) -> u16 {
        let next_byte = self.read_byte(pc + 0x0001);
        let second_byte = self.read_byte(pc + 0x0002);
        ((second_byte as u16) << 8) + (next_byte as u16)
    }
    /// TODO: check that I understand correctly the little endian here
    pub fn get_a16_value(&self, pc: u16) -> u8 {
        self.read_byte(self.get_a16_address(pc))
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
    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
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
}

#[cfg(test)]
mod tests {
    use crate::bus::Bus;
    #[test]
    fn test_read() {
        let mut bus = Bus::new();        bus.data[0x0003] = 0xFF;
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
        let mut bus = Bus::new();        assert_ne!(bus.read_byte(0x0010), 0x12);
        bus.write_byte(0x0010, 0x12);
        assert_eq!(bus.read_byte(0x0010), 0x12);
    }
    #[test]
    fn read_slice() {
        let mut bus = Bus::new();        bus.write_slice(0x00F0, &[1,2,3,4,5]);
        assert_eq!(bus.data[0x00F0],1);
        assert_eq!(bus.data[0x00F1],2);
        assert_eq!(bus.data[0x00F2],3);
    }
    #[test]
    fn write_slice() {
        let mut bus = Bus::new();        bus.write_slice(0x0000, &[1, 2, 3, 4]);
        assert_eq!(bus.data[0x0002], 3);
        bus.write_slice(0xFFFF, &[4]);
        assert_eq!(bus.data[0xFFFF], 4);
    }
    #[test]
    #[should_panic(expected = "Trying to read out of bus memory")]
    fn read_slice_panic() {
        let bus = Bus::new();        bus.read_bytes_range(0xFFFF, 2);
    }
    #[test]
    #[should_panic(expected = "Trying to write out of bus memory")]
    fn write_slice_panic() {
        let mut bus = Bus::new();
        bus.write_slice(0xFFFF, &[1, 2]);
    }
    #[test]
    fn load_boot_loader(){
        let mut bus = Bus::new();
        let _ = bus.load_boot_rom();
        assert_eq!(bus.read_bytes_range(0x0000, 5),&[0x31,0xfe,0xff,0xaf,0x21]);
        assert_eq!(bus.read_byte(0x00FF),0x50);
    }
    #[test]
    fn get_a16_address_test() {
        let mut bus = Bus::new();
        bus.write_slice(0x0010, &[0x00, 0x10, 0x01]);
        assert_eq!(bus.get_a16_address(0x0010),0x0110);
    }
}
