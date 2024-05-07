pub mod disassemble;

mod diffcheck;
mod test;

use diffcheck::diffcheck;
use disassemble::disassemble_file;
use std::{env, process::exit};
use test::test_assembly_listings;

fn main() {
    println!("My 8086 decoder!");

    let args: Vec<String> = env::args().collect();

    // arg parsing: (currently assumes "cargo run", so that should be handled at some point)

    let command = match args.get(1) {
        Some(command) => command,
        None => {
            println!("Missing command");
            exit(1);
        }
    };

    if command == "disassemble" {
        // first arg is just the file to disassemble
        let file_path = match args.get(2) {
            Some(file_path) => file_path,
            None => exit(1),
        };
        println!("Reading file: {:?}", file_path);

        // second arg is just the file to output to
        let output_file_path = match args.get(3) {
            Some(path) => path,
            None => {
                println!("Missing output file path");
                exit(1);
            }
        };
        println!("Outputting to file {:?}", output_file_path);

        disassemble_file(file_path, output_file_path);
    } else if command == "diffcheck" {
        // arg parsing: (currently assumes "cargo run", so that should be handled at some point)

        // first arg is just the file to disassemble
        let file_one_path = match args.get(2) {
            Some(file_one_path) => file_one_path,
            None => {
                println!("Missing file 1");
                exit(1);
            }
        };
        println!("File 1: {:?}", file_one_path);

        // second arg is just the file to output to
        let file_two_path = match args.get(3) {
            Some(file_two_path) => file_two_path,
            None => {
                println!("Missing file 2");
                exit(1);
            }
        };
        println!("File 2: {:?}", file_two_path);
        diffcheck(file_one_path, file_two_path);
    } else if command == "test" {
        let args: Vec<String> = env::args().collect();

        let asm_path = match args.get(2) {
            Some(path) => {
                println!("Using path {}", path);
                path
            }
            None => {
                println!("Unable to find path to asm listings.");
                exit(1);
            }
        };

        test_assembly_listings(asm_path);
    }
}
