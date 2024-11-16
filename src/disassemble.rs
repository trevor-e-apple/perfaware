use crate::assembly_types::{get_register_enum, register_to_assembly_name, OpCode, WordByte};
use crate::mov_mem::mov_mem;

/// get the 6-bit op code from the first byte of an instruction
fn get_opcode(byte: u8) -> OpCode {
    let opcode = (byte & 0b11111100) >> 2;

    if opcode == (OpCode::MovMem as u8) {
        OpCode::MovMem
    } else if opcode == (OpCode::RegisterImmediateMov as u8) {
        OpCode::RegisterImmediateMov
    } else {
        panic!("Unexpected opcode");
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
            OpCode::MovMem => {
                let (instruction, index_increment) = mov_mem(&machine_code, index);

                result.push_str(&instruction);
                index += index_increment;
            }
        }
    }

    result
}
