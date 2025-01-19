use crate::byte_operations::concat_bytes;
use crate::common_assembly::{
    get_opcode, get_register_enum, get_rm_register_field, ArithmeticOpCode, Direction, Mode,
    OpCode, Register, WordByte,
};
use crate::disassemble::get_instruction;
use crate::simulator_state::{get_sim_state_diff, SimMem, SimulationState};

/// Returns a string and the number of bytes in the displacement for a no-displacement mov
/// rm_field: the rm_field
/// machine_code: the machine code vector
/// index: The index of the opcode-containing byte
/// returns: the memory address and the number of bytes in the displacement
pub fn no_displacement_address(
    sim_state: &SimulationState,
    rm_field: u8,
    machine_code: &Vec<u8>,
    index: usize,
) -> (usize, usize) {
    if rm_field == 0b000 {
        // ("[bx + si]".to_owned(), 0)
        ((sim_state.bx + sim_state.si) as usize, 0)
    } else if rm_field == 0b001 {
        // ("[bx + di]".to_owned(), 0)
        ((sim_state.bx + sim_state.di) as usize, 0)
    } else if rm_field == 0b010 {
        // ("[bp + si]".to_owned(), 0)
        ((sim_state.bp + sim_state.si) as usize, 0)
    } else if rm_field == 0b011 {
        // ("[bp + di]".to_owned(), 0)
        ((sim_state.bp + sim_state.di) as usize, 0)
    } else if rm_field == 0b100 {
        // ("si".to_owned(), 0)
        (sim_state.si as usize, 0)
    } else if rm_field == 0b101 {
        // ("di".to_owned(), 0)
        (sim_state.di as usize, 0)
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
        (displacement as usize, 2)
    } else if rm_field == 0b111 {
        // ("bx".to_owned(), 0)
        (sim_state.bx as usize, 0)
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
/// returns: the usize for the address and the number of bytes in the displacement (direct address case only)
pub fn no_displacement_address_arithmetic(
    rm_field: u8,
    machine_code: &Vec<u8>,
    index: usize,
) -> (usize, usize) {
    if rm_field == 0b000 {
        // ("[bx + si]".to_owned(), 0)
        todo!()
    } else if rm_field == 0b001 {
        // ("[bx + di]".to_owned(), 0)
        todo!()
    } else if rm_field == 0b010 {
        // ("[bp + si]".to_owned(), 0)
        todo!()
    } else if rm_field == 0b011 {
        // ("[bp + di]".to_owned(), 0)
        todo!()
    } else if rm_field == 0b100 {
        // ("[si]".to_owned(), 0)
        todo!()
    } else if rm_field == 0b101 {
        // ("[di]".to_owned(), 0)
        todo!()
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
        (displacement as usize, 2)
    } else if rm_field == 0b111 {
        // ("[bx]".to_owned(), 0)
        todo!()
    } else {
        panic!("Bad rm field")
    }
}

/// Takes the rm_field and returns the corresponding displacement address
/// rm_field: the rm_field
/// displacement: The displacement from the address
pub fn rm_field_to_displacement(
    sim_state: &SimulationState,
    rm_field: u8,
    displacement: u16,
) -> usize {
    if rm_field == 0b000 {
        (sim_state.bx + sim_state.si + displacement) as usize
    } else if rm_field == 0b001 {
        (sim_state.bx + sim_state.di + displacement) as usize
    } else if rm_field == 0b010 {
        (sim_state.bp + sim_state.si + displacement) as usize
    } else if rm_field == 0b011 {
        (sim_state.bp + sim_state.di + displacement) as usize
    } else if rm_field == 0b100 {
        (sim_state.si + displacement) as usize
    } else if rm_field == 0b101 {
        (sim_state.di + displacement) as usize
    } else if rm_field == 0b110 {
        (sim_state.bp + displacement) as usize
    } else if rm_field == 0b111 {
        (sim_state.bx + displacement) as usize
    } else {
        panic!("Bad rm field")
    }
}

fn simulate_mem_mem(
    sim_state: &mut SimulationState,
    sim_mem: &mut SimMem,
    direction: Direction,
    word_byte: WordByte,
    address_calculation: usize,
    register: Register,
) {
    match direction {
        Direction::RegRm => match word_byte {
            WordByte::Byte => {
                sim_mem.mem[address_calculation] = sim_state.get_register_value(register) as u8
            }
            WordByte::Word => {
                let register_value: u16 = sim_state.get_register_value(register);
                sim_mem.mem[address_calculation] = (register_value & 0xFF) as u8;
                sim_mem.mem[address_calculation + 1] = (register_value & 0xFF00) as u8;
            }
        },
        Direction::RmReg => match word_byte {
            WordByte::Byte => {
                sim_state.set_register_value(register, sim_mem.mem[address_calculation] as u16);
            }
            WordByte::Word => {
                let value: u16 = ((sim_mem.mem[address_calculation + 1] as u16) << 8)
                    + sim_mem.mem[address_calculation] as u16;
                sim_state.set_register_value(register, value);
            }
        },
    }
}

/// get the disassembly string and the number of bytes that were a part of the instruction for
/// any disassembly with the form [opcode:6 d:1 w:1] [mod:2 reg:3 rm:3] [disp-lo] [disp-hi]
fn mem_mem_disassembly(
    opcode: OpCode,
    machine_code: &Vec<u8>,
    index: usize,
    sim_state: &mut SimulationState,
    sim_mem: &mut SimMem,
) -> i8 {
    let first_byte = machine_code[index];

    let direction: Direction = ((first_byte & 0b00000010) >> 1).into();
    let word_byte: WordByte = (first_byte & 0b00000001).into();

    let second_byte = machine_code[index + 1];
    let mode: Mode = ((second_byte & 0b11000000) >> 6).into();
    let register_field = (second_byte & 0b00111000) >> 3;
    let register = get_register_enum(register_field, word_byte);

    let index_increment = match mode {
        Mode::MemNoDisplacement => {
            let rm_field = second_byte & 0b00000111;

            let (address_calculation, displacement_byte_count) =
                no_displacement_address(&sim_state, rm_field, &machine_code, index);

            simulate_mem_mem(
                sim_state,
                sim_mem,
                direction,
                word_byte,
                address_calculation,
                register,
            );

            2 + displacement_byte_count
        }
        Mode::Mem8BitDisplacement => {
            let rm_field = second_byte & 0b0000111;
            let displacement = machine_code[index + 2];
            let address_calculation =
                rm_field_to_displacement(&sim_state, rm_field, displacement as u16);

            simulate_mem_mem(
                sim_state,
                sim_mem,
                direction,
                word_byte,
                address_calculation,
                register,
            );

            3
        }
        Mode::Mem16BitDisplacement => {
            let rm_field = second_byte & 0b0000111;
            let displacement = concat_bytes(machine_code[index + 3], machine_code[index + 2]);
            let address_calculation = rm_field_to_displacement(&sim_state, rm_field, displacement);

            simulate_mem_mem(
                sim_state,
                sim_mem,
                direction,
                word_byte,
                address_calculation,
                register,
            );

            4
        }
        Mode::Register => {
            let second_register = get_rm_register_field(second_byte, word_byte);

            let (src_register, dest_register) = match direction {
                Direction::RegRm => (register, second_register),
                Direction::RmReg => (second_register, register),
            };

            // update simstate for register to register mem mov
            match opcode {
                OpCode::MovMem => {
                    let value = sim_state.get_register_value(src_register);
                    sim_state.set_register_value(dest_register, value);
                }
                OpCode::AddMemMem => {
                    let operand_value = sim_state.get_register_value(src_register);
                    let dest_value = sim_state.get_register_value(dest_register);
                    let value = operand_value + dest_value;
                    sim_state.set_register_value(dest_register, value);
                    sim_state.set_flags(value);
                }
                OpCode::SubMemMem => {
                    let operand_value = sim_state.get_register_value(src_register);
                    let dest_value = sim_state.get_register_value(dest_register);
                    let value = if dest_value > operand_value {
                        dest_value - operand_value
                    } else {
                        let diff = operand_value - dest_value;
                        let low_bytes = 0b1000000000000000 - diff;
                        0b1000000000000000 + low_bytes
                    };
                    sim_state.set_register_value(dest_register, value);
                    sim_state.set_flags(value);
                }
                OpCode::CmpMemMem => {
                    let operand_value = sim_state.get_register_value(src_register);
                    let dest_value = sim_state.get_register_value(dest_register);
                    let value = if dest_value >= operand_value {
                        dest_value - operand_value
                    } else {
                        let diff = operand_value - dest_value;
                        let low_bytes = 0b1000000000000000 - diff;
                        0b1000000000000000 + low_bytes
                    };

                    sim_state.set_flags(value);
                }
                _ => panic!("Unexpected opcode for mem to mem instruction"),
            };

            2
        }
    };

    index_increment as i8
}

/// common function for accumulator arithmetic
/// operation: the string for the operation. e.g. 'add', 'sub', 'cmp'
/// machine_code: the vector containing the machine code
/// index: the index for the first byte (containing the opcode)
fn accumulator_arithmetic(_operation: OpCode, machine_code: &Vec<u8>, index: usize) -> i8 {
    let first_byte = machine_code[index];

    let word_byte: WordByte = (first_byte & 0b00000001).into();
    let (_, _, index_increment) = match word_byte {
        WordByte::Byte => {
            let data = machine_code[index + 1] as u16;
            (data, "al", 2)
        }
        WordByte::Word => {
            let data = concat_bytes(machine_code[index + 2], machine_code[index + 1]);
            (data, "ax", 3)
        }
    };

    index_increment as i8
}

/// Get the immediate from the instruction and return both it and the number of bytes in the immediate value
/// machine_code: the vector containing all of the machine code
/// index: the index of the first byte in the instruction
/// low_byte_index: the index of the less significant byte. Will not be used if immediate is single byte
/// high_byte_index: the index of the more significant byte. Will not be used if immediate is single byte
/// word_byte: the word/byte field enum
/// sign_extension: the sign_extension field
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

/// Common function for jump opcodes
/// machine_code: the vector containing all of our machine code
/// index: the index of the first byte of the instruction
/// operation: the jump operation string
fn get_jump_offset(machine_code: &Vec<u8>, index: usize) -> i8 {
    // add 2 b/c ip register *should* be incremented before execution
    machine_code[index + 1] as i8 + 2
}

fn interpret_unsigned_as_signed(a: u16, bytes: usize) -> i16 {
    let (neg_part, pos_part) = if bytes == 1 {
        ((a & 0x80) as i16, (a & 0x7F) as i16)
    } else {
        ((a & 0x8000) as i16, (a & 0x7FFF) as i16)
    };

    -1 * neg_part + pos_part
}

pub fn simulate(machine_code: &Vec<u8>) -> String {
    let mut sim_log = "".to_owned();
    let mut sim_state = SimulationState {
        ..Default::default()
    };
    let mut sim_mem = SimMem::new(2 << 20);

    while (sim_state.ip as usize) < machine_code.len() {
        let previous_state = sim_state.clone();

        let index: usize = sim_state.ip as usize;

        let first_byte = machine_code[index];
        let opcode = get_opcode(first_byte);

        let ip_offset: i8 = match opcode {
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

                // 1 byte for the opcode + the number of bytes in the immediate
                let index_increment = immediate_bytes + 1;

                sim_state.set_register_value(register, immediate);

                index_increment
            }
            OpCode::ImmediateToMem => {
                let word_byte: WordByte = (first_byte & 0b00000001).into();

                let second_byte = machine_code[index + 1];

                let mode: Mode = ((second_byte & 0b11000000) >> 6).into();

                let (dest_arg, immediate, index_increment) = match mode {
                    Mode::MemNoDisplacement => {
                        let rm_field = second_byte & 0b00000111;

                        let (address_calculation, displacement_bytes) =
                            no_displacement_address_arithmetic(rm_field, machine_code, index);

                        // 2 bytes + displacment bytes is the low data byte
                        let low_byte_index = 2 + displacement_bytes;
                        // 2 bytes + displacment bytes + 1 low byte + 1 is the high data byte
                        let high_byte_index = 3 + displacement_bytes;

                        let (immediate, data_increment) = get_immediate(
                            machine_code,
                            index,
                            low_byte_index,
                            high_byte_index,
                            word_byte,
                            0,
                        );

                        (
                            address_calculation,
                            immediate,
                            2 + displacement_bytes + data_increment,
                        )
                    }
                    Mode::Mem8BitDisplacement => {
                        let rm_field = second_byte & 0b0000111;
                        let displacement = machine_code[index + 2];
                        let address_calculation =
                            rm_field_to_displacement(&sim_state, rm_field, displacement as u16);

                        let (immediate, data_increment) =
                            get_immediate(&machine_code, index, 3, 4, word_byte, 0);

                        (address_calculation, immediate, 3 + data_increment)
                    }
                    Mode::Mem16BitDisplacement => {
                        let rm_field = second_byte & 0b0000111;
                        let displacement =
                            concat_bytes(machine_code[index + 3], machine_code[index + 2]);
                        let address_calculation =
                            rm_field_to_displacement(&sim_state, rm_field, displacement);

                        let (immediate, data_increment) =
                            get_immediate(&machine_code, index, 4, 5, word_byte, 0);

                        (address_calculation, immediate, 4 + data_increment)
                    }
                    Mode::Register => {
                        panic!("Unexpected register mode when moving to memory")
                    }
                };

                // let instruction = format!("mov {}, {}\n", dest_arg, immediate);

                match word_byte {
                    WordByte::Byte => {
                        sim_mem.mem[dest_arg] = immediate as u8;
                    }
                    WordByte::Word => {
                        sim_mem.mem[dest_arg] = (immediate & 0xFF) as u8;
                        sim_mem.mem[dest_arg + 1] = ((immediate & 0xFF00) >> 8) as u8;
                    }
                }

                index_increment as i8
            }
            OpCode::MovMem => mem_mem_disassembly(
                OpCode::MovMem,
                &machine_code,
                index,
                &mut sim_state,
                &mut sim_mem,
            ),
            OpCode::AddMemMem => mem_mem_disassembly(
                OpCode::AddMemMem,
                &machine_code,
                index,
                &mut sim_state,
                &mut sim_mem,
            ),
            OpCode::SubMemMem => mem_mem_disassembly(
                OpCode::SubMemMem,
                &machine_code,
                index,
                &mut sim_state,
                &mut sim_mem,
            ),
            OpCode::CmpMemMem => mem_mem_disassembly(
                OpCode::CmpMemMem,
                &machine_code,
                index,
                &mut sim_state,
                &mut sim_mem,
            ),
            OpCode::ImmediateArithmetic => {
                let word_byte: WordByte = (first_byte & 0b00000001).into();

                let sign_extension = (first_byte & 0b00000010) >> 1;

                let second_byte = machine_code[index + 1];
                let mode: Mode = ((second_byte & 0b11000000) >> 6).into();
                let arithmetic_code: ArithmeticOpCode = ((second_byte & 0b00111000) >> 3).into();

                let index_increment = match mode {
                    Mode::MemNoDisplacement => {
                        let rm_field = second_byte & 0b00000111;

                        let (_, displacement_bytes) =
                            no_displacement_address_arithmetic(rm_field, &machine_code, index);

                        // 2 bytes + displacment bytes is the low data byte
                        let low_byte_index = 2 + displacement_bytes;
                        // 2 bytes + displacment bytes + 1 low byte + 1 is the high data byte
                        let high_byte_index = 3 + displacement_bytes;

                        let (_, data_increment) = get_immediate(
                            &machine_code,
                            index,
                            low_byte_index,
                            high_byte_index,
                            word_byte,
                            sign_extension,
                        );

                        2 + displacement_bytes + data_increment
                    }
                    Mode::Mem8BitDisplacement => {
                        let rm_field = second_byte & 0b0000111;
                        let displacement = machine_code[index + 2];
                        let _ = rm_field_to_displacement(&sim_state, rm_field, displacement as u16);

                        let (_, data_increment) =
                            get_immediate(&machine_code, index, 3, 4, word_byte, sign_extension);

                        3 + data_increment
                    }
                    Mode::Mem16BitDisplacement => {
                        let rm_field = second_byte & 0b0000111;
                        let displacement =
                            concat_bytes(machine_code[index + 3], machine_code[index + 2]);
                        let _ = rm_field_to_displacement(&sim_state, rm_field, displacement);

                        let (_, data_increment) =
                            get_immediate(&machine_code, index, 4, 5, word_byte, sign_extension);

                        4 + data_increment
                    }
                    Mode::Register => {
                        let register = get_rm_register_field(second_byte, word_byte);
                        let low_byte_index = 2;
                        let high_byte_index = 3;
                        let (immediate, immediate_bytes) = get_immediate(
                            &machine_code,
                            index,
                            low_byte_index,
                            high_byte_index,
                            word_byte,
                            sign_extension,
                        );

                        match arithmetic_code {
                            ArithmeticOpCode::Add => {
                                let value = if sign_extension == 0 {
                                    sim_state.get_register_value(register) + immediate
                                } else {
                                    let signed_result = interpret_unsigned_as_signed(
                                        sim_state.get_register_value(register),
                                        2,
                                    ) + interpret_unsigned_as_signed(
                                        immediate,
                                        immediate_bytes,
                                    );
                                    signed_result as u16
                                };
                                sim_state.set_register_value(register, value);
                                sim_state.set_flags(value);
                            }
                            ArithmeticOpCode::Sub => {
                                let value = if sign_extension == 0 {
                                    sim_state.get_register_value(register) - immediate
                                } else {
                                    let signed_result = interpret_unsigned_as_signed(
                                        sim_state.get_register_value(register),
                                        2,
                                    ) - interpret_unsigned_as_signed(
                                        immediate,
                                        immediate_bytes,
                                    );
                                    signed_result as u16
                                };
                                sim_state.set_register_value(register, value);
                                sim_state.set_flags(value);
                            }
                            ArithmeticOpCode::Cmp => {
                                let value = if sign_extension == 0 {
                                    sim_state.get_register_value(register) - immediate
                                } else {
                                    let signed_result = interpret_unsigned_as_signed(
                                        sim_state.get_register_value(register),
                                        2,
                                    ) - interpret_unsigned_as_signed(
                                        immediate,
                                        immediate_bytes,
                                    );
                                    signed_result as u16
                                };
                                sim_state.set_flags(value);
                            }
                        }

                        2 + immediate_bytes
                    }
                };

                index_increment as i8
            }
            OpCode::ImmediateToAccumulator => {
                accumulator_arithmetic(OpCode::ImmediateToAccumulator, &machine_code, index)
            }
            OpCode::ImmediateFromAccumulator => {
                accumulator_arithmetic(OpCode::ImmediateFromAccumulator, &machine_code, index)
            }
            OpCode::CmpImmediateToAccumulator => {
                accumulator_arithmetic(OpCode::CmpImmediateToAccumulator, &machine_code, index)
            }
            OpCode::JneJnz => {
                if !sim_state.zero_flag {
                    get_jump_offset(&machine_code, index)
                } else {
                    2
                }
            }
            OpCode::Je => unimplemented!(),
            OpCode::Jl => unimplemented!(),
            OpCode::Jle => unimplemented!(),
            OpCode::Jb => unimplemented!(),
            OpCode::Jbe => unimplemented!(),
            OpCode::Jp => unimplemented!(),
            OpCode::Jo => unimplemented!(),
            OpCode::Js => unimplemented!(),
            OpCode::Jnl => unimplemented!(),
            OpCode::Jg => unimplemented!(),
            OpCode::Jnb => unimplemented!(),
            OpCode::Ja => unimplemented!(),
            OpCode::Jnp => unimplemented!(),
            OpCode::Jno => unimplemented!(),
            OpCode::Jns => unimplemented!(),
            OpCode::Loop => unimplemented!(),
            OpCode::Loopz => unimplemented!(),
            OpCode::Loopnz => unimplemented!(),
            OpCode::Jcxz => unimplemented!(),
        };

        // remove newline from instruction
        let (mut instruction, _) = get_instruction(machine_code, index);
        instruction.truncate(instruction.len() - 1);

        sim_state.ip = ((sim_state.ip as i16) + (ip_offset as i16)) as u16;

        let state_diff = get_sim_state_diff(&previous_state, &sim_state);
        sim_log.push_str(&format!("{} ; {}", &instruction, state_diff));
    }

    sim_log.push_str(&format!("Final registers:\n"));
    sim_log.push_str(&format!("{}\n", sim_state.pretty_string()));

    sim_log
}
