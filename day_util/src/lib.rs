use std::{path::PathBuf, fs};

const INPUT_FILE_STRING: &str = "res/input";

pub fn input_file_path() -> PathBuf {
    PathBuf::from(INPUT_FILE_STRING)
}

pub fn read_input_file(path: &PathBuf) -> String {
    fs::read_to_string(path).expect("Input file expected at \"res/input\"")
}

pub fn read_input() -> String {
    read_input_file(&input_file_path())
}
