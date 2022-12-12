use std::{path::Path, fs, collections::HashSet, process::exit};

fn main() {
    part_one();
    part_two();
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
    let mut rope = ShortRope::new();
    let moves = input_to_moves(&input);

    moves.iter().for_each(|some_move| { rope.apply_move(&some_move); });

    println!("Visited: {}", rope.visited_count());
}

fn part_two() {
    let input = read_input();
    let mut rope = LongRope::new(9);
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

trait Rope {
    fn apply_move(&mut self, some_move: &Move);
    fn visited_count(&self) -> usize;
}

struct ShortRope {
    head: (i64, i64),
    tail: (i64, i64),
    visited: HashSet<(i64, i64)>,
}

impl ShortRope {
    pub fn new() -> Self {
        Self { head: (0, 0), tail: (0, 0), visited: HashSet::from([(0, 0)]) }
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
}

impl Rope for ShortRope {
    fn apply_move(&mut self, some_move: &Move) {
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

    fn visited_count(&self) -> usize {
        self.visited.len()
    }
}

struct LongRope {
    head: (i64, i64),
    tail: Vec<(i64, i64)>,
    visited: HashSet<(i64, i64)>,
}

impl LongRope {
    pub fn new(length: usize) -> Self {
        Self { head: (0, 0), tail: vec![(0, 0); length], visited: HashSet::from([(0, 0)]) }
    }

    fn pull_tail(&mut self) {
        for i in 0..self.tail.len() {
            self.pull_element(i);
        }

        self.visited.insert(*self.tail.last().unwrap());
    }

    fn pull_element(&mut self, n: usize) {
        let previous_element = if n == 0 {
            self.head
        } else {
            self.tail[n - 1]
        };
        let mut current_element = &mut self.tail[n];

        let diff_x = previous_element.0 - current_element.0;
        let diff_y = previous_element.1 - current_element.1;

        if diff_x.abs() > 1 && diff_y.abs() > 0
            || diff_x.abs() > 0 && diff_y.abs() > 1 {

            current_element.0 += diff_x.signum();
            current_element.1 += diff_y.signum();
        } else {
            if diff_x.abs() > 1 {
                current_element.0 += diff_x.signum();
            } else if diff_y.abs() > 1 {
                current_element.1 += diff_y.signum();
            }
        }
    }
}

impl Rope for LongRope {
    fn apply_move(&mut self, some_move: &Move) {
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

    fn visited_count(&self) -> usize {
        self.visited.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_ONE_INPUT: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

    const PART_TWO_INPUT: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;

    #[test]
    fn part_one_test() {
        let mut rope = ShortRope::new();
        let moves = input_to_moves(PART_ONE_INPUT);

        moves.iter().for_each(|some_move| { rope.apply_move(&some_move); });

        assert_eq!(rope.visited_count(), 13);
    }

    #[test]
    fn part_two_test() {
        let mut rope = LongRope::new(9);
        let moves = input_to_moves(PART_TWO_INPUT);

        moves.iter().for_each(|some_move| { rope.apply_move(&some_move); });

        assert_eq!(rope.visited_count(), 36);
    }
}
