mod disassemble;

use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
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
    let directory = &args[1];

    let dir_path = Path::new(directory);
    for dir_entry in fs::read_dir(directory).unwrap() {
        let dir_entry = dir_entry.unwrap();
        let original_asm_path = dir_entry.path().into_os_string().into_string().unwrap();
        let file_name_no_extension = {
            let with_extension = dir_entry.file_name().into_string().unwrap();
            Path::file_stem(Path::new(&with_extension))
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
        };

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
    }

    // disassemble all files that were converted using nasm

    // assemble all disassembly

    // perform a diff check
}
