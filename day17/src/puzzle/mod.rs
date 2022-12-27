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
    let mut found_patterns = Vec::new();
    let mut total_pattern_height = 0;
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
                    if rock_position.column + width < chamber.width - 1 {
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
        
        if dropped_rocks % 1000 == 0 {
            state_map.insert(create_state(chamber, jet_index, 5), dropped_rocks);
        }
        
        if total_pattern_height == 0 {
            if let Some(pattern) = find_pattern(chamber) {
                found_patterns.push((pattern, dropped_rocks, jet_index));
            
                if let Some((before_rocks, pattern_rocks, before_height, pattern_height, dropped_counter)) = pattern_repeats(&found_patterns) {
                    let pattern_count = (rock_count - before_rocks) / pattern_rocks;
                    total_pattern_height =  before_height + pattern_height * pattern_count as usize;
                    dropped_rocks = before_rocks + pattern_count * pattern_rocks;
                    jet_index = dropped_counter;
                }
            }
        }
    }
    
    total_pattern_height as u64 + chamber.highest_rock()
}

fn create_state(chamber: &Chamber, jet_index: usize, top_layers_count: usize) -> State {
    let mut top_layers = vec![Vec::new(); top_layers_count];
    let start_layer = chamber.filled_space.len() - (top_layers_count + 1);
    
    top_layers.clone_from_slice(&chamber.filled_space[start_layer..]);
    
    State { top_layers, jet_index, height: chamber.filled_space.len() }
}

fn find_pattern(chamber: &mut Chamber) -> Option<Vec<Vec<Filling>>> {
    
    for i in 0..chamber.filled_space.len() {
        if !chamber.filled_space[i].contains(&Filling::Air) {
            let mut pattern = vec![Vec::new(); i + 1];
            pattern.clone_from_slice(&chamber.filled_space[0..=i]);
            
            let remaining_len = chamber.filled_space.len() - (i + 1);
            if remaining_len > 0 {
                let mut rest = vec![Vec::new(); remaining_len];
                rest.clone_from_slice(&chamber.filled_space[i + 1..]);
                
                chamber.filled_space = rest;
            } else {
                chamber.filled_space = Vec::new()
            }

            return Some(pattern)
        }
    }

    None
}

fn pattern_repeats(patterns: &Vec<(Vec<Vec<Filling>>, u64, usize)>) -> Option<(u64, u64, usize, usize, usize)> {
    for i in 0..&patterns.len() - 1 {
        let last = &patterns[patterns.len() - 1]; 
        let previous = &patterns[i];
        
        if last.0 == previous.0 && last.2 == previous.2 {
            dbg!(i);
            dbg!(patterns.len() - 1);
            let before_height: usize = patterns[..i].iter().map(|pattern| pattern.0.len()).sum();
            let before_rocks = if i == 0 {
                0
            } else {
                patterns[i].1
            };
            let repition_height: usize = patterns[i..patterns.len()].iter().map(|pattern| pattern.0.len()).sum();
            let repition_rocks: u64 = patterns[patterns.len() - 1].1 - before_rocks;

            dbg!(before_height);
            dbg!(before_rocks);

            dbg!(repition_height);
            dbg!(repition_rocks);

            return Some((before_rocks, repition_rocks, before_height, repition_height, last.2))
        }
    }

    None
}

pub struct Chamber {
    width: u64,
    filled_space: Vec<Vec<Filling>>,
}

impl Chamber {
    pub fn new(width: u64) -> Self {
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

#[derive(Hash, PartialEq, Eq)]
struct State {
    top_layers: Vec<Vec<Filling>>,
    jet_index: usize,
    height: usize,
}