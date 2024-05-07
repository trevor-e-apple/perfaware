use crate::disassemble::disassemble_file;
use std::{
    fs,
    path::Path,
    process::{exit, Command},
};

pub fn test_assembly_listings(asm_path: &str) {
    // Get all files in the asm directory
    let files = match fs::read_dir(asm_path) {
        Ok(files) => files,
        Err(err) => {
            println!("Unable to read files at asm directory. Error: {:?}", err);
            exit(1);
        }
    };

    for file_result in files {
        let file = match file_result {
            Ok(file) => file,
            Err(err) => {
                println!("Error with file {:?}", err);
                exit(1);
            }
        };

        let file_name = {
            let file_name = file.file_name();
            match file_name.to_str() {
                Some(file_name) => file_name.to_owned(),
                None => {
                    println!("Error converting {:?} name to string", file_name);
                    exit(1);
                }
            }
        };

        if file_name.starts_with("listing_") {
            println!("Testing {}", file_name);

            // assemble with NASM
            let file_in_path = Path::new(asm_path).join(file_name);
            let os_file_name = &file.file_name();
            let build_path = Path::new(asm_path).join("build");
            let file_stem = Path::new(os_file_name)
                .file_stem()
                .expect("Error getting file stem");
            let file_out_path = build_path.join(file_stem);
            Command::new("nasm")
                .arg(&file_in_path)
                .arg("-o")
                .arg(&file_out_path)
                .output()
                .expect("Failed to execute NASM process");

            // disassemble
            let disassembly_path = Path::new(asm_path);
            disassemble_file(
                file_out_path.to_str().expect("Unable to convert to string"),
                disassembly_path
                    .to_str()
                    .expect("Unable to convert to string"),
            );

            // assemble disassembly with NASM

            // perform diff check and report result
        }
    }
}
