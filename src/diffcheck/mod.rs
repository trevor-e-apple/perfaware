use std::{fs, process::exit};

pub fn diffcheck(file_one_path: &String, file_two_path: &String) {
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
