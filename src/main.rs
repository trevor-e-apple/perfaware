use std::{env, fs, path::Path, process::Command};

fn run_nasm(path: &str, outpath: &str) {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg("nasm")
            .arg(path)
            .arg("-o")
            .arg(outpath)
            .spawn()
            .expect("failed to execute process");
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("nasm")
            .arg(path)
            .spawn()
            .expect("failed to execute process");
    }
}

fn main() {
    // get args
    let args: Vec<String> = env::args().collect();
    let directory = &args[1];

    let dir_path = Path::new(directory);
    for dir_entry in fs::read_dir(directory).unwrap() {
        let dir_entry = dir_entry.unwrap();
        let original_asm_path = dir_entry.path().into_os_string().into_string().unwrap();
        let original_outpath = {
            let with_extension = dir_entry.file_name().into_string().unwrap();
            let file_name = format!(
                "{}.bin",
                Path::file_stem(Path::new(&with_extension))
                    .unwrap()
                    .to_str()
                    .unwrap()
            );
            Path::join(dir_path, Path::new(&file_name))
                .into_os_string()
                .into_string()
                .unwrap()
        };

        println!("{} {}", &original_asm_path, &original_outpath);

        // assemble with nasm
        run_nasm(&original_asm_path, &original_outpath);

        // disassemble with our disassembler

        // assemble with nasm
        // run_nasm(path, outpath);

        // perform a diff check
    }

    // disassemble all files that were converted using nasm

    // assemble all disassembly

    // perform a diff check
}
