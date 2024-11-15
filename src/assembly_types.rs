#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum OpCode {
    RegisterImmediateMov = 0b1011,
    MovMem = 0b100010,
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
