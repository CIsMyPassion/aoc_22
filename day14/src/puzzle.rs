use std::str::FromStr;

pub struct Position(pub u64, pub u64);

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err(())
    }
}

pub struct RockShape {
    line: Vec<Position>
}

impl FromStr for RockShape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.split(" -> ").map(|position| position.parse().unwrap()).collect();
        Ok(RockShape { line })
    }
}

pub enum Filling {
    Air,
    Rock,
    Sand,
}

pub struct Cave {
    area: Vec<Vec<Filling>>
}

impl Cave {
    pub fn new(rock_shapes: Vec<RockShape>) -> Self {
        todo!()
    }
}
