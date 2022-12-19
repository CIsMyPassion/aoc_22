use super::*;

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

        let mut area = vec![vec![Filling::Air; lower_bound + 1]; width];

        rock_set.iter().for_each(|pos| {
            area[pos.0 - left_bound][pos.1] = Filling::Rock;
        });

        Self { left_bound, right_bound, lower_bound, area }
    }

    pub fn drop_sand(&mut self, drop_location: Position) -> bool {
        let mut sand_position = drop_location;
        loop {
            let step = self.sand_step(&mut sand_position);
            let is_in_loop = self.is_in_bounds(&sand_position);

            if !step || !is_in_loop {
                break;
            }
        }

        let is_in = self.is_in_bounds(&sand_position);
        is_in
    }

    pub fn sand_count(&self) -> usize {
        let mut sand_counter = 0;

        self.area.iter().for_each(|line| {
            sand_counter += line.iter().filter(|&element| *element == Filling::Sand).count();
        });

        sand_counter
    }

    #[cfg(test)]
    pub fn left_bound(&self) -> usize {
        self.left_bound
    }

    #[cfg(test)]
    pub fn right_bound(&self) -> usize {
        self.right_bound
    }

    #[cfg(test)]
    pub fn lower_bound(&self) -> usize {
        self.lower_bound
    }

    #[cfg(test)]
    pub fn rock_count(&self) -> usize {
        let mut rock_counter = 0;

        self.area.iter().for_each(|line| {
            rock_counter += line.iter().filter(|&element| *element == Filling::Rock).count();
        });

        rock_counter
    }

    fn sand_step(&mut self, sand_position: &mut Position) -> bool {
        if sand_position.1 >= self.lower_bound {
            sand_position.1 += 1;
            true
        } else {
            let below_middle = self.get_filling(&Position(sand_position.0, sand_position.1 + 1));
            let below_left = self.get_filling(&Position(sand_position.0 - 1, sand_position.1 + 1));
            let below_right = self.get_filling(&Position(sand_position.0 + 1, sand_position.1 + 1));

            if below_middle == Filling::Air {
                sand_position.1 += 1;
                return true
            } else if below_left == Filling::Air {
                sand_position.1 += 1;
                sand_position.0 -= 1;
                return true
            } else if below_right == Filling::Air {
                sand_position.1 += 1;
                sand_position.0 += 1;
                return true
            } else {
                self.set_filling(&sand_position, Filling::Sand);
                false
            }
        }
    }

    fn get_filling(&self, position: &Position) -> Filling {
        if self.is_in_bounds(position) {
            self.area[position.0 - self.left_bound][position.1]
        } else {
            Filling::Air
        }
    }

    fn set_filling(&mut self, position: &Position, filling: Filling) {
        self.area[position.0 - self.left_bound][position.1] = filling;
    }

    fn is_in_bounds(&self, sand_position: &Position) -> bool {
        if sand_position.0 > self.right_bound || sand_position.0 < self.left_bound {
            false
        } else {
            sand_position.1 <= self.lower_bound
        }
    }
}
