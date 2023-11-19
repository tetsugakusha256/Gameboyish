use game_boyish::{
    bus::{Bus, VRAM},
    cpu::CPU,
    emulator::{Emulator, EmulatorState},
    io_handler::IOHandler,
    ppu::PPU,
    timer::Timer,
    util::{
        cartridge_util::{check_checksum, load, print_header},
        extract_opcode::load_json,
    },
    windows::game_window::GameWindow,
};
use std::{cell::RefCell, rc::Rc};

fn main() {
    println!("Welcome to GameBoyish the wanna be gameboy emulator!");
    let tetris_game = load("roms/Tetris (JUE) (V1.1) [!].gb").unwrap().0;
    let mario_game = load("roms/Super Mario Land (JUE) (V1.1) [!].gb").unwrap().0;
    let pokemon_game = load("roms/Pokemon Red.gb").unwrap().0;

    let test_rom = load("/home/anon/Documents/Code/GameBoyish/roms/cpu_instrs/06-ld r,r.gb")
        .unwrap()
        .0;
    let link_game = load("roms/Legend of Zelda, The - Link's Awakening (U) (V1.2) [!].gb")
        .unwrap()
        .0;
    let check1 = check_checksum(&tetris_game);
    let check2 = check_checksum(&mario_game);
    let check3 = check_checksum(&pokemon_game);
    print_header(&test_rom);
    print_header(&tetris_game);
    // print_header(&mario_game);
    // print_header(&pokemon_game);
    // print_header(&link_game);
    println!("check: {}", check1);
    println!("check: {}", check2);
    println!("check: {}", check3);
    // let cpu:CPU = CPU::new();
    let bus = Rc::new(RefCell::new(Bus::new()));
    let mut emu = Emulator {
        cpu: CPU::new_doctor(Rc::clone(&bus)),
        ppu: PPU::new(VRAM::new(Rc::clone(&bus))),
        io_handler: IOHandler::new(Rc::clone(&bus)),
        bus,
        timer: Timer::new(),
        state: EmulatorState::Running,
        cycles: 0,
        screen: GameWindow::new(144 * 6, 160 * 6),
        debug_screen: GameWindow::new(144 * 6, 160 * 6),
    };
    emu.init();
    let opcodes_pre = load_json("opcodes_pre.json");
    let opcodes_nopre = load_json("opcodes_nopre.json");
}
