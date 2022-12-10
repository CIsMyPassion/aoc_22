use std::{path::Path, fs, collections::HashSet};

fn main() {
    part_one();
    part_two();
}

fn read_input() -> String {
    let path = Path::new("res/input");
    let contents = fs::read_to_string(path).expect("input file needed");

    contents.replace("\n", "")
}

fn part_one() {
    let sequence = read_input();
    let packet_index = find_packet_marker_index(&sequence);

    println!("Packet index: {packet_index}");
}

fn part_two() {
    let sequence = read_input();
    let message_index = find_message_marker_index(&sequence);

    println!("Message index: {message_index}");
}

fn find_packet_marker_index(sequence: &str) -> usize {
    find_marker_index(sequence, 4)
}

fn find_message_marker_index(sequence: &str) -> usize {
    find_marker_index(sequence, 14)
}

fn find_marker_index(sequence: &str, length: usize) -> usize {
    for i in length..sequence.len() {
        let marker = &sequence[i-length..i];
        let mut char_set = HashSet::new();
        marker.chars().for_each(|c| { char_set.insert(c); });

        if char_set.len() == length {
            return i;
        }
    }

    return sequence.len();
}
