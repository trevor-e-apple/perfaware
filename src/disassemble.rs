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

fn get_register_enum(register_field: u8, word_byte_field: WordByte) -> Register {
    TO_REGISTER[(register_field as usize) + (8 * (word_byte_field as usize))]
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

#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
enum WordByte {
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
enum Mode {
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

pub fn disassemble(machine_code: Vec<u8>) -> String {
    let mut result = "bits 16\n".to_owned();

    let mut index = 0;

    while index < machine_code.len() {
        let first_byte = machine_code[index];
        let opcode = get_opcode(first_byte);

        match opcode {
            OpCode::RegisterImmediateMov => {
                let word_byte: WordByte = ((first_byte & 0b00001000) >> 3).into();
                let register_field = first_byte & 0b00000111;
                let register = get_register_enum(register_field, word_byte);

                let (immediate, bytes_read) = match word_byte {
                    WordByte::Byte => {
                        let second_byte = machine_code[index + 1];
                        (second_byte as u16, 1)
                    }
                    WordByte::Word => {
                        let second_byte = machine_code[index + 1];
                        let third_byte = machine_code[index + 1];
                        let immediate = ((second_byte as u16) << 8) | (third_byte as u16);
                        (immediate, 2)
                    }
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
                let word_byte: WordByte = (first_byte & 0b00000001).into();

                let second_byte = machine_code[index + 1];
                let mode: Mode = ((second_byte & 0b11000000) >> 6).into();
                let register_field = (second_byte & 0b00111000) >> 3;
                let register = get_register_enum(register_field, word_byte);

                let instruction = match mode {
                    Mode::MemNoDisplacement => {
                        let rm_field = second_byte & 0b00000111;
                        let address_calculation = {
                            let address_calculation = if rm_field == 0b000 {
                                "[bx + si]"
                            } else if rm_field == 0b001 {
                                "[bx + di]"
                            } else if rm_field == 0b010 {
                                "[bp + si]"
                            } else if rm_field == 0b011 {
                                "[bp + di]"
                            } else if rm_field == 0b100 {
                                "si"
                            } else if rm_field == 0b101 {
                                "di"
                            } else if rm_field == 0b110 {
                                todo!("Need to get direct address data, get displacement")
                            } else if rm_field == 0b111 {
                                "bx"
                            } else {
                                panic!("Bad rm field")
                            };
                            address_calculation.to_owned()
                        };

                        let (dest, source) = if direction == 0 {
                            (address_calculation, register_to_assembly_name(register))
                        } else if direction == 1 {
                            (register_to_assembly_name(register), address_calculation)
                        } else {
                            panic!("Unexpected direction")
                        };

                        format!("mov {}, {}\n", dest, source)
                    }
                    Mode::Mem8BitDisplacement => todo!(),
                    Mode::Mem16BitDisplacement => todo!(),
                    Mode::Register => {
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
            }
        }
    }

    result
}
