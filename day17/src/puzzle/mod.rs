use std::{collections::{HashSet, HashMap}, ops::{Add, Sub}};

#[derive(Copy, Clone)]
pub enum PushDirection {
    Left,
    Right,
}

impl PushDirection {
    pub fn parse_to_vec(s: &str) -> Vec<Self> {
        s.chars().flat_map(|c| {
            match c {
                '<' => Some(PushDirection::Left),
                '>' => Some(PushDirection::Right),
                _ => None,
            }
        }).collect()
    }
}

enum RockShape {
    HorizontalLine,
    Plus,
    BackwardsL,
    VerticalLine,
    Cube,
}

impl RockShape {
    pub fn from_index(index: u64) -> Self {
        match (index - 1) % 5 {
            0 => RockShape::HorizontalLine,
            1 => RockShape::Plus,
            2 => RockShape::BackwardsL,
            3 => RockShape::VerticalLine,
            _ => RockShape::Cube,
        }
    }
    
    pub fn shape(&self) -> HashSet<Position> {
        match self {
            Self::HorizontalLine => HashSet::from([
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(0, 2),
                Position::new(0, 3),
            ]),
            Self::Plus => HashSet::from([
                Position::new(0, 1),
                Position::new(1, 0),
                Position::new(1, 1),
                Position::new(1, 2),
                Position::new(2, 1),
            ]),
            Self::BackwardsL => HashSet::from([
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(0, 2),
                Position::new(1, 2),
                Position::new(2, 2),
            ]),
            Self::VerticalLine => HashSet::from([
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(2, 0),
                Position::new(3, 0),
            ]),
            Self::Cube => HashSet::from([
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(0, 1),
                Position::new(1, 1),
            ]),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: u64,
    column: u64,
}

impl Position {
    pub fn new(row: u64, column: u64) -> Self {
        Self { row, column }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            column: self.column + rhs.column,
        }
    }
}

impl Sub for Position {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row - rhs.row,
            column: self.column - rhs.column,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Filling {
    Rock,
    Air,
}

pub fn height_for_dropped_rocks(chamber: &mut Chamber, push_list: Vec<PushDirection>, rock_count: u64) -> u64 {
    let mut dropped_rocks = 0;
    let mut jet_index = 0;
    let mut state_map = HashMap::new();

    while dropped_rocks < rock_count {
        dropped_rocks += 1;

        let rock = RockShape::from_index(dropped_rocks);
        let rock_shape = rock.shape();
        let mut rock_position = Position::new(chamber.highest_rock() + 3, 2);

        let width = rock_shape.iter().map(|pos| pos.column).max().unwrap();
        loop {
            jet_index = jet_index % push_list.len();
            let push_direction = push_list[jet_index];
            jet_index += 1;
            
            match push_direction {
                PushDirection::Left => {
                    if rock_position.column > 0 {
                        rock_position.column -= 1;

                        if chamber.collides(&rock_position, &rock_shape) {
                            rock_position.column += 1;
                        }
                    }
                    
                },
                PushDirection::Right => {
                    if rock_position.column + width < chamber.width as u64 - 1 {
                        rock_position.column += 1;
                    
                        if chamber.collides(&rock_position, &rock_shape) {
                            rock_position.column -= 1;
                        }
                    }
                },
            };
            
            if rock_position.row > 0 {
                rock_position.row -= 1;
                if chamber.collides(&rock_position, &rock_shape) {
                    rock_position.row += 1;
                    chamber.set_rock(&rock_position, &rock_shape);
                    break;
                }
            } else {
                chamber.set_rock(&rock_position, &rock_shape);
                break;
            }
        }
        
        let state = create_state(chamber, jet_index, (dropped_rocks % 5) as usize);
        
        if dropped_rocks % 10000 == 0 {
            dbg!(&state);
            dbg!(state_map.len());
        }

        if state_map.contains_key(&state) {
            dbg!(state);
            todo!("PLS HELP")
        } else {
            state_map.insert(state, (dropped_rocks, chamber.highest_rock()));
        }
    }
    
    chamber.highest_rock()
}

fn create_state(chamber: &Chamber, jet_index: usize, rock_index: usize) -> State {
    let mut rock_distance = Vec::new();
    
    for i in 0..chamber.width {
        for j in (0..chamber.filled_space.len()).rev() {
            if chamber.filled_space[j][i] == Filling::Rock {
                rock_distance.push(chamber.filled_space.len() - (j + 1));
                break;
            }
        }
        
        if rock_distance.len() < i + 1 {
            rock_distance.push(chamber.highest_rock() as usize);
        }
    }
    
    assert_eq!(rock_distance.len(), chamber.width);
    
    State { rock_distance, jet_index, rock_index }
}

pub struct Chamber {
    width: usize,
    filled_space: Vec<Vec<Filling>>,
}

impl Chamber {
    pub fn new(width: usize) -> Self {
        Self { width, filled_space: Vec::new() }
    }

    fn collides(&self, position: &Position, shape: &HashSet<Position>) -> bool {
        if position.row < self.filled_space.len() as u64 {
            for piece in shape {
                let total_position = *position + *piece;
                if total_position.row < self.filled_space.len() as u64 {
                    match self.filled_space[total_position.row as usize][total_position.column as usize] {
                        Filling::Air => (),
                        Filling::Rock => return true,
                    }
                }
            }
            false
        } else {
            false
        }
    }

    fn set_rock(&mut self, position: &Position, shape: &HashSet<Position>) {
        let height = shape.iter().map(|pos| pos.row).max().unwrap();
        let total_height = position.row + height;
        for _ in self.filled_space.len() as u64..total_height + 1 {
            self.filled_space.push(vec![Filling::Air; self.width as usize]);
        }
        
        shape.iter().for_each(|pos| {
            let total_position = *pos + *position;
            self.filled_space[total_position.row as usize][total_position.column as usize] = Filling::Rock;
        });
    }

    pub fn highest_rock(&self) -> u64 {
        self.filled_space.len() as u64
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct State {
    rock_distance: Vec<usize>,
    jet_index: usize,
    rock_index: usize,
}