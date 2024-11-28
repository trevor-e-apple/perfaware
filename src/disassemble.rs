use crate::byte_operations::concat_bytes;
use crate::common_assembly::{
    get_register_enum, get_rm_register_field, register_to_assembly_name, ArithmeticOpCode,
    Direction, Mode, OpCode, WordByte,
};

/// get the 6-bit op code from the first byte of an instruction
/// byte: the byte containing the opcode
/// returns: an OpCode enum type
fn get_opcode(byte: u8) -> OpCode {
    let first_four_bits = (byte & 0b11110000) >> 2;
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
    } else if first_six_bits == (OpCode::ImmediateToAccumulator as u8) {
        OpCode::ImmediateToAccumulator
    } else if first_six_bits == (OpCode::SubMemMem as u8) {
        OpCode::SubMemMem
    } else if first_six_bits == (OpCode::ImmediateFromAccumulator as u8) {
        OpCode::ImmediateFromAccumulator
    } else if first_six_bits == (OpCode::CmpMemMem as u8) {
        OpCode::CmpMemMem
    } else if first_six_bits == (OpCode::CmpImmediateToAccumulator as u8) {
        OpCode::CmpImmediateToAccumulator
    } else {
        panic!("Unexpected opcode");
    }
}

/// Returns a string and the number of bytes in the displacement for a no-displacement mov
/// rm_field: the rm_field
/// machine_code: the machine code vector
/// index: The index of the opcode-containing byte
/// returns: the string for the address and the number of bytes in the displacement (direct address case only)
pub fn no_displacement_address(
    rm_field: u8,
    machine_code: &Vec<u8>,
    index: usize,
) -> (String, usize) {
    if rm_field == 0b000 {
        ("[bx + si]".to_owned(), 0)
    } else if rm_field == 0b001 {
        ("[bx + di]".to_owned(), 0)
    } else if rm_field == 0b010 {
        ("[bp + si]".to_owned(), 0)
    } else if rm_field == 0b011 {
        ("[bp + di]".to_owned(), 0)
    } else if rm_field == 0b100 {
        ("si".to_owned(), 0)
    } else if rm_field == 0b101 {
        ("di".to_owned(), 0)
    } else if rm_field == 0b110 {
        let low_byte = match machine_code.get(index + 2) {
            Some(low_byte) => *low_byte,
            None => panic!("Failed to fetch low byte for direct address"),
        };
        let high_byte = match machine_code.get(index + 3) {
            Some(high_byte) => *high_byte,
            None => panic!("Failed to fetch high byte for direct address"),
        };
        let displacement = concat_bytes(high_byte, low_byte);
        (format!("{}", displacement), 2)
    } else if rm_field == 0b111 {
        ("bx".to_owned(), 0)
    } else {
        panic!("Bad rm field")
    }
}

/// Returns a string and the number of bytes in the displacement for a no-displacement arithmetic
/// instruction. A separate function from the mov version b/c we need brackets around all addresses
/// returned, not just some.
/// rm_field: the rm_field
/// machine_code: the machine code vector
/// index: The index of the opcode-containing byte
/// returns: the string for the address and the number of bytes in the displacement (direct address case only)
pub fn no_displacement_address_arithmetic(
    rm_field: u8,
    machine_code: &Vec<u8>,
    index: usize,
) -> (String, usize) {
    if rm_field == 0b000 {
        ("[bx + si]".to_owned(), 0)
    } else if rm_field == 0b001 {
        ("[bx + di]".to_owned(), 0)
    } else if rm_field == 0b010 {
        ("[bp + si]".to_owned(), 0)
    } else if rm_field == 0b011 {
        ("[bp + di]".to_owned(), 0)
    } else if rm_field == 0b100 {
        ("[si]".to_owned(), 0)
    } else if rm_field == 0b101 {
        ("[di]".to_owned(), 0)
    } else if rm_field == 0b110 {
        let low_byte = match machine_code.get(index + 2) {
            Some(low_byte) => *low_byte,
            None => panic!("Failed to fetch low byte for direct address"),
        };
        let high_byte = match machine_code.get(index + 3) {
            Some(high_byte) => *high_byte,
            None => panic!("Failed to fetch high byte for direct address"),
        };
        let displacement = concat_bytes(high_byte, low_byte);
        (format!("[{}]", displacement), 2)
    } else if rm_field == 0b111 {
        ("[bx]".to_owned(), 0)
    } else {
        panic!("Bad rm field")
    }
}

/// Takes the rm_field and returns the corresponding displacement address
/// rm_field: the rm_field
/// displacement: The displacement from the address
pub fn rm_field_to_displacement<T: std::fmt::Display>(rm_field: u8, displacement: T) -> String {
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

/// get the disassembly string and the number of bytes that were a part of the instruction for
/// any disassembly with the form [opcode:6 d:1 w:1] [mod:2 reg:3 rm:3] [disp-lo] [disp-hi]
pub fn mem_mem_disassembly(
    assembly_name: String,
    machine_code: &Vec<u8>,
    index: usize,
) -> (String, usize) {
    let first_byte = machine_code[index];

    let direction: Direction = ((first_byte & 0b00000010) >> 1).into();
    let word_byte: WordByte = (first_byte & 0b00000001).into();

    let second_byte = machine_code[index + 1];
    let mode: Mode = ((second_byte & 0b11000000) >> 6).into();
    let register_field = (second_byte & 0b00111000) >> 3;
    let register = get_register_enum(register_field, word_byte);

    let (instruction, index_increment) = match mode {
        Mode::MemNoDisplacement => {
            let rm_field = second_byte & 0b00000111;

            let (address_calculation, displacement_byte_count) =
                no_displacement_address(rm_field, &machine_code, index);

            let (dest, source) = match direction {
                Direction::RegRm => (address_calculation, register_to_assembly_name(register)),
                Direction::RmReg => (register_to_assembly_name(register), address_calculation),
            };

            (
                format!("{} {}, {}\n", assembly_name, dest, source),
                2 + displacement_byte_count,
            )
        }
        Mode::Mem8BitDisplacement => {
            let rm_field = second_byte & 0b0000111;
            let displacement = machine_code[index + 2];
            let address_calculation = rm_field_to_displacement(rm_field, displacement);

            let (dest, source) = match direction {
                Direction::RegRm => (address_calculation, register_to_assembly_name(register)),
                Direction::RmReg => (register_to_assembly_name(register), address_calculation),
            };

            (format!("{} {}, {}\n", assembly_name, dest, source), 3)
        }
        Mode::Mem16BitDisplacement => {
            let rm_field = second_byte & 0b0000111;
            let displacement = concat_bytes(machine_code[index + 3], machine_code[index + 2]);
            let address_calculation = rm_field_to_displacement(rm_field, displacement);

            let (dest, source) = match direction {
                Direction::RegRm => (address_calculation, register_to_assembly_name(register)),
                Direction::RmReg => (register_to_assembly_name(register), address_calculation),
            };

            (format!("{} {}, {}\n", assembly_name, dest, source), 4)
        }
        Mode::Register => {
            let second_register = get_rm_register_field(second_byte, word_byte);

            let (src_register, dest_register) = match direction {
                Direction::RegRm => (register, second_register),
                Direction::RmReg => (second_register, register),
            };

            (
                format!(
                    "{} {}, {}\n",
                    assembly_name,
                    register_to_assembly_name(dest_register),
                    register_to_assembly_name(src_register)
                ),
                2,
            )
        }
    };

    (instruction, index_increment)
}

/// common function for accumulator arithmetic
/// operation: the string for the operation. e.g. 'add', 'sub', 'cmp'
/// machine_code: the vector containing the machine code
/// index: the index for the first byte (containing the opcode)
fn accumulator_arithmetic(
    operation: &str,
    machine_code: &Vec<u8>,
    index: usize,
) -> (String, usize) {
    let first_byte = machine_code[index];

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

    let instruction = format!("{} {}, {}\n", operation, register, data);

    (instruction, index_increment)
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
                    mem_mem_disassembly("mov".to_owned(), &machine_code, index);

                result.push_str(&instruction);
                index += index_increment;
            }
            OpCode::AddMemMem => {
                let (instruction, index_increment) =
                    mem_mem_disassembly("add".to_owned(), &machine_code, index);

                result.push_str(&instruction);
                index += index_increment;
            }
            OpCode::SubMemMem => {
                let (instruction, index_increment) =
                    mem_mem_disassembly("sub".to_owned(), &machine_code, index);

                result.push_str(&instruction);
                index += index_increment;
            }
            OpCode::CmpMemMem => {
                let (instruction, index_increment) =
                    mem_mem_disassembly("cmp".to_owned(), &machine_code, index);

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

                let (dest_arg, immediate, index_increment) = match mode {
                    Mode::MemNoDisplacement => {
                        let rm_field = second_byte & 0b00000111;

                        let (address_calculation, displacement_bytes) =
                            no_displacement_address_arithmetic(rm_field, &machine_code, index);

                        // 2 bytes + displacment bytes is the low data byte
                        let low_byte_index = 2 + displacement_bytes;
                        // 2 bytes + displacment bytes + 1 low byte + 1 is the high data byte
                        let high_byte_index = 3 + displacement_bytes;

                        let (immediate, data_increment) = get_immediate(
                            &machine_code,
                            index,
                            low_byte_index,
                            high_byte_index,
                            word_byte,
                            sign_extension,
                        );

                        (
                            format!("{} {}", word_byte_string, address_calculation),
                            immediate,
                            2 + displacement_bytes + data_increment,
                        )
                    }
                    Mode::Mem8BitDisplacement => {
                        let rm_field = second_byte & 0b0000111;
                        let displacement = machine_code[index + 2];
                        let address_calculation = rm_field_to_displacement(rm_field, displacement);

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
                        let address_calculation = rm_field_to_displacement(rm_field, displacement);

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
                    ArithmeticOpCode::Cmp => {
                        let instruction = format!("cmp {}, {}\n", dest_arg, immediate);

                        instruction
                    }
                };

                result.push_str(&instruction);

                index += index_increment;
            }
            OpCode::ImmediateToAccumulator => {
                let (instruction, index_increment) =
                    accumulator_arithmetic("add", &machine_code, index);

                result.push_str(&instruction);
                index += index_increment;
            }
            OpCode::ImmediateFromAccumulator => {
                let (instruction, index_increment) =
                    accumulator_arithmetic("sub", &machine_code, index);

                result.push_str(&instruction);
                index += index_increment;
            }
            OpCode::CmpImmediateToAccumulator => {
                let (instruction, index_increment) =
                    accumulator_arithmetic("cmp", &machine_code, index);
                result.push_str(&instruction);
                index += index_increment;
            }
        }
    }

    result
}
