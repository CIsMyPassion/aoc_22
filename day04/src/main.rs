use std::{path::Path, fs, str::FromStr, ops::RangeInclusive};

fn main() {
    part_one();
}

fn read_input() -> Vec<Team> {
    let path = Path::new("res/input");
    let contents = fs::read_to_string(path).expect("input file needed");
    let lines = contents.split("\n").filter(|line| !line.is_empty());

    lines.map(|line| line_to_team(line)).collect()
}

fn line_to_team(line: &str) -> Team {
    let parts: Vec<&str> = line.split(",").collect();
    assert_eq!(parts.len(), 2);

    Team::new(piece_to_tasks(parts[0]), piece_to_tasks(parts[1]))
}

fn piece_to_tasks(piece: &str) -> Tasks {
    let nums: Vec<&str> = piece.split("-").collect();
    assert_eq!(nums.len(), 2);

    Tasks::new(FromStr::from_str(nums[0]).unwrap(), FromStr::from_str(nums[1]).unwrap())
}

fn part_one() {
    let teams = read_input();

    let mut total_overlaps = 0;
    teams.iter().for_each(|team| if team.overlaps() { total_overlaps += 1; });

    println!("Total overlaps: {}", total_overlaps);
}

struct Tasks {
    range: RangeInclusive<u32>,
}

impl Tasks {
    pub fn new(start: u32, end: u32) -> Self {
        Self { range: start..=end }
    }
}

struct Team (Tasks, Tasks);

impl Team {
    pub fn new(first: Tasks, second: Tasks) -> Self {
        Self(first, second)
    }

    pub fn overlaps(&self) -> bool {
        if self.0.range.contains(&self.1.range.start()) && self.0.range.contains(&self.1.range.end()) {
            true
        } else if self.1.range.contains(&self.0.range.start()) && self.1.range.contains(&self.0.range.end()) {
            true
        } else {
            false
        }
    }
}
