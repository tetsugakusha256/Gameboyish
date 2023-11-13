use super::error_type::Errors;
use std::{collections::HashMap, error::Error, fs::File, io::Read};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Operand {
    pub name: String,
    pub immediate: bool,
    pub increment: Option<bool>,
    pub decrement: Option<bool>,
    pub bytes: Option<usize>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Flags {
    pub Z: String,
    pub N: String,
    pub H: String,
    pub C: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Instruction {
    pub mnemonic: String,
    pub bytes: usize,
    pub cycles: Vec<usize>,
    pub operands: Vec<Operand>,
    pub immediate: bool,
    pub flags: Flags,
}
impl Instruction{
    pub fn operands_tuple(&self) -> Option<(Operand,Operand)>{
        if self.operands.len() == 2 {
            return Some((self.operands[0].clone(),self.operands[1].clone()));
        }
        None
    }
}

// TODO: try converting to a vec and access by opcode as it's exactly the index
#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Instructions(HashMap<String, Instruction>);

pub fn load_json(file_path: &str) -> Result<Vec<Instruction>, Errors> {
    let mut file = File::open(file_path)?;
    /// Load data from disk
    pub fn load(file: &mut File) -> Result<Instructions, Errors> {
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let deserialized: Instructions = serde_json::from_str(&content)?;
        Ok(deserialized)
    }
    let data = load(&mut file);

    let test = data.unwrap();
    let vec_in_order: Vec<Instruction> = (0x00..=0xFF)
        .map(|num| test.0.get(&format!("0x{:02X}", num)).cloned().unwrap())
        .collect();
    let mut hash: HashMap<String, bool> = HashMap::new();
    for instruction in &vec_in_order {
        if instruction.operands.len() != 0 {
            for op in &instruction.operands {
                if !hash.contains_key(&op.name) {
                    hash.insert(op.name.clone(), true);
                }
            }
        }
    }
    for mnem in hash {
        println!("{}", mnem.0);
    }
    println!("test :{:?}", vec_in_order[0xAF]);
    Ok(vec_in_order)
}
