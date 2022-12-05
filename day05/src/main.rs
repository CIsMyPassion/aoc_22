use std::{path::Path, fs};

fn main() {
    part_one();
}

fn read_input() -> (Ship, Vec<Move>) {
    let path = Path::new("res/input");
    let contents = fs::read_to_string(path).expect("input file needed");
}

fn part_one() {
    let (ship, moves) = read_input();

    moves.iter().for_each(|some_move| ship.apply_move(&some_move));
}

struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

struct Ship {
    stacks: Vec<Vec<char>>
}

impl Ship {
    pub fn apply_move(&mut self, some_move: &Move) {
        for i in 0..some_move.amount {
            self.single_move(some_move.from, some_move.to);
        }
    }

    fn single_move(&mut self, from: usize, to: usize) {
        let from_letter = self.stacks[from].pop().unwrap();
        self.stacks[to].push(from_letter);
    }

    fn get_tops(&self) -> String {
        self.stacks.iter().map(|stack| stack[stack.len() - 1]).collect()
    }
}
