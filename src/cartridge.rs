use crate::util::error_type::Errors;
use std::{fs::File, io::Read, os::unix::prelude::FileExt};

// think of data type
struct CartridgeData(bool);
struct CartridgeHeader {}

pub fn load(file_path: &String) -> Result<bool, Errors> {
    let file = File::open(file_path)?;
    let mut buf = [0u8; 9];
    let _ = file.read_at(&mut buf, 0x0134);
    return Ok(true);
}

#[cfg(test)]
mod tests {
    #[test]
    fn load_tetris() {
        assert_eq!(0x0100, 0x0100);
    }
}
