use super::error_type::Errors;
use std::{collections::HashMap, error::Error, fs::File, io::Read};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
struct Operand {
    name: String,
    immediate: bool,
    bytes: Option<usize>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
struct Flags {
    Z: String,
    N: String,
    H: String,
    C: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
struct Instruction {
    mnemonic: String,
    bytes: usize,
    cycles: Vec<usize>,
    operands: Vec<Operand>,
    immediate: bool,
    flags: Flags,
}

// TODO: try converting to a vec and access by opcode as it's exactly the index
#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct Instructions(HashMap<String, Instruction>);

pub fn load_json(file_path: &str) -> Result<(), Errors> {
    let mut file = File::open(file_path)?;
    /// Load data from disk
    pub fn load(file: &mut File) -> Result<Instructions, Errors> {
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let deserialized: Instructions = serde_json::from_str(&content)?;
        Ok(deserialized)
    }
    let data = load(&mut file);

    let mut test = data.unwrap();
    let a = test.0.entry("0x02".to_string()).or_insert(Instruction {
        mnemonic: "ters".to_string(),
        bytes: 2,
        cycles: vec![2],
        operands: vec![],
        immediate: true,
        flags: Flags {
            Z: "a".to_string(),
            N: "a".to_string(),
            H: "a".to_string(),
            C: "a".to_string(),
        },
    });
    println!("test :{:?}", &a);
    Ok(())
}
