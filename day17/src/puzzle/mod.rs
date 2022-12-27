use std::{collections::HashSet, ops::{Add, Sub}};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filling {
    Rock,
    Air,
}

pub struct Chamber {
    width: u64,
    filled_space: Vec<Vec<Filling>>,
    dropped_rocks: u64,
    push_list: Vec<PushDirection>,
    drop_counter: u64,
}

impl Chamber {
    pub fn new(width: u64, push_list: Vec<PushDirection>) -> Self {
        Self { width, filled_space: Vec::new(), dropped_rocks: 0, push_list, drop_counter: 0 }
    }
    
    pub fn drop_rock(&mut self) {
        self.dropped_rocks += 1;
        let rock = RockShape::from_index(self.dropped_rocks);
        let rock_shape = rock.shape();
        let mut rock_position = Position::new(self.highest_rock() + 3, 2);

        let width = rock_shape.iter().map(|pos| pos.column).max().unwrap();
        loop {
            let push_direction = self.current_push_direction();
            self.drop_counter += 1;
            
            match push_direction {
                PushDirection::Left => {
                    if rock_position.column > 0 {
                        rock_position.column -= 1;

                        if self.collides(&rock_position, &rock_shape) {
                            rock_position.column += 1;
                        }
                    }
                    
                },
                PushDirection::Right => {
                    if rock_position.column + width < self.width - 1 {
                        rock_position.column += 1;
                    
                        if self.collides(&rock_position, &rock_shape) {
                            rock_position.column -= 1;
                        }
                    }
                },
            };
            
            if rock_position.row > 0 {
                rock_position.row -= 1;
                if self.collides(&rock_position, &rock_shape) {
                    rock_position.row += 1;
                    self.set_rock(&rock_position, &rock_shape);
                    break;
                }
            } else {
                self.set_rock(&rock_position, &rock_shape);
                break;
            }
            
        }
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

    fn current_push_direction(&self) -> PushDirection {
        let index = self.drop_counter as usize % self.push_list.len();
        self.push_list[index]
    }

    pub fn dropped_rocks(&self) -> u64 {
        self.dropped_rocks
    }
    
    pub fn highest_rock(&self) -> u64 {
        self.filled_space.len() as u64
    }
    
    #[cfg(test)]
    pub fn filled_line(&self, index: usize) -> Vec<Filling> {
        self.filled_space[index].clone()
    }
}