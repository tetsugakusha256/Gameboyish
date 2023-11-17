#[derive(Debug, PartialEq, Eq)]
pub enum PreOperands {
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    A,
    B,
    C,
    D,
    E,
    H,
    HL,
    L,
    INVALID,
}
impl From<String> for PreOperands {
    fn from(operand: String) -> Self {
        match operand.as_str() {
            "0" => Self::N0,
            "1" => Self::N1,
            "2" => Self::N2,
            "3" => Self::N3,
            "4" => Self::N4,
            "5" => Self::N5,
            "6" => Self::N6,
            "7" => Self::N7,
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            "E" => Self::E,
            "H" => Self::H,
            "HL" => Self::HL,
            "L" => Self::L,
            _ => Self::INVALID,
        }
    }
}
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum NopreOperands {
    X00,
    X08,
    X10,
    X18,
    X20,
    X28,
    X30,
    X38,
    A,
    AF,
    B,
    BC,
    C,
    D,
    DE,
    E,
    H,
    HL,
    L,
    NC,
    NZ,
    SP,
    Z,
    a16,
    a8,
    e8,
    n16,
    n8,
    INVALID,
}
impl From<String> for NopreOperands {
    fn from(operand: String) -> Self {
        match operand.as_str() {
            "$00" => Self::X00,
            "$08" => Self::X08,
            "$10" => Self::X10,
            "$18" => Self::X18,
            "$20" => Self::X20,
            "$28" => Self::X28,
            "$30" => Self::X30,
            "$38" => Self::X38,
            "A" => Self::A,
            "AF" => Self::AF,
            "B" => Self::B,
            "BC" => Self::BC,
            "C" => Self::C,
            "D" => Self::D,
            "DE" => Self::DE,
            "E" => Self::E,
            "H" => Self::H,
            "HL" => Self::HL,
            "L" => Self::L,
            "NC" => Self::NC,
            "NZ" => Self::NZ,
            "SP" => Self::SP,
            "Z" => Self::Z,
            "a16" => Self::a16,
            "a8" => Self::a8,
            "e8" => Self::e8,
            "n16" => Self::n16,
            "n8" => Self::n8,
            _ => Self::INVALID,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PrefixOpcodeMnemonics {
    BIT,
    INVALID,
    RES,
    RL,
    RLC,
    RR,
    RRC,
    SET,
    SLA,
    SRA,
    SRL,
    SWAP,
}
impl From<String> for PrefixOpcodeMnemonics {
    fn from(mnemonic: String) -> Self {
        match mnemonic.as_str() {
            "BIT" => Self::BIT,
            "RES" => Self::RES,
            "RL" => Self::RL,
            "RLC" => Self::RLC,
            "RR" => Self::RR,
            "RRC" => Self::RRC,
            "SET" => Self::SET,
            "SLA" => Self::SLA,
            "SRA" => Self::SRA,
            "SRL" => Self::SRL,
            "SWAP" => Self::SWAP,
            _ => Self::INVALID,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum NopreOpcodeMnemonics {
    ADC,
    ADD,
    AND,
    CALL,
    CCF,
    CP,
    CPL,
    DAA,
    DEC,
    DI,
    EI,
    HALT,
    INC,
    IllegalD3,
    IllegalDb,
    IllegalDd,
    IllegalE3,
    IllegalE4,
    IllegalEb,
    IllegalEc,
    IllegalEd,
    IllegalF4,
    IllegalFc,
    IllegalFd,
    JP,
    JR,
    LD,
    LDH,
    NOP,
    OR,
    POP,
    PREFIX,
    PUSH,
    RET,
    RETI,
    RLA,
    RLCA,
    RRA,
    RRCA,
    RST,
    SBC,
    SCF,
    STOP,
    SUB,
    XOR,
    INVALID,
}
impl From<String> for NopreOpcodeMnemonics {
    fn from(mnemonic: String) -> Self {
        match mnemonic.as_str() {
            "ADC" => Self::ADC,
            "ADD" => Self::ADD,
            "AND" => Self::AND,
            "CALL" => Self::CALL,
            "CCF" => Self::CCF,
            "CP" => Self::CP,
            "CPL" => Self::CPL,
            "DAA" => Self::DAA,
            "DEC" => Self::DEC,
            "DI" => Self::DI,
            "EI" => Self::EI,
            "HALT" => Self::HALT,
            "ILLEGAL_D3" => Self::IllegalD3,
            "ILLEGAL_DB" => Self::IllegalDb,
            "ILLEGAL_DD" => Self::IllegalDd,
            "ILLEGAL_E3" => Self::IllegalE3,
            "ILLEGAL_E4" => Self::IllegalE4,
            "ILLEGAL_EB" => Self::IllegalEb,
            "ILLEGAL_EC" => Self::IllegalEc,
            "ILLEGAL_ED" => Self::IllegalEd,
            "ILLEGAL_F4" => Self::IllegalF4,
            "ILLEGAL_FC" => Self::IllegalFc,
            "ILLEGAL_FD" => Self::IllegalFd,
            "INC" => Self::INC,
            "JP" => Self::JP,
            "JR" => Self::JR,
            "LD" => Self::LD,
            "LDH" => Self::LDH,
            "NOP" => Self::NOP,
            "OR" => Self::OR,
            "POP" => Self::POP,
            "PREFIX" => Self::PREFIX,
            "PUSH" => Self::PUSH,
            "RET" => Self::RET,
            "RETI" => Self::RETI,
            "RLA" => Self::RLA,
            "RLCA" => Self::RLCA,
            "RRA" => Self::RRA,
            "RRCA" => Self::RRCA,
            "RST" => Self::RST,
            "SBC" => Self::SBC,
            "SCF" => Self::SCF,
            "STOP" => Self::STOP,
            "SUB" => Self::SUB,
            "XOR" => Self::XOR,
            _ => Self::INVALID,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::opcode_dict_util::{NopreOpcodeMnemonics, PrefixOpcodeMnemonics};

    #[test]
    fn opcode_mnemonics_from_string() {
        assert_eq!(
            NopreOpcodeMnemonics::NOP,
            NopreOpcodeMnemonics::from("NOP".to_string())
        );
        assert_eq!(
            NopreOpcodeMnemonics::INVALID,
            NopreOpcodeMnemonics::from("not a valid mnemonic".to_string())
        );
    }
    #[test]
    fn prefix_opcode_mnemonics_from_string() {
        assert_eq!(
            PrefixOpcodeMnemonics::BIT,
            PrefixOpcodeMnemonics::from("BIT".to_string())
        );
        assert_eq!(
            PrefixOpcodeMnemonics::INVALID,
            PrefixOpcodeMnemonics::from("not a valid mnemonic".to_string())
        );
    }
}
