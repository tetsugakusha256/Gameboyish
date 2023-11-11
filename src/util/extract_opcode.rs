use std::{fs::File, io::Read, error::Error};
use super::error_type::Errors;

pub fn load_json(file_path: &str)-> Result<(), Errors>{
    let mut file = File::open(file_path)?;
    
    Ok(())
}
