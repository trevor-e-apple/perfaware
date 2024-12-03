#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum OpCode {
    // 4 bit op codes - final 4 bits are irrelevant
    RegisterImmediateMov = 0b10110000,

    // 6 bit op codes - final 2 bits are irrelevant
    MovMem = 0b10001000,
    AddMemMem = 0b00000000,
    SubMemMem = 0b00101000,
    CmpMemMem = 0b00111000,
    ImmediateArithmetic = 0b10000000,
    ImmediateToAccumulator = 0b00000100,
    ImmediateFromAccumulator = 0b00101100,
    CmpImmediateToAccumulator = 0b00111100,

    // 8 bit opcodes
    JneJnz = 0b01110101,
    Je = 0b01110100,
    Jl = 0b01111100,
    Jle = 0b01111110,
    Jb = 0b01110010,
    Jbe = 0b01110110,
    Jp = 0b01111010,
}

/// get the 6-bit op code from the first byte of an instruction
/// byte: the byte containing the opcode
/// returns: an OpCode enum type
pub fn get_opcode(byte: u8) -> OpCode {
    let first_four_bits = byte & 0b11110000;
    if first_four_bits == (OpCode::RegisterImmediateMov as u8) {
        return OpCode::RegisterImmediateMov;
    }

    let first_six_bits = byte & 0b11111100;
    if first_six_bits == (OpCode::MovMem as u8) {
        return OpCode::MovMem;
    } else if first_six_bits == (OpCode::AddMemMem as u8) {
        return OpCode::AddMemMem;
    } else if first_six_bits == (OpCode::ImmediateArithmetic as u8) {
        return OpCode::ImmediateArithmetic;
    } else if first_six_bits == (OpCode::ImmediateToAccumulator as u8) {
        return OpCode::ImmediateToAccumulator;
    } else if first_six_bits == (OpCode::SubMemMem as u8) {
        return OpCode::SubMemMem;
    } else if first_six_bits == (OpCode::ImmediateFromAccumulator as u8) {
        return OpCode::ImmediateFromAccumulator;
    } else if first_six_bits == (OpCode::CmpMemMem as u8) {
        return OpCode::CmpMemMem;
    } else if first_six_bits == (OpCode::CmpImmediateToAccumulator as u8) {
        return OpCode::CmpImmediateToAccumulator;
    }

    if byte == (OpCode::JneJnz as u8) {
        OpCode::JneJnz
    } else if byte == (OpCode::Je as u8) {
        OpCode::Je
    } else if byte == (OpCode::Jl as u8) {
        OpCode::Jl
    } else if byte == (OpCode::Jle as u8) {
        OpCode::Jle
    } else if byte == (OpCode::Jb as u8) {
        OpCode::Jb
    } else if byte == (OpCode::Jbe as u8) {
        OpCode::Jbe
    } else if byte == (OpCode::Jp as u8) {
        OpCode::Jp
    } else {
        panic!("Unexpected opcode");
    }
}

#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum Register {
    // Byte reg
    Al = 0b0000,
    Cl = 0b0001,
    Dl = 0b0010,
    Bl = 0b0011,
    Ah = 0b0100,
    Ch = 0b0101,
    Dh = 0b0110,
    Bh = 0b0111,

    // Word reg
    Ax = 0b1000,
    Cx = 0b1001,
    Dx = 0b1010,
    Bx = 0b1011,
    Sp = 0b1100,
    Bp = 0b1101,
    Si = 0b1110,
    Di = 0b1111,
}

const TO_REGISTER: [Register; 16] = [
    Register::Al,
    Register::Cl,
    Register::Dl,
    Register::Bl,
    Register::Ah,
    Register::Ch,
    Register::Dh,
    Register::Bh,
    Register::Ax,
    Register::Cx,
    Register::Dx,
    Register::Bx,
    Register::Sp,
    Register::Bp,
    Register::Si,
    Register::Di,
];

pub fn get_register_enum(register_field: u8, word_byte_field: WordByte) -> Register {
    TO_REGISTER[(register_field as usize) + (8 * (word_byte_field as usize))]
}

pub fn get_rm_register_field(byte: u8, word_byte_field: WordByte) -> Register {
    let register_field = byte & 0b00000111;
    get_register_enum(register_field, word_byte_field)
}

pub fn register_to_assembly_name(register: Register) -> String {
    match register {
        Register::Al => "al".to_owned(),
        Register::Cl => "cl".to_owned(),
        Register::Dl => "dl".to_owned(),
        Register::Bl => "bl".to_owned(),
        Register::Ah => "ah".to_owned(),
        Register::Ch => "ch".to_owned(),
        Register::Dh => "dh".to_owned(),
        Register::Bh => "bh".to_owned(),
        Register::Ax => "ax".to_owned(),
        Register::Cx => "cx".to_owned(),
        Register::Dx => "dx".to_owned(),
        Register::Bx => "bx".to_owned(),
        Register::Sp => "sp".to_owned(),
        Register::Bp => "bp".to_owned(),
        Register::Si => "si".to_owned(),
        Register::Di => "di".to_owned(),
    }
}

#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum WordByte {
    Byte = 0b0,
    Word = 0b1,
}

impl From<u8> for WordByte {
    fn from(value: u8) -> Self {
        if value == 0 {
            WordByte::Byte
        } else if value == 1 {
            WordByte::Word
        } else {
            panic!("Unable to convert")
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    RegRm = 0b0,
    RmReg = 0b1,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        if value == 0 {
            Direction::RegRm
        } else if value == 1 {
            Direction::RmReg
        } else {
            panic!("Unable to convert")
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum Mode {
    MemNoDisplacement = 0b00,
    Mem8BitDisplacement = 0b01,
    Mem16BitDisplacement = 0b10,
    Register = 0b11,
}

impl From<u8> for Mode {
    fn from(value: u8) -> Self {
        if value == 0b00 {
            Mode::MemNoDisplacement
        } else if value == 0b01 {
            Mode::Mem8BitDisplacement
        } else if value == 0b10 {
            Mode::Mem16BitDisplacement
        } else if value == 0b11 {
            Mode::Register
        } else {
            panic!("Bad mode value")
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum ArithmeticOpCode {
    Add = 0b000,
    Sub = 0b101,
    Cmp = 0b111,
}

impl From<u8> for ArithmeticOpCode {
    fn from(value: u8) -> Self {
        if value == 0b000 {
            ArithmeticOpCode::Add
        } else if value == 0b101 {
            ArithmeticOpCode::Sub
        } else if value == 0b111 {
            ArithmeticOpCode::Cmp
        } else {
            panic!("Bad mode value")
        }
    }
}
