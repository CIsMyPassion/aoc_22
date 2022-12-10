use std::{path::Path, fs, collections::HashSet};

fn main() {
    part_one();
}

fn read_input() -> String {
    let path = Path::new("res/input");
    let contents = fs::read_to_string(path).expect("input file needed");

    contents.replace("\n", "")
}

fn part_one() {
    let sequence = read_input();
    let marker_index = find_marker_index(&sequence);

    println!("Marker index {marker_index}");
}

fn find_marker_index(sequence: &str) -> usize {
    for i in 4..sequence.len() {
        let marker = &sequence[i-4..i];
        let mut char_set = HashSet::new();
        marker.chars().for_each(|c| { char_set.insert(c); });

        if char_set.len() == 4 {
            return i;
        }
    }

    return sequence.len();
}
