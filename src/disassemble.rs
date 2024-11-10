#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
enum OpCode {
    RegisterRegisterMov = 0b100010,
}

/// get the 6-bit op code from the first byte of an instruction
fn get_opcode(byte: u8) -> OpCode {
    let opcode = (byte & 0b11111100) >> 2;

    if opcode == (OpCode::RegisterRegisterMov as u8) {
        OpCode::RegisterRegisterMov
    } else {
        panic!("Unexpected opcode");
    }
}

/// get the direction from the first byte of an instruction. return value is always 0 or 1
fn get_direction(byte: u8) -> u8 {
    (byte & 0b00000010) >> 1
}

#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
enum WordByte {
    Byte = 0b0,
    Word = 0b1,
}

/// get the word-byte field from the first byte of an instruction. return value is always 0 or 1
fn get_word_byte_field(byte: u8) -> WordByte {
    let word_byte = byte & 0b00000001;
    if word_byte == WordByte::Byte as u8 {
        WordByte::Byte
    } else if word_byte == WordByte::Word as u8 {
        WordByte::Word
    } else {
        panic!("Unexpected word byte bit value")
    }
}

#[repr(u8)]
#[derive(PartialEq)]
enum Mode {
    MemNoDisplacement = 0b00,
    Mem8Displacement = 0b01,
    Mem16Displacement = 0b10,
    Reg = 0b11,
}

fn get_mode_field(byte: u8) -> Mode {
    let mode = (byte & 0b11000000) >> 6;

    if mode == (Mode::MemNoDisplacement as u8) {
        Mode::MemNoDisplacement
    } else if mode == (Mode::Mem8Displacement as u8) {
        Mode::Mem8Displacement
    } else if mode == (Mode::Mem16Displacement as u8) {
        Mode::Mem16Displacement
    } else if mode == (Mode::Reg as u8) {
        Mode::Reg
    } else {
        panic!("Unexpected mode field")
    }
}

#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
enum Register {
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

fn get_register_enum(register_field: u8, word_byte_field: WordByte) -> Register {
    if word_byte_field == WordByte::Byte {
        TO_REGISTER[register_field as usize]
    } else if word_byte_field == WordByte::Word {
        TO_REGISTER[(register_field as usize) + 8]
    } else {
        panic!("Bad")
    }
}

/// get the register field from the second byte
fn get_register_field(byte: u8, word_byte_field: WordByte) -> Register {
    let register_field = (byte & 0b00111000) >> 3;
    get_register_enum(register_field, word_byte_field)
}

fn get_rm_register_field(byte: u8, word_byte_field: WordByte) -> Register {
    let register_field = byte & 0b00000111;
    get_register_enum(register_field, word_byte_field)
}

fn register_to_assembly_name(register: Register) -> String {
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

pub fn disassemble(machine_code: Vec<u8>) -> String {
    let mut result = "bits 16\n".to_owned();

    let mut index = 0;

    while index < machine_code.len() {
        let first_byte = machine_code[index];
        let opcode = get_opcode(first_byte);

        if opcode == OpCode::RegisterRegisterMov {
            let direction = get_direction(first_byte);
            let word_byte = get_word_byte_field(first_byte);

            let second_byte = machine_code[index + 1];
            let mode = get_mode_field(second_byte);
            let register = get_register_field(second_byte, word_byte);

            let instruction = match mode {
                Mode::MemNoDisplacement => todo!(),
                Mode::Mem8Displacement => todo!(),
                Mode::Mem16Displacement => todo!(),
                Mode::Reg => {
                    let second_register = get_rm_register_field(second_byte, word_byte);

                    let (src_register, dest_register) = if direction == 0 {
                        (register, second_register)
                    } else if direction == 1 {
                        (second_register, register)
                    } else {
                        panic!("Bad")
                    };

                    format!(
                        "mov {}, {}\n",
                        register_to_assembly_name(dest_register),
                        register_to_assembly_name(src_register)
                    )
                }
            };

            result.push_str(&instruction);
            index += 2;
        } else {
        }
    }

    result
}
