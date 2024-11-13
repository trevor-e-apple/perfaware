#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
enum OpCode {
    RegisterImmediateMov = 0b1011,
    RegisterRegisterMov = 0b100010,
}

/// get the 6-bit op code from the first byte of an instruction
fn get_opcode(byte: u8) -> OpCode {
    let opcode = (byte & 0b11111100) >> 2;

    if opcode == (OpCode::RegisterRegisterMov as u8) {
        OpCode::RegisterRegisterMov
    } else if opcode == (OpCode::RegisterImmediateMov as u8) {
        OpCode::RegisterImmediateMov
    } else {
        panic!("Unexpected opcode");
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

fn get_register_enum(register_field: u8, word_byte_field: u8) -> Register {
    TO_REGISTER[(register_field as usize) + (8 * (word_byte_field as usize))]
}

/// get the register field from the second byte
fn get_register_field(byte: u8, word_byte_field: u8) -> Register {
    let register_field = (byte & 0b00111000) >> 3;
    get_register_enum(register_field, word_byte_field)
}

fn get_rm_register_field(byte: u8, word_byte_field: u8) -> Register {
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

        match opcode {
            OpCode::RegisterImmediateMov => {
                let word_byte = (first_byte & 0b00001000) >> 3;
                let register_field = first_byte & 0b00000111;
                let register = get_register_field(register_field, word_byte);
                let (immediate, bytes_read) = if word_byte == 0 {
                    let second_byte = machine_code[index + 1];
                    (second_byte as u16, 1)
                } else if word_byte == 1 {
                    let second_byte = machine_code[index + 1];
                    let third_byte = machine_code[index + 1];
                    let immediate = ((second_byte as u16) << 8) | (third_byte as u16);
                    (immediate, 2)
                } else {
                    panic!("Bad word byte value");
                };

                let instruction = format!(
                    "mov {}, {}\n",
                    register_to_assembly_name(register),
                    immediate
                );

                result.push_str(&instruction);

                index += bytes_read;
            }
            OpCode::RegisterRegisterMov => {
                let direction = (first_byte & 0b00000010) >> 1;
                let word_byte = first_byte & 0b00000001;

                let second_byte = machine_code[index + 1];
                let mode = (second_byte & 0b11000000) >> 6;
                let register = get_register_field(second_byte, word_byte);

                let instruction = if mode == 0b00 {
                    todo!()
                } else if mode == 0b01 {
                    todo!()
                } else if mode == 0b10 {
                    todo!()
                } else if mode == 0b11 {
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
                } else {
                    panic!("Unexpected mode value")
                };

                result.push_str(&instruction);
                index += 2;
            }
        }
    }

    result
}
