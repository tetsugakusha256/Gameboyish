use super::u8_traits::Bit;

const TILE_BYTE_SIZE: usize = 16usize;
// const VRAM_BYTE_SIZE: usize = 6144usize;
const VRAM_BYTE_SIZE: usize = 512usize;
// TODO: make a struct that has a Vec<u8> and the length of a line
#[derive(Debug)]
struct TileVec {
    vector: Vec<u32>,
    tile_per_line: u8,
}
impl TileVec {
    pub fn new(vector: Vec<u32>, tile_per_line: u8) -> TileVec {
        TileVec {
            vector,
            tile_per_line,
        }
    }
}

fn tile_fuse_byte(h: u8, l: u8) -> Vec<u8> {
    let h_bits = h.to_bits_array();
    let l_bits = l.to_bits_array();
    let iterator = h_bits.iter().zip(l_bits.iter());
    iterator.map(|(h_bit, l_bit)| h_bit + l_bit).collect()
}

fn vram_to_vec(vram: Vec<u8>, tile_per_line: u8) -> TileVec {
    // buf of the vram size
    let tile_per_line = tile_per_line as usize;
    let byte_per_line = tile_per_line * 2;
    let mut buf: Vec<u32> = vec![0; VRAM_BYTE_SIZE];
    let mut test: Vec<u32> = vec![0; VRAM_BYTE_SIZE];
    for (i, byte) in vram.iter().enumerate() {
        let pair = byte_per_line * ((i / 2) % 8);
        let tile = (2 * (i / 16) + (i % 2)) % (tile_per_line * 2);
        let block = (8 * byte_per_line as usize) * (i / (8 * byte_per_line as usize));
        let offset = pair + tile + block;
        // println!("{:?}", (i, block));
        if offset < VRAM_BYTE_SIZE {
            if offset == 16 {
                println!("i:{}", i);
            }
            test[offset] = i as u32;
            buf[offset] = *byte as u32;
        }
    }
    print_tile_vec(TileVec::new(test, tile_per_line as u8));
    TileVec::new(buf, tile_per_line as u8)
}
fn print_tile_vec(tile_vec: TileVec) {
    let width = tile_vec.tile_per_line * 2;
    for (i, tile) in tile_vec.vector.iter().enumerate() {
        if i as u8 % width == 0 {
            println!("");
        }
        print!("{} \t", tile);
    }
}

#[cfg(test)]
mod tests {
    use crate::util::tiles_util::vram_to_vec;

    use super::{tile_fuse_byte, VRAM_BYTE_SIZE};

    #[test]
    fn tile_fuse_byte_test() {
        let h = 0b0110_0000;
        let l = 0b0101_0000;
        let fused = tile_fuse_byte(h, l);
        assert_eq!(fused, vec![0, 2, 1, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn vram_to_vec_test() {
        let mut vram = vec![0; VRAM_BYTE_SIZE];
        for (i, e) in &mut vram.iter_mut().enumerate() {
            let a = i as u8;
            *e = a;
        }
        let tile_vec = vram_to_vec(vram, 8);
        assert_eq!(tile_vec.vector[131], 2);
    }
}
