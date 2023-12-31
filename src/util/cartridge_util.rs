use crate::util::{
    display_impl::{u8_array_to_individual_char, u8_array_to_string},
    error_type::Errors,
};
use std::{fs::File, io::Read};

// start, offset

// think of data type
/// Type to specify a range in memory
pub struct MemRange(pub usize, pub usize);
pub struct CartridgeData(pub Vec<u8>);
impl CartridgeData {
    pub fn get_mbc_type(&self) -> MBCType {
        self.0[CARTRIDGE_TYPE.0].into()
    }
}
pub const NINTENDO_LOGO: MemRange = MemRange(0x0104, 0x0133);
pub const TITLE: MemRange = MemRange(0x0134, 0x0143);
pub const MANUFACTURER_CODE_OLD: MemRange = MemRange(0x013F, 0x0143);
pub const CGB_FLAG: MemRange = MemRange(0x0143, 0x0143);
pub const NEW_LICENSEE_CODE: MemRange = MemRange(0x0144, 0x0145);
pub const SGB_FLAG: MemRange = MemRange(0x0146, 0x0146);
pub const CARTRIDGE_TYPE: MemRange = MemRange(0x0147, 0x0147);
pub const ROM_SIZE: MemRange = MemRange(0x0148, 0x0148);
pub const RAM_SIZE: MemRange = MemRange(0x0149, 0x0149);
pub const DESTINATION_CODE: MemRange = MemRange(0x014a, 0x014a);
pub const OLD_LICENSEE_CODE: MemRange = MemRange(0x014b, 0x014b);
pub const ROM_VERSION_NUMBER: MemRange = MemRange(0x014c, 0x014c);
pub const HEADER_CHECKSUM: MemRange = MemRange(0x014d, 0x014d);
pub const GLOBAL_CHECKSUM: MemRange = MemRange(0x014e, 0x014f);

#[allow(non_camel_case_types)]
pub enum MBCType {
    ROM_ONLY,
    MBC1,
    MBC1_RAM,
    MBC1_RAM_BATTERY,
    MBC2,
    MBC2_BATTERY,
    ROM_RAM_1,
    ROM_RAM_BATTERY_1,
    MMM01,
    MMM01_RAM,
    MMM01_RAM_BATTERY,
    MBC3_TIMER_BATTERY,
    MBC3_TIMER_RAM_BATTERY_2,
    MBC3,
    MBC3_RAM_2,
    MBC3_RAM_BATTERY_2,
    MBC5,
    MBC5_RAM,
    MBC5_RAM_BATTERY,
    MBC5_RUMBLE,
    MBC5_RUMBLE_RAM,
    MBC5_RUMBLE_RAM_BATTERY,
    MBC6,
    MBC7_SENSOR_RUMBLE_RAM_BATTERY,
    POCKET_CAMERA,
    BANDAI_TAMA5,
    HuC3,
    HuC1_RAM_BATTERY,
}
impl From<u8> for MBCType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => MBCType::ROM_ONLY,
            0x01 => MBCType::MBC1,
            0x02 => MBCType::MBC1_RAM,
            0x03 => MBCType::MBC1_RAM_BATTERY,
            0x05 => MBCType::MBC2,
            0x06 => MBCType::MBC2_BATTERY,
            0x08 => MBCType::ROM_RAM_1,
            0x09 => MBCType::ROM_RAM_BATTERY_1,
            0x0B => MBCType::MMM01,
            0x0C => MBCType::MMM01_RAM,
            0x0D => MBCType::MMM01_RAM_BATTERY,
            0x0F => MBCType::MBC3_TIMER_BATTERY,
            0x10 => MBCType::MBC3_TIMER_RAM_BATTERY_2,
            0x11 => MBCType::MBC3,
            0x12 => MBCType::MBC3_RAM_2,
            0x13 => MBCType::MBC3_RAM_BATTERY_2,
            0x19 => MBCType::MBC5,
            0x1A => MBCType::MBC5_RAM,
            0x1B => MBCType::MBC5_RAM_BATTERY,
            0x1C => MBCType::MBC5_RUMBLE,
            0x1D => MBCType::MBC5_RUMBLE_RAM,
            0x1E => MBCType::MBC5_RUMBLE_RAM_BATTERY,
            0x20 => MBCType::MBC6,
            0x22 => MBCType::MBC7_SENSOR_RUMBLE_RAM_BATTERY,
            0xFC => MBCType::POCKET_CAMERA,
            0xFD => MBCType::BANDAI_TAMA5,
            0xFE => MBCType::HuC3,
            0xFF => MBCType::HuC1_RAM_BATTERY,
            _ => panic!("Not a valid mbc type"),
        }
    }
}

/// Load the rom into a Vec<u8>
pub fn load(file_path: &str) -> Result<CartridgeData, Errors> {
    let mut file = File::open(file_path)?;
    let mut file_data = Vec::new();
    let byte_read = file.read_to_end(&mut file_data);
    match byte_read {
        Ok(x) => {
            println!("loading file: {}", file_path);
            eprintln!("Byte read: {}", x);
        }
        Err(e) => return Err(Errors::ErrorReadingFile(e)),
    }
    return Ok(CartridgeData(file_data));
}
/// Calculate the header checksum and return true if it matched the rom_data
pub fn check_checksum(rom_data: &Vec<u8>) -> bool {
    let mut checksum = 0u8;
    for address in 0x0134..=0x014C {
        checksum = checksum.wrapping_sub(rom_data.get(address).unwrap() + 1);
    }
    let checksum_cart = &rom_data[HEADER_CHECKSUM.0 as usize..=HEADER_CHECKSUM.1 as usize]
        .get(0)
        .unwrap();
    match checksum == **checksum_cart {
        true => {
            println!("Checksum header OK!");
            return true;
        }
        false => (),
    }
    false
}
pub fn print_header(rom_data: &Vec<u8>) -> () {
    let title = &rom_data[TITLE.0..=TITLE.1];
    println!("##################");
    println!("Title: {}", u8_array_to_individual_char(title));
    let man_code = &rom_data[MANUFACTURER_CODE_OLD.0..=MANUFACTURER_CODE_OLD.1];
    println!("Manufacturer code old: {}", u8_array_to_string(man_code));
    let nintendo_logo = &rom_data[NINTENDO_LOGO.0..=NINTENDO_LOGO.1];
    println!("Nintendo logo: {}", u8_array_to_string(nintendo_logo));
    let cgb_flag = &rom_data[CGB_FLAG.0..=CGB_FLAG.1];
    println!("CGB flag: {}", u8_array_to_string(cgb_flag));
    let sgb_flag = &rom_data[SGB_FLAG.0..=SGB_FLAG.1];
    println!("SGB flag: {}", u8_array_to_string(sgb_flag));
    let new_licensee_code = &rom_data[NEW_LICENSEE_CODE.0..=NEW_LICENSEE_CODE.1];
    println!(
        "New licensee code: {}",
        u8_array_to_string(new_licensee_code)
    );
    let old_licensee_code = &rom_data[OLD_LICENSEE_CODE.0..=OLD_LICENSEE_CODE.1];
    println!(
        "Old licensee code: {}",
        u8_array_to_string(old_licensee_code)
    );
    let cartridge_type = &rom_data[CARTRIDGE_TYPE.0..=CARTRIDGE_TYPE.1];
    println!("Cartridge type: {}", u8_array_to_string(cartridge_type));
    let rom_size = &rom_data[ROM_SIZE.0..=ROM_SIZE.1];
    println!("Rom size: {}", u8_array_to_string(rom_size));
    let ram_size = &rom_data[RAM_SIZE.0..=RAM_SIZE.1];
    println!("Ram size: {}", u8_array_to_string(ram_size));
    let dest_code = &rom_data[DESTINATION_CODE.0 as usize..=DESTINATION_CODE.1 as usize];
    println!("Destination code: {}", u8_array_to_string(dest_code));
    let rom_version = &rom_data[ROM_VERSION_NUMBER.0 as usize..=ROM_VERSION_NUMBER.1 as usize];
    println!("Rom version number: {}", u8_array_to_string(rom_version));
    let header_checksum = &rom_data[HEADER_CHECKSUM.0 as usize..=HEADER_CHECKSUM.1 as usize];
    println!("Header checksum: {}", u8_array_to_string(header_checksum));
    let global_checksum = &rom_data[GLOBAL_CHECKSUM.0 as usize..=GLOBAL_CHECKSUM.1 as usize];
    println!("Global checksum: {}", u8_array_to_string(global_checksum));
}

#[cfg(test)]
mod tests {
    use crate::util::cartridge_util::load;

    #[test]
    fn load_tetris() {
        assert_eq!(0x0100, 0x0100);
        let test = load("../roms/Tetris (JUE) (V1.1) [!].gb");
        let test = load("/home/anon/Documents/Code/GameBoyish/roms/cpu_instrs/06-ld r,r.gb");
    }
}
