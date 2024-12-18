mod byte_operations;
mod common_assembly;
mod disassemble;
mod simulate;
mod simulator_state;

use std::{
    fs::{self, remove_file, File},
    io::Write,
    iter::zip,
    path::{Path, PathBuf},
    process::Command,
};

use argparse;
use argparse::ArgumentParser;
use disassemble::disassemble;
use simulate::simulate;

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
    let mut target = "".to_owned();
    let mut should_reassemble = false;
    let mut should_simulate = false;

    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Disassemble and/or simulate 8086 instructions");
        ap.refer(&mut target).add_argument(
            "target",
            argparse::Store,
            "The directory / file to target",
        );
        ap.refer(&mut should_reassemble)
            .add_option(&["--reassemble"], argparse::StoreTrue, "Whether or not to reassemble and perform a diff between original assembly and diassembly-based assembly");
        ap.refer(&mut should_simulate).add_option(
            &["--simulate"],
            argparse::StoreTrue,
            "Whether or not to run a program simulation",
        );
        ap.parse_args_or_exit();
    }

    let path = Path::new(&target);

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

        // read assembler output
        let contents = match fs::read(&original_outpath) {
            Ok(contents) => contents,
            Err(error) => {
                eprintln!("Failed to read {}", &original_outpath);
                eprintln!("{}", error);
                return;
            }
        };

        // perform a diff check
        if should_reassemble {
            // disassemble with our disassembler
            let (gen_asm_path, gen_outpath) = {
                let disassembly = disassemble(&contents);

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

        if should_simulate {
            let (_, simulation_log) = simulate(&contents);
            println!("Simulation results:");
            print!("{}", simulation_log);
        }
    }
}
