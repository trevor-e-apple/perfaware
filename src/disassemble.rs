#[repr(u8)]
#[derive(PartialEq)]
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

/// get the word-byte field from the first byte of an instruction. return value is always 0 or 1
fn get_word_byte_field(byte: u8) -> u8 {
    todo!("Probably should make this an enum");
    byte & 0b00000001
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

enum Register {
    // Byte reg
    Al,

    // Word reg
    Ax,
}

fn get_register_field(byte: u8, word_byte_field: u8) -> Register {
    let register_field = (byte & 0b00111000) >> 3;
    if word_byte_field == 0 {
        if register_field == 0b000 {
            Register::Al
        } else {
            panic!("Bad")
        }
    } else if word_byte_field == 1 {
        if register_field == 0b000 {
            Register::Ax
        } else {
            panic!("Bad")
        }
    } else {
        panic!("Bad")
    }
}

pub fn disassemble(machine_code: Vec<u8>) -> String {
    let result = "bits 16\n".to_owned();

    let mut index = 0;

    while index < machine_code.len() {
        let first_byte = machine_code[index];
        let opcode = get_opcode(first_byte);

        if opcode == OpCode::RegisterRegisterMov {
            let direction = get_direction(first_byte);
            let word_byte = get_word_byte_field(first_byte);

            let second_byte = machine_code[index + 1];
            let mode = get_mode_field(second_byte);
            let register = get_register_field(second_byte);

            match mode {
                Mode::MemNoDisplacement => todo!(),
                Mode::Mem8Displacement => todo!(),
                Mode::Mem16Displacement => todo!(),
                Mode::Reg => {}
            }

            index += 2;
        } else {
        }
    }

    result
}
