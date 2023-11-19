use crate::{windows::game_window::from_u32_gray_to_rgb, bus::{OAMSprite, VRAM}};

use super::u8_traits::Bit;

const TILE_BYTE_SIZE: usize = 16usize;
const VRAM_BYTE_SIZE: usize = 6144usize;
// const VRAM_BYTE_SIZE: usize = 512usize;
// TODO: make a struct that has a Vec<u8> and the length of a line
#[derive(Debug)]
pub struct ScreenVector {
    pub pixelcolor_vec: Vec<u32>,
    pub width: usize,
}
impl ScreenVector {
    pub fn new_with_screen_size(width: usize, height: usize) -> ScreenVector {
        let length = width * height;
        ScreenVector {
            pixelcolor_vec: vec![0u32; length],
            width,
        }
    }
    pub fn new(vector: Vec<u32>, width: usize) -> ScreenVector {
        ScreenVector {
            pixelcolor_vec: vector,
            width,
        }
    }
    // TODO:
    pub fn insert_object_tile(&mut self, object_tile: &OAMSprite, vram: &VRAM){
           
    }
    pub fn set_x_y_gray(&mut self, x: usize, y: usize, gray_value: u8) {
        if x >= self.width || y >= self.height() {
            panic!("Out of bound screen read attempt");
        }
        self.pixelcolor_vec[x + y * self.width] = from_u8_gray_to_rgb(gray_value)
    }
    pub fn height(&self) -> usize {
        self.pixelcolor_vec.len() / self.width
    }
    pub fn from_gray_vec_to_screen_vec(&mut self) {
        self.pixelcolor_vec = self
            .pixelcolor_vec
            .iter()
            .map(|pixel| from_u32_gray_to_rgb(pixel.saturating_mul(85)))
            .collect();
    }
}

#[derive(Debug)]
pub struct TileVector {
    tile_vec: Vec<u8>,
    bytes_per_line: usize,
}
impl TileVector {
    pub fn new(vector: Vec<u8>, width: usize) -> TileVector {
        TileVector {
            tile_vec: vector,
            bytes_per_line: width,
        }
    }
    pub fn height(&self) -> usize {
        self.tile_vec.len() / self.bytes_per_line
    }
}

fn tile_fuse_byte(l: u8, h: u8) -> Vec<u32> {
    let h_bits = h.to_bits_array();
    let l_bits = l.to_bits_array();
    let iterator = h_bits.iter().zip(l_bits.iter());
    iterator
        .map(|(h_bit, l_bit)| *h_bit as u32 * 2 + *l_bit as u32)
        .collect()
}
pub fn vram_to_screen(vram: Vec<u8>, tile_per_line: u8) -> ScreenVector {
    let tile_vec = vram_to_tile_vec(vram, tile_per_line);
    let mut gray_vec = from_tile_vec_to_gray_vec(tile_vec);
    // print_screen_vec(&gray_vec);
    gray_vec.from_gray_vec_to_screen_vec();
    gray_vec
}
// Takes a vec of tile (usually the entire vram) and converts it for the screen to display
// with a given number of tile per line
// Effectively fusing the tiles to be displayed
fn vram_to_tile_vec(vram: Vec<u8>, tile_per_line: u8) -> TileVector {
    let tile_per_line = tile_per_line as usize;
    let byte_per_line = tile_per_line * 2;
    // buf of the vram size
    let mut buf: Vec<u8> = vec![0; VRAM_BYTE_SIZE];
    let mut test: Vec<u8> = vec![0; VRAM_BYTE_SIZE];

    for (i, byte) in vram.iter().enumerate() {
        // Variation within a tile
        let pair = byte_per_line * ((i / 2) % 8);
        // Variation within line
        let tile = (2 * (i / 16) + (i % 2)) % (tile_per_line * 2);
        // Variation within block
        let block = (8 * byte_per_line as usize) * (i / (8 * byte_per_line as usize));

        let new_i = pair + tile + block;

        if new_i < VRAM_BYTE_SIZE {
            test[new_i] = i as u8;
            buf[new_i] = *byte;
        }
    }
    // print_tile_vec(TileVector::new(buf.clone(), 2 * tile_per_line));
    // ScreenVector::new(buf, tile_per_line * 2)
    TileVector {
        tile_vec: buf,
        bytes_per_line: byte_per_line,
    }
}

fn from_tile_vec_to_gray_vec(byte_vec: TileVector) -> ScreenVector {
    let mut gray_vec: Vec<u32> = Vec::new();
    for byte_pair in byte_vec.tile_vec.chunks(2) {
        gray_vec.extend_from_slice(&tile_fuse_byte(byte_pair[0], byte_pair[1]));
    }
    ScreenVector {
        pixelcolor_vec: gray_vec,
        width: byte_vec.bytes_per_line * 4,
    }
}

fn print_screen_vec(tile_vec: &ScreenVector) {
    let width = tile_vec.width;
    println!("ScreenVector : {}", width);
    for (i, tile) in tile_vec.pixelcolor_vec.iter().enumerate() {
        if i % width == 0 {
            println!("");
        }
        print!("{} \t", tile);
    }
}
fn print_tile_vec(tile_vec: TileVector) {
    let width = tile_vec.bytes_per_line;
    println!("Tile Vec: {}", width);
    for (i, tile) in tile_vec.tile_vec.iter().enumerate() {
        if i % width == 0 {
            println!("");
        }
        print!("{} \t", tile);
    }
}

pub fn from_u8_gray_to_rgb(gray: u8) -> u32 {
    let gray = gray.saturating_mul(85) as u32;
    let (r, g, b) = (gray, gray, gray);
    (r << 16) | (g << 8) | b
}
#[cfg(test)]
mod tests {
    use crate::util::tiles_util::vram_to_tile_vec;

    use super::{tile_fuse_byte, VRAM_BYTE_SIZE};

    #[test]
    fn tile_fuse_byte_test() {
        let h = 0b0110_0000;
        let l = 0b0101_0000;
        let fused = tile_fuse_byte(l, h);
        assert_eq!(fused, vec![0, 3, 2, 1, 0, 0, 0, 0]);
        let h = 0b1010_0001;
        let l = 0b0100_0101;
        let fused = tile_fuse_byte(l, h);
        assert_eq!(fused, vec![2, 1, 2, 0, 0, 1, 0, 3]);
        let mut vec = vec![0, 8];
        vec.extend_from_slice(vec![1, 2, 0].as_slice());
        assert_eq!(vec, vec![0, 8, 1, 2, 0]);
    }
    #[test]
    fn tile_fuse_byte_test2() {
        let vec = vec![0x3C, 0x7E, 0x3C, 0x7E];
        let mut gray_vec: Vec<u32> = vec![];
        for byte_pair in vec.chunks(2) {
            gray_vec.extend_from_slice(&tile_fuse_byte(byte_pair[0], byte_pair[1]));
        }
        assert_eq!(
            gray_vec,
            vec![0, 2, 3, 3, 3, 3, 2, 0, 0, 2, 3, 3, 3, 3, 2, 0]
        );
    }

    #[test]
    fn vram_to_vec_test() {
        let mut vram = vec![0; VRAM_BYTE_SIZE];
        let tile_per_line = 4u8;
        for (i, e) in &mut vram.iter_mut().enumerate() {
            let a = i as u8 % 4;
            *e = a;
        }
        let tile_vec = vram_to_tile_vec(vram, tile_per_line);
        // assert_eq!(tile_vec.pixelcolor_vec[tile_per_line as usize * 2], 2);
        // assert_eq!(tile_vec.pixelcolor_vec[16], 2);
    }
}
