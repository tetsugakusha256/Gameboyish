use game_boyish::cartridge::{load, print_header, check_checksum};

fn main() {
    println!("Welcome to GameBoyish the wanna be gameboy emulator!");
    let tetris_game = load("/home/anon/Documents/Code/GameBoyish/roms/Tetris (JUE) (V1.1) [!].gb").unwrap();
    let mario_game = load("/home/anon/Documents/Code/GameBoyish/roms/Super Mario Land (JUE) (V1.1) [!].gb").unwrap();
    let pokemon_game = load("/home/anon/Documents/Code/GameBoyish/roms/Pokemon Red.gb").unwrap();
    let link_game = load("/home/anon/Documents/Code/GameBoyish/roms/Legend of Zelda, The - Link's Awakening (U) (V1.2) [!].gb").unwrap();
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
}
