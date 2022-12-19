use super::*;
use super::cave::Cave;

#[derive(Debug)]
pub struct InfiniteCave {
    floor_level: usize,
    rock_set: HashSet<Position>,
    sand_set: HashSet<Position>,
}

impl Cave for InfiniteCave {
    fn drop_sand(&mut self, drop_location: Position) -> bool {
        if self.sand_set.contains(&drop_location) {
            false
        } else {
            let mut sand_position = drop_location;
            loop {
                if !self.sand_step(&mut sand_position) {
                    break
                }
            }
            true
        }
    }

    fn sand_count(&self) -> usize {
        self.sand_set.len()
    }

    #[cfg(test)]
    fn rock_count(&self) -> usize {
        self.rock_set.len()
    }
}

impl InfiniteCave {
    pub fn new(rock_shapes: Vec<RockShape>) -> Self {
        let mut rock_set = HashSet::new();

        for rock_shape in rock_shapes {
            for rock_position in rock_shape.rock_positions() {
                rock_set.insert(rock_position);
            }
        }

        let lower_bound = rock_set.iter().map(|pos| pos.1).max().unwrap();

        Self { floor_level: lower_bound + 2, rock_set, sand_set: HashSet::new() }
    }

    fn sand_step(&mut self, sand_position: &mut Position) -> bool {

        if sand_position.1 >= self.floor_level - 1 {
            self.sand_set.insert(*sand_position);
            false

        } else {
            let left = Position(sand_position.0 - 1, sand_position.1 + 1);
            let middle = Position(sand_position.0, sand_position.1 + 1);
            let right = Position(sand_position.0 + 1, sand_position.1 + 1);

            let below_left = self.rock_set.contains(&left) || self.sand_set.contains(&left);
            let below_middle = self.rock_set.contains(&middle) || self.sand_set.contains(&middle);
            let below_right = self.rock_set.contains(&right) || self.sand_set.contains(&right);

            if !below_middle {
                sand_position.1 += 1;
                true
            } else if !below_left {
                sand_position.1 += 1;
                sand_position.0 -= 1;
                true
            } else if !below_right {
                sand_position.1 += 1;
                sand_position.0 += 1;
                true
            } else {
                self.sand_set.insert(*sand_position);
                false
            }
        }
    }

    #[cfg(test)]
    pub fn floor_level(&self) -> usize {
        self.floor_level
    }
}