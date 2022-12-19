use std::{path::PathBuf, fs, process::exit};

const INPUT_FILE_STRING: &str = "res/input";

fn input_file_path() -> PathBuf {
    PathBuf::from(INPUT_FILE_STRING)
}

fn safe_input_file_path(day: &str) -> PathBuf {
    let full_path = day.to_owned() + "/" + INPUT_FILE_STRING;
    PathBuf::from(full_path)
}

fn read_input_file(path: &PathBuf) -> String {
    fs::read_to_string(path).expect("Input file expected at \"res/input\"")
}

pub fn read_input() -> String {
    read_input_file(&input_file_path())
}

pub fn read_input_safe(day: &str) -> String {
    match fs::read_to_string(input_file_path()) {
        Err(_) => {
            match fs::read_to_string(safe_input_file_path(day)) {
                Err(_) => exit(1),
                Ok(content) => content,
            }
        },
        Ok (content) => content,
    }
}