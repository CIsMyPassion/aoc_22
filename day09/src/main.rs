use std::{path::Path, fs, collections::HashSet, process::exit};

fn main() {
    part_one();
}

fn read_input() -> String {
    let path = Path::new("res/input");
    fs::read_to_string(path).expect("input file needed")
}

fn input_to_moves(input: &str) -> Vec<Move> {
    let lines = input.split("\n").filter(|line| !line.is_empty());
    lines.map(|line| line_to_move(line)).collect()
}

fn line_to_move(line: &str) -> Move {
    let direction = match line.chars().nth(0).unwrap() {
        'U' => Direction::UP,
        'D' => Direction::DOWN,
        'L' => Direction::LEFT,
        'R' => Direction::RIGHT,
        _ => exit(1),
    };

    let steps = line.split(" ").nth(1).unwrap().parse().unwrap();

    Move::new(direction, steps)
}

fn part_one() {
    let input = read_input();
    let mut rope = Rope::new();
    let moves = input_to_moves(&input);

    moves.iter().for_each(|some_move| { rope.apply_move(&some_move); });

    println!("Visited: {}", rope.visited_count());
}

struct Move {
    direction: Direction,
    steps: u64,
}

impl Move {
    pub fn new(direction: Direction, steps: u64) -> Self {
        Self { direction, steps }
    }
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Rope {
    head: (i64, i64),
    tail: (i64, i64),
    visited: HashSet<(i64, i64)>,
}

impl Rope {
    pub fn new() -> Self {
        Self { head: (0, 0), tail: (0, 0), visited: HashSet::from([(0, 0)]) }
    }

    pub fn apply_move(&mut self, some_move: &Move) {
        for _ in 0..some_move.steps {
            match some_move.direction {
                Direction::UP => self.head.1 += 1,
                Direction::DOWN => self.head.1 -= 1,
                Direction::LEFT => self.head.0 -= 1,
                Direction::RIGHT => self.head.0 += 1,
            }
            self.pull_tail();
        }
    }

    fn pull_tail(&mut self) {
        let diff_x = self.head.0 - self.tail.0;
        let diff_y = self.head.1 - self.tail.1;

        if diff_x.abs() > 1 && diff_y.abs() > 0
            || diff_x.abs() > 0 && diff_y.abs() > 1 {

            self.tail.0 += diff_x.signum();
            self.tail.1 += diff_y.signum();
        } else {
            if diff_x.abs() > 1 {
                self.tail.0 += diff_x.signum();
            } else if diff_y.abs() > 1 {
                self.tail.1 += diff_y.signum();
            }
        }

        self.visited.insert(self.tail);
    }

    fn visited_count(&self) -> usize {
        self.visited.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

    #[test]
    fn part_one_test() {
        let mut rope = Rope::new();
        let moves = input_to_moves(INPUT_TEXT);

        moves.iter().for_each(|some_move| { rope.apply_move(&some_move); });

        assert_eq!(rope.visited_count(), 13);
    }
}
