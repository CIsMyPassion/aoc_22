use std::{str::FromStr, collections::HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub fn line_between(&self, other: &Self) -> Vec<Position> {
        if self.0 == other.0 {
            let range = if self.1 > other.1 {
                other.1+1..self.1-1
            } else {
                self.1..other.1
            };
            dbg!(&range);
            range.into_iter().map(|num| Position(self.0, num)).collect()
        } else {
            let range = if self.0 > other.0 {
                other.0+1..self.0-1
            } else {
                self.0..other.0
            };
            dbg!(&range);
            range.into_iter().map(|num| Position(num, self.1)).collect()
        }
    }
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(",").collect();

        if parts.len() == 2 {
            Ok(Position(parts[0].parse().unwrap(), parts[1].parse().unwrap()))
        } else {
            Err(())
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RockShape {
    line: Vec<Position>
}

impl RockShape {
    pub fn new(line: Vec<Position>) -> Self {
        RockShape { line }
    }

    pub fn rock_positions(&self) -> Vec<Position> {

        if self.line.len() == 1 {
            vec![self.line[0]]
        } else {
            let mut collected_positions = Vec::new();

            for i in 0..self.line.len() - 1 {
                let line = self.line[i].line_between(&self.line[i + 1]);
                dbg!(&line);
                for element in line {
                    collected_positions.push(element);
                }
            }
            collected_positions.push(self.line[self.line.len() - 1]);

            collected_positions
        }
    }
}

impl FromStr for RockShape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.split(" -> ").map(|position| position.parse().unwrap()).collect();
        Ok(RockShape { line })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Filling {
    Air,
    Rock,
    Sand,
}

#[derive(Debug, PartialEq)]
pub struct Cave {
    left_bound: usize,
    right_bound: usize,
    lower_bound: usize,
    area: Vec<Vec<Filling>>,
}

impl Cave {
    pub fn new(rock_shapes: Vec<RockShape>) -> Self {
        let mut rock_set = HashSet::new();

        for rock_shape in rock_shapes {
            for rock_position in rock_shape.rock_positions() {
                rock_set.insert(rock_position);
            }
        }

        let left_bound = rock_set.iter().map(|pos| pos.0).min().unwrap();
        let right_bound = rock_set.iter().map(|pos| pos.0).max().unwrap();
        let lower_bound = rock_set.iter().map(|pos| pos.1).max().unwrap();
        let width = right_bound - left_bound + 1;

        let area = vec![vec![Filling::Air; width]; lower_bound];

        Self { left_bound, right_bound, lower_bound, area }
    }

    pub fn left_bound(&self) -> usize {
        self.left_bound
    }

    pub fn right_bound(&self) -> usize {
        self.right_bound
    }

    pub fn lower_bound(&self) -> usize {
        self.lower_bound
    }
}
