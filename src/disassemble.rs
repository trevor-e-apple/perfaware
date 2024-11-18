use crate::byte_operations::concat_bytes;
use crate::common_assembly::{
    get_direction_wordbyte_fields, get_register_enum, register_to_assembly_name, OpCode, WordByte,
};
use crate::mov_mem::mov_mem;

/// get the 6-bit op code from the first byte of an instruction
fn get_opcode(byte: u8) -> OpCode {
    let first_four_bits = (byte & 0b11110000) >> 4;
    if first_four_bits == (OpCode::RegisterImmediateMov as u8) {
        return OpCode::RegisterImmediateMov;
    }

    let first_six_bits = (byte & 0b11111100) >> 2;
    if first_six_bits == (OpCode::MovMem as u8) {
        OpCode::MovMem
    } else if first_six_bits == (OpCode::AddMemMem as u8) {
        OpCode::AddMemMem
    } else {
        panic!("Unexpected opcode");
    }
}

/// perform disassembly
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

                let (immediate, immediate_bytes) = match word_byte {
                    WordByte::Byte => {
                        let second_byte = machine_code[index + 1];
                        (second_byte as u16, 1)
                    }
                    WordByte::Word => {
                        let second_byte = machine_code[index + 1];
                        let third_byte = machine_code[index + 2];
                        let immediate = concat_bytes(third_byte, second_byte);
                        (immediate, 2)
                    }
                };

                let instruction = format!(
                    "mov {}, {}\n",
                    register_to_assembly_name(register),
                    immediate
                );

                result.push_str(&instruction);

                // 1 byte for the opcode + the number of bytes in the immediate
                index += immediate_bytes + 1;
            }
            OpCode::MovMem => {
                let (instruction, index_increment) = mov_mem(&machine_code, index);

                result.push_str(&instruction);
                index += index_increment;
            }
            OpCode::AddMemMem => {
                let (direction, word_byte) = get_direction_wordbyte_fields(first_byte);
            }
        }
    }

    result
}
