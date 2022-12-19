use std::{str::FromStr, ops::RangeInclusive};

#[derive(Debug, Clone, Copy)]
pub struct Coordinates {
    x: i64,
    y: i64,
}

impl Coordinates {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    
    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }
}

impl FromStr for Coordinates {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(", ").collect();
        if parts.len() != 2 {
            Err(())
        } else {
            let x_part = &parts[0][2..];
            let y_part = &parts[1][2..];
            let x = x_part.parse().unwrap();
            let y = y_part.parse().unwrap();
            
            Ok(Coordinates { x, y })
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sensor {
    position: Coordinates,
    closest_beacon: Coordinates,
}

impl Sensor {
    pub fn coverage(&self) -> u64 {
        let diff = self.diff();
        diff.x as u64 + diff.y as u64
    }
    
    pub fn covered_in_line(&self, line: i64) -> Option<RangeInclusive<i64>> {
        let coverage = self.coverage();
        let vertical_diff = line.abs_diff(self.position.y);
        
        if coverage >= vertical_diff {
            let horizontal_reach = coverage as i64 - vertical_diff as i64;
            Some(self.position.x-horizontal_reach..=self.position.x+horizontal_reach)
        } else {
            None
        }
    }

    pub fn closest_beacon(&self) -> Coordinates {
        self.closest_beacon
    }
    
    fn diff(&self) -> Coordinates {
        let x_diff = self.position.x.abs_diff(self.closest_beacon.x);
        let y_diff = self.position.y.abs_diff(self.closest_beacon.y);
        
        Coordinates::new(x_diff as i64, y_diff as i64)
    }
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        if parts.len() != 10 {
            Err(())
        } else {
            let position_part = parts[2..=3].join(" ");
            let position = position_part[0..position_part.len() - 1].parse().unwrap();
            let closest_beacon = parts[8..=9].join(" ").parse().unwrap();
            
            Ok(Sensor { position, closest_beacon })
        }
    }
}