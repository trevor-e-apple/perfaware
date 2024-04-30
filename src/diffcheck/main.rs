use std::{env, fs, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    // arg parsing: (currently assumes "cargo run", so that should be handled at some point)

    // first arg is just the file to disassemble
    let file_one_path = match args.get(1) {
        Some(file_one_path) => file_one_path,
        None => {
            println!("Missing file 1");
            exit(1);
        }
    };
    println!("File 1: {:?}", file_one_path);

    // second arg is just the file to output to
    let file_two_path = match args.get(2) {
        Some(file_two_path) => file_two_path,
        None => {
            println!("Missing file 2");
            exit(1);
        }
    };
    println!("File 2: {:?}", file_two_path);

    let f1_contents = match fs::read(file_one_path) {
        Ok(contents) => contents,
        Err(error) => {
            println!("Error {:?}", error);
            exit(1);
        }
    };
    let f2_contents = match fs::read(file_two_path) {
        Ok(contents) => contents,
        Err(error) => {
            println!("Error {:?}", error);
            exit(1);
        }
    };

    let all_match = {
        let mut all_match = true;
        for (f1_byte, f2_byte) in f1_contents.into_iter().zip(f2_contents.into_iter()) {
            if f1_byte != f2_byte {
                all_match = false;
            }
        }
        all_match
    };

    if all_match {
        println!("Perfect match");
    } else {
        println!("Diff found");
    }
}
