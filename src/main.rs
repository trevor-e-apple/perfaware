mod byte_operations;
mod common_assembly;
mod disassemble;
mod simulator_state;

use std::{
    env,
    fs::{self, remove_file, File},
    io::Write,
    iter::zip,
    path::{Path, PathBuf},
    process::Command,
};

use disassemble::disassemble;

fn run_nasm(path: &str, outpath: &str) {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg("nasm")
            .arg(path)
            .arg("-o")
            .arg(outpath)
            .output()
            .expect("failed to execute process");
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(format!("nasm {} -o {}", path, outpath))
            .output()
            .expect("failed to execute process");
    }
}

fn main() {
    // get args
    let args: Vec<String> = env::args().collect();
    let target = &args[1];
    let path = Path::new(target);

    let (dir_path, file_paths) = match fs::read_dir(path) {
        Ok(dir_iter) => {
            let mut file_paths: Vec<PathBuf> = Vec::new();
            for dir_entry in dir_iter {
                match dir_entry {
                    Ok(file_path) => {
                        file_paths.push(file_path.path().to_path_buf());
                    }
                    Err(_) => {}
                }
            }
            (path, file_paths)
        }
        Err(_) => {
            if path.is_file() {
                let parent_path = path.parent().unwrap();
                (parent_path, vec![path.to_path_buf()])
            } else {
                std::process::exit(1)
            }
        }
    };

    for file_path in file_paths {
        if file_path.extension().unwrap().to_str().unwrap() != "asm" {
            continue;
        }

        let original_asm_path = file_path.clone().into_os_string().into_string().unwrap();
        println!("Testing {}", &original_asm_path);

        let file_name_no_extension = file_path.file_stem().unwrap().to_str().unwrap().to_owned();

        let original_outpath = {
            let file_name = format!("{}.bin", &file_name_no_extension);
            Path::join(dir_path, Path::new(&file_name))
                .into_os_string()
                .into_string()
                .unwrap()
        };

        // assemble with nasm
        run_nasm(&original_asm_path, &original_outpath);

        // disassemble with our disassembler
        let (gen_asm_path, gen_outpath) = {
            // read assembler output
            let contents = match fs::read(&original_outpath) {
                Ok(contents) => contents,
                Err(error) => {
                    eprintln!("Failed to read {}", &original_outpath);
                    eprintln!("{}", error);
                    return;
                }
            };
            let disassembly = disassemble(contents);

            let gen_asm_name = format!("{}_test_gen.asm", file_name_no_extension);
            let gen_asm_path = Path::join(dir_path, gen_asm_name);

            // write to file
            let mut file = File::create(&gen_asm_path).unwrap();
            file.write(&disassembly.as_bytes())
                .expect("Failed to write disassembly to file");

            // gen outpath
            let out_file_name = format!("{}_test_gen.bin", &file_name_no_extension);
            let gen_outpath = Path::join(dir_path, Path::new(&out_file_name))
                .into_os_string()
                .into_string()
                .unwrap();

            (
                gen_asm_path.into_os_string().into_string().unwrap(),
                gen_outpath,
            )
        };

        // assemble with nasm
        run_nasm(&gen_asm_path, &gen_outpath);

        // perform a diff check
        let test_passed: bool = {
            let original_data = fs::read(&original_outpath).expect("Unexpected read error");
            let gen_data = fs::read(&gen_outpath).expect("Unexpected read error");

            let mut test_passed = true;
            for (original_byte, gen_byte) in zip(&original_data, &gen_data) {
                if *original_byte != *gen_byte {
                    println!("{} dissassembly failed", original_asm_path);
                    test_passed = false;
                    break;
                }
            }

            test_passed
        };

        // delete the generated files
        if test_passed {
            remove_file(&gen_asm_path).expect("Unable to remove gen asm");
            remove_file(&gen_outpath).expect("Unable to remove gen binary");
            println!("Test passed");
        }
    }
}
