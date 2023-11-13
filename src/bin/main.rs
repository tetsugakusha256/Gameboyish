use std::{cell::RefCell, rc::Rc};
use game_boyish::{
    cartridge::{check_checksum, load, print_header},
    emulator::{Emulator, EmulatorState},
    util::extract_opcode::load_json, bus::Bus, cpu::CPU, io_handler::IOHandler, screen::Screen, ppu::PPU, timer::Timer,
};

fn main() {
    println!("Welcome to GameBoyish the wanna be gameboy emulator!");
    let tetris_game =
        load("roms/Tetris (JUE) (V1.1) [!].gb").unwrap();
    let mario_game =
        load("roms/Super Mario Land (JUE) (V1.1) [!].gb")
            .unwrap();
    let pokemon_game = load("roms/Pokemon Red.gb").unwrap();
    let link_game = load("roms/Legend of Zelda, The - Link's Awakening (U) (V1.2) [!].gb").unwrap();
    let check1 = check_checksum(&tetris_game);
    let check2 = check_checksum(&mario_game);
    let check3 = check_checksum(&pokemon_game);
    print_header(&tetris_game);
    print_header(&mario_game);
    print_header(&pokemon_game);
    print_header(&link_game);
    println!("check: {}", check1);
    println!("check: {}", check2);
    println!("check: {}", check3);
    let opcodes_nopre = load_json("opcodes_nopre.json");
    let opcodes_pre = load_json("opcodes_pre.json");
    // let cpu:CPU = CPU::new();
    let bus = Rc::new(RefCell::new(Bus::new()));
    let mut emu = Emulator {
        cpu: CPU::new(Rc::clone(&bus)),
        ppu: PPU::new(Rc::clone(&bus)),        
        io_handler: IOHandler {
            bus: Rc::clone(&bus),
        },
        bus,
        timer: Timer::new(),
        state: EmulatorState::Running,
        cycles: 0,
        screen: Screen::new(144*6,160*6),
    };
    // emu.init();
}
