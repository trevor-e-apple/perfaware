#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum OpCode {
    RegisterImmediateMov = 0b101100,

    MovMem = 0b100010,
    AddMemMem = 0b000000,
    SubMemMem = 0b001010,
    ImmediateArithmetic = 0b100000,
    ImmediateToAccumulator = 0b000001,
    ImmediateFromAccumulator = 0b001011,
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

/// Format the displacement address
pub fn displacement_address<T: std::fmt::Display>(rm_field: u8, displacement: T) -> String {
    if rm_field == 0b000 {
        format!("[bx + si + {}]", displacement)
    } else if rm_field == 0b001 {
        format!("[bx + di + {}]", displacement)
    } else if rm_field == 0b010 {
        format!("[bp + si + {}]", displacement)
    } else if rm_field == 0b011 {
        format!("[bp + di + {}]", displacement)
    } else if rm_field == 0b100 {
        format!("[si + {}]", displacement)
    } else if rm_field == 0b101 {
        format!("[di + {}]", displacement)
    } else if rm_field == 0b110 {
        format!("[bp + {}]", displacement)
    } else if rm_field == 0b111 {
        format!("[bx + {}]", displacement)
    } else {
        panic!("Bad rm field")
    }
}
