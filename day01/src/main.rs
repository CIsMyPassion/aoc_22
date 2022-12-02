use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::cmp::Ordering;

fn main() {
    part_one();
    part_two();
}

fn read_input() -> Vec<Elf> {
    let path = Path::new("res/input");
    let contents = fs::read_to_string(path).expect("input file needed");
    let block = contents.split("\n\n");

    let block_lists: Vec<Vec<u32>> = block.into_iter().map(|block| block_to_numbers(block)).collect();
    block_lists.iter().map(|block| Elf::new(block.to_vec())).collect()
}

fn block_to_numbers(block: &str) -> Vec<u32> {
    let lines = block.split("\n");

    lines.into_iter().filter(|line| !line.is_empty()).map(|number| FromStr::from_str(number).unwrap()).collect()
}

fn part_one() {
    let elfs = read_input();

    let max_elf = elfs.iter().max().unwrap();
    println!("Elf with max calories: {}", max_elf.total_calories());
}

fn part_two() {
    let mut elfs = read_input();

    elfs.sort_by(|a, b| b.cmp(a));
    let total_top_three = elfs[0].total_calories() + elfs[1].total_calories() + elfs[2].total_calories();

    println!("The top three elfs calories total: {}", total_top_three);
}

#[derive(Eq)]
struct Elf {
    items: Vec<u32>
}

impl Elf {
    pub fn new(items: Vec<u32>) -> Self {
        Self { items }
    }

    pub fn total_calories(&self) -> u32 {
        self.items.iter().sum()
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_calories().cmp(&other.total_calories())
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.total_calories() == other.total_calories()
    }
}
