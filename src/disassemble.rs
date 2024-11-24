use crate::arithmetic_disassembly::{arithmetic_diassembly, no_displacement_address};
use crate::byte_operations::concat_bytes;
use crate::common_assembly::{
    displacement_address, get_register_enum, get_rm_register_field, register_to_assembly_name,
    ArithmeticOpCode, Mode, OpCode, WordByte,
};

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
    } else if first_six_bits == (OpCode::ImmediateArithmetic as u8) {
        OpCode::ImmediateArithmetic
    } else if first_six_bits == (OpCode::ImmediateAccumulator as u8) {
        OpCode::ImmediateAccumulator
    } else if first_six_bits == (OpCode::SubMemMem as u8) {
        OpCode::SubMemMem
    } else {
        panic!("Unexpected opcode");
    }
}

fn get_immediate(
    machine_code: &Vec<u8>,
    index: usize,
    low_byte_index: usize,
    high_byte_index: usize,
    word_byte: WordByte,
    sign_extension: u8,
) -> (u16, usize) {
    let byte_value = (machine_code[index + low_byte_index] as u16, 1);

    match word_byte {
        WordByte::Byte => byte_value,
        WordByte::Word => {
            if sign_extension == 0 {
                (
                    concat_bytes(
                        machine_code[index + high_byte_index],
                        machine_code[index + low_byte_index],
                    ),
                    2,
                )
            } else {
                byte_value
            }
        }
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
                let second_byte = machine_code[index + 1];

                let (immediate, immediate_bytes) = match word_byte {
                    WordByte::Byte => (second_byte as u16, 1),
                    WordByte::Word => {
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
                let (instruction, index_increment) =
                    arithmetic_diassembly("mov".to_owned(), &machine_code, index);

                result.push_str(&instruction);
                index += index_increment;
            }
            OpCode::AddMemMem => {
                let (instruction, index_increment) =
                    arithmetic_diassembly("add".to_owned(), &machine_code, index);

                result.push_str(&instruction);
                index += index_increment;
            }
            OpCode::SubMemMem => {
                let (instruction, index_increment) =
                    arithmetic_diassembly("sub".to_owned(), &machine_code, index);

                result.push_str(&instruction);
                index += index_increment;
            }
            OpCode::ImmediateArithmetic => {
                let word_byte: WordByte = (first_byte & 0b00000001).into();
                let word_byte_string = match word_byte {
                    WordByte::Byte => "byte".to_owned(),
                    WordByte::Word => "word".to_owned(),
                };

                let sign_extension = (first_byte & 0b00000010) >> 1;

                let second_byte = machine_code[index + 1];
                let mode: Mode = ((second_byte & 0b11000000) >> 6).into();
                let arithmetic_code: ArithmeticOpCode = ((second_byte & 0b00111000) >> 3).into();
                // let rm_field = second_byte & 0b00000111;

                // let third_byte = machine_code[index + 2];
                // let fourth_byte = machine_code[index + 3];

                let (dest_arg, immediate, index_increment) = match mode {
                    Mode::MemNoDisplacement => {
                        let rm_field = second_byte & 0b00000111;
                        // TODO: do we need high and low bytes here???
                        // TODO: we need a better way of unifying no_displacement address...
                        let (address_calculation, address_increment) =
                            no_displacement_address(rm_field, 0, 0);

                        let (immediate, data_increment) =
                            get_immediate(&machine_code, index, 2, 3, word_byte, sign_extension);

                        (
                            format!("{} [{}]", word_byte_string, address_calculation),
                            immediate,
                            2 + data_increment,
                        )
                    }
                    Mode::Mem8BitDisplacement => {
                        let rm_field = second_byte & 0b0000111;
                        let displacement = machine_code[index + 2];
                        let address_calculation = displacement_address(rm_field, displacement);

                        let (immediate, data_increment) =
                            get_immediate(&machine_code, index, 3, 4, word_byte, sign_extension);

                        (
                            format!("{} {}", word_byte_string, address_calculation),
                            immediate,
                            3 + data_increment,
                        )
                    }
                    Mode::Mem16BitDisplacement => {
                        let rm_field = second_byte & 0b0000111;
                        let displacement =
                            concat_bytes(machine_code[index + 3], machine_code[index + 2]);
                        let address_calculation = displacement_address(rm_field, displacement);

                        let (immediate, data_increment) =
                            get_immediate(&machine_code, index, 4, 5, word_byte, sign_extension);

                        (
                            format!("{} {}", word_byte_string, address_calculation),
                            immediate,
                            4 + data_increment,
                        )
                    }
                    Mode::Register => {
                        let register = get_rm_register_field(second_byte, word_byte);
                        let name = register_to_assembly_name(register);
                        let third_byte = machine_code[index + 2];
                        (format!("{}", name), third_byte as u16, 3)
                    }
                };

                let instruction = match arithmetic_code {
                    ArithmeticOpCode::Add => {
                        let instruction = format!("add {}, {}\n", dest_arg, immediate);

                        instruction
                    }
                    ArithmeticOpCode::Sub => {
                        let instruction = format!("sub {}, {}\n", dest_arg, immediate);

                        instruction
                    }
                    ArithmeticOpCode::Cmp => todo!(),
                };

                result.push_str(&instruction);

                index += index_increment;
            }
            OpCode::ImmediateAccumulator => {
                let word_byte: WordByte = (first_byte & 0b00000001).into();
                let (data, register, index_increment) = match word_byte {
                    WordByte::Byte => {
                        let data = machine_code[index + 1] as u16;
                        (data, "al", 2)
                    }
                    WordByte::Word => {
                        let data = concat_bytes(machine_code[index + 2], machine_code[index + 1]);
                        (data, "ax", 3)
                    }
                };

                let instruction = format!("add {}, {}\n", register, data);

                result.push_str(&instruction);
                index += index_increment;
            }
        }
    }

    result
}
