use std::{path::Path, fs, process::exit};

fn main() {
    part_one();
    part_two();
}

fn read_input_one() -> Vec<Game> {
    let path = Path::new("res/input");
    let contents = fs::read_to_string(path).expect("input file needed");
    let lines = contents.split("\n");

    lines.into_iter().filter(|line| !line.is_empty()).map(|line| line_to_game(line)).collect()
}

fn read_input_two() -> Vec<(Shape, Outcome)> {
    let path = Path::new("res/input");
    let contents = fs::read_to_string(path).expect("input file needed");
    let lines = contents.split("\n");

    lines.into_iter().filter(|line| !line.is_empty()).map(|line| line_to_shape_outcome(line)).collect()
}

fn line_to_game(line: &str) -> Game {
    let letters: Vec<&str> = line.split(" ").collect();

    assert_eq!(letters.len(), 2);
    assert_eq!(letters[0].len(), 1);
    assert_eq!(letters[1].len(), 1);

    let opponent_move;
    let own_move;

    match letters[0].chars().nth(0).unwrap() {
        'A' => {
            opponent_move = Shape::Rock;
        },
        'B' => {
            opponent_move = Shape::Paper;
        },
        'C' => {
            opponent_move = Shape::Scissors;
        },
        _ => {
            exit(1);
        }
    }

    match letters[1].chars().nth(0).unwrap() {
        'X' => {
            own_move = Shape::Rock;
        },
        'Y' => {
            own_move = Shape::Paper;
        },
        'Z' => {
            own_move = Shape::Scissors;
        },
        _ => {
            exit(1);
        }
    }

    Game::new(&opponent_move, &own_move)
}

fn line_to_shape_outcome(line: &str) -> (Shape, Outcome) {
    let letters: Vec<&str> = line.split(" ").collect();

    assert_eq!(letters.len(), 2);
    assert_eq!(letters[0].len(), 1);
    assert_eq!(letters[1].len(), 1);

    let opponent_move;
    let outcome;

    match letters[0].chars().nth(0).unwrap() {
        'A' => {
            opponent_move = Shape::Rock;
        },
        'B' => {
            opponent_move = Shape::Paper;
        },
        'C' => {
            opponent_move = Shape::Scissors;
        },
        _ => {
            exit(1);
        }
    }

    match letters[1].chars().nth(0).unwrap() {
        'X' => {
            outcome = Outcome::Loose;
        },
        'Y' => {
            outcome = Outcome::Draw;
        },
        'Z' => {
            outcome = Outcome::Win;
        },
        _ => {
            exit(1);
        }
    }

    (opponent_move, outcome)
}

fn part_one() {
    let games = read_input_one();
    let mut points_count = 0;
    games.iter().for_each(|game| { points_count += game.get_result(); });

    println!("Total points: {}", points_count);
}

fn part_two() {
    let games = read_input_two();
    let mut points_count = 0;
    games.iter().for_each(|(opponent_shape, outcome)| { points_count += calculate_game(opponent_shape, outcome).get_result(); });

    println!("Total points: {}", points_count);
}

fn calculate_game(opponent_move: &Shape, outcome: &Outcome) -> Game {
    match outcome {
        Outcome::Draw => Game::new(opponent_move, opponent_move),
        Outcome::Win => Game::new(opponent_move, &winning_shape(opponent_move)),
        Outcome::Loose => Game::new(opponent_move, &loosing_shape(opponent_move))
    }
}

fn winning_shape(opponent_move: &Shape) -> Shape {
    match opponent_move {
        Shape::Rock => Shape::Paper,
        Shape::Paper => Shape::Scissors,
        Shape::Scissors => Shape::Rock
    }
}

fn loosing_shape(opponent_move: &Shape) -> Shape {
    match opponent_move {
        Shape::Rock => Shape::Scissors,
        Shape::Paper => Shape::Rock,
        Shape::Scissors => Shape::Paper
    }
}

struct Game {
    opponent_move: Shape,
    own_move: Shape,
}

impl Game {
    pub fn new(opponent_move: &Shape, own_move: &Shape) -> Self {
        Self { opponent_move: *opponent_move, own_move: *own_move }
    }

    pub fn get_result(&self) -> u32 {
        self.own_move.get_points() + self.own_move.calculate_outcome(&self.opponent_move).get_points()
    }
}

enum Outcome {
    Win,
    Draw,
    Loose
}

impl Outcome {
    pub fn get_points(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loose => 0
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    pub fn get_points(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3
        }
    }

    pub fn calculate_outcome(&self, other: &Self) -> Outcome {
        if *self == *other {
            Outcome::Draw
        } else {
            if *self == Shape::Rock {
                if *other == Shape::Scissors {
                    Outcome::Win
                } else {
                    Outcome::Loose
                }
            } else if *self == Shape::Paper {
                if *other == Shape::Rock {
                    Outcome::Win
                } else {
                    Outcome::Loose
                }
            } else {
                if *other == Shape::Paper {
                    Outcome::Win
                } else {
                    Outcome::Loose
                }
            }
        }
    }
}
