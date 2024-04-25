use std::{collections::HashMap, env, fmt::Write, fs, hash::Hash, process::exit};

enum Opcode {
    Mov = 0b100010,
}

enum ModFieldCodes {
    MemoryNoDisplacement = 0b00,
    Memory8bitDisplacement = 0b01,
    Memory16bitDisplacement = 0b10,
    RegToReg = 0b11,
}

fn main() {
    println!("My 8086 decoder!");

    /*
    Construct our map for the reg field, which takes a tuple of (reg_field, w_bit) and
    returns the name of the register. This same map also works for the rm field when
    performing moves between registers.
    */
    let register_map = {
        let mut register_map: HashMap<(u8, u8), String> = HashMap::new();
        register_map.insert((0b000, 0b0), "al".to_owned());
        register_map.insert((0b000, 0b1), "ax".to_owned());
        register_map.insert((0b001, 0b0), "cl".to_owned());
        register_map.insert((0b001, 0b1), "cx".to_owned());
        register_map.insert((0b010, 0b0), "dl".to_owned());
        register_map.insert((0b010, 0b1), "dx".to_owned());
        register_map.insert((0b011, 0b0), "bl".to_owned());
        register_map.insert((0b011, 0b1), "bx".to_owned());
        register_map.insert((0b100, 0b0), "ah".to_owned());
        register_map.insert((0b100, 0b1), "sp".to_owned());
        register_map.insert((0b101, 0b0), "ch".to_owned());
        register_map.insert((0b101, 0b1), "bp".to_owned());
        register_map.insert((0b110, 0b0), "dh".to_owned());
        register_map.insert((0b110, 0b1), "si".to_owned());
        register_map.insert((0b111, 0b0), "bh".to_owned());
        register_map.insert((0b111, 0b1), "di".to_owned());

        register_map
    };

    /* Construct the map from op codes to asm name */
    let opcode_map = {
        let mut opcode_map: HashMap<u8, String> = HashMap::new();
        opcode_map.insert(0b100010, "mov".to_owned());

        opcode_map
    };

    let args: Vec<String> = env::args().collect();

    // first arg is just the file to disassemble
    let file_path = match args.get(0) {
        Some(file_path) => file_path,
        None => exit(1),
    };

    // first arg is just the file to output to
    let output_file_path = match args.get(1) {
        Some(path) => path,
        None => {
            println!("Missing output file path");
            exit(1);
        }
    };

    let contents = match fs::read(file_path) {
        Ok(contents) => contents,
        Err(error) => {
            println!("Error {:?}", error);
            exit(1);
        }
    };

    let mut output_buffer = String::with_capacity(16 * contents.len());
    output_buffer
        .write_str("bits 16\n")
        .expect("Unable to write to output buffer");

    // 8086 instructions come in pairs
    for pair_index in 0..(contents.len() / 2) {
        let byte_one_index = 2 * pair_index;
        let byte_two_index = byte_one_index + 1;

        let byte_one = contents
            .get(byte_one_index)
            .expect(&format!("Unable to get byte at index {:?}", byte_one_index));
        let byte_two = contents
            .get(byte_two_index)
            .expect(&format!("Unable to get byte at index {:?}", byte_two_index));

        // First six bits of byte_one is the instruction
        let opcode = (byte_one & 0b11111100) >> 2;
        let opcode_name = opcode_map
            .get(&opcode)
            .expect(&format!("Unable to get opcode for byte {:?}", opcode));

        // mov
        if opcode == (Opcode::Mov as u8) {
            let d_bit = (byte_one & 0b00000010) >> 1;
            let w_bit = byte_one & 0b00000001;
            let mod_field = (byte_two & 0b11000000) >> 6;
            let reg_field = (byte_two & 0b00111000) >> 3;
            let rm_field = (byte_two & 0b00000111) >> 3;

            let (from_field, to_field) = if d_bit == 0b0 {
                (reg_field, rm_field)
            } else {
                (rm_field, reg_field)
            };

            let from = register_map
                .get(&(from_field, w_bit))
                .expect("Unable to get register name");
            let to = register_map
                .get(&(to_field, w_bit))
                .expect("Unable to get register name");

            output_buffer
                .write_str(&format!("{:?} {:?}, {:?}\n", opcode_name, to, from))
                .expect("Unable to write to output buffer");
        } else {
            unimplemented!("Opcode not recognized");
        }
    }

    fs::write(output_file_path, output_buffer).expect("Unable to write output file");
}
