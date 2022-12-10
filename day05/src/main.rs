use std::{path::Path, fs};

fn main() {
    part_one();
}

fn read_input() -> (Ship, Vec<Move>) {
    let path = Path::new("res/input");
    let contents = fs::read_to_string(path).expect("input file needed");

    let (ship_input, moves_input) = contents.split_at(contents.find("\n\n").unwrap());

    let ship_lines: Vec<&str> = ship_input.split("\n").collect();
    let ship_lines_trimmed = &ship_lines[0..ship_lines.len() - 1];
    let row_count = (ship_lines.last().unwrap().len() + 1) / 4;
    let stacks: Vec<Vec<char>> = (0..=row_count).into_iter().map(|row| extract_row(row, ship_lines_trimmed)).collect();

    let moves = moves_input.split("\n").filter(|line| !line.is_empty()).map(|line| line_to_move(line)).collect();

    (Ship::new(stacks), moves)
}

fn extract_row(row: usize, ship_lines_trimmed: &[&str]) -> Vec<char> {
    ship_lines_trimmed.iter().map(|line| char_for_row(row, line)).filter(|c| *c != ' ').rev().collect()
}

fn char_for_row(row: usize, line: &str) -> char {
    line.chars().nth(row * 4 + 1).unwrap_or(' ')
}

fn line_to_move(line: &str) -> Move {
    let parts: Vec<&str> = line.split(" ").collect();
    Move::new(parts[3].parse().unwrap(), parts[5].parse().unwrap(), parts[1].parse().unwrap())
}

fn part_one() {
    let (mut ship, moves) = read_input();
    moves.iter().for_each(|some_move| ship.apply_move(&some_move));

    println!("Tops: {}", ship.get_tops());
}

struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

impl Move {
    pub fn new(from: usize, to: usize, amount: usize) -> Self {
        Self { from, to, amount }
    }
}

struct Ship {
    stacks: Vec<Vec<char>>
}

impl Ship {
    pub fn new(stacks: Vec<Vec<char>>) -> Self {
        Self { stacks }
    }

    pub fn apply_move(&mut self, some_move: &Move) {
        for _ in 0..some_move.amount {
            self.single_move(some_move.from, some_move.to);
        }
    }

    fn single_move(&mut self, from: usize, to: usize) {
        let from_letter = self.stacks[from - 1].pop().unwrap();
        self.stacks[to - 1].push(from_letter);
    }

    fn get_tops(&self) -> String {
        self.stacks.iter().map(|stack| stack[stack.len() - 1]).collect()
    }
}
