use crate::byte_operations::concat_bytes;
use crate::common_assembly::{
    get_direction_wordbyte_fields, get_register_enum, get_rm_register_field,
    register_to_assembly_name, Direction, Mode,
};

/// Format the displacement address
fn displacement_address<T: std::fmt::Display>(rm_field: u8, displacement: T) -> String {
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

/// get the mov mem disassembly string and the number of bytes that were a part of the instruction
pub fn mov_mem(machine_code: &Vec<u8>, index: usize) -> (String, usize) {
    let first_byte = machine_code[index];
    let (direction, word_byte) = get_direction_wordbyte_fields(first_byte);

    let second_byte = machine_code[index + 1];
    let mode: Mode = ((second_byte & 0b11000000) >> 6).into();
    let register_field = (second_byte & 0b00111000) >> 3;
    let register = get_register_enum(register_field, word_byte);

    let (instruction, index_increment) = match mode {
        Mode::MemNoDisplacement => {
            let rm_field = second_byte & 0b00000111;
            let (address_calculation, index_increment) = if rm_field == 0b000 {
                ("[bx + si]".to_owned(), 2)
            } else if rm_field == 0b001 {
                ("[bx + di]".to_owned(), 2)
            } else if rm_field == 0b010 {
                ("[bp + si]".to_owned(), 2)
            } else if rm_field == 0b011 {
                ("[bp + di]".to_owned(), 2)
            } else if rm_field == 0b100 {
                ("si".to_owned(), 2)
            } else if rm_field == 0b101 {
                ("di".to_owned(), 2)
            } else if rm_field == 0b110 {
                let displacement = concat_bytes(machine_code[index + 3], machine_code[index + 2]);
                (format!("{}", displacement), 4)
            } else if rm_field == 0b111 {
                ("bx".to_owned(), 2)
            } else {
                panic!("Bad rm field")
            };

            let (dest, source) = match direction {
                Direction::RegRm => (address_calculation, register_to_assembly_name(register)),
                Direction::RmReg => (register_to_assembly_name(register), address_calculation),
            };

            (format!("mov {}, {}\n", dest, source), index_increment)
        }
        Mode::Mem8BitDisplacement => {
            let rm_field = second_byte & 0b0000111;
            let displacement = machine_code[index + 2];
            let address_calculation = displacement_address(rm_field, displacement);

            let (dest, source) = match direction {
                Direction::RegRm => (address_calculation, register_to_assembly_name(register)),
                Direction::RmReg => (register_to_assembly_name(register), address_calculation),
            };

            (format!("mov {}, {}\n", dest, source), 3)
        }
        Mode::Mem16BitDisplacement => {
            let rm_field = second_byte & 0b0000111;
            let displacement = concat_bytes(machine_code[index + 3], machine_code[index + 2]);
            let address_calculation = displacement_address(rm_field, displacement);

            let (dest, source) = match direction {
                Direction::RegRm => (address_calculation, register_to_assembly_name(register)),
                Direction::RmReg => (register_to_assembly_name(register), address_calculation),
            };

            (format!("mov {}, {}\n", dest, source), 4)
        }
        Mode::Register => {
            let second_register = get_rm_register_field(second_byte, word_byte);

            let (src_register, dest_register) = match direction {
                Direction::RegRm => (register, second_register),
                Direction::RmReg => (second_register, register),
            };

            (
                format!(
                    "mov {}, {}\n",
                    register_to_assembly_name(dest_register),
                    register_to_assembly_name(src_register)
                ),
                2,
            )
        }
    };

    (instruction, index_increment)
}
