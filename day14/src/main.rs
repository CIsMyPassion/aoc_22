use crate::puzzle::*;

mod puzzle;

const SAND_SPAWN_POINT: Position = Position(500, 0);

fn main() {
    part_one();
}

fn input_to_cave(input: &str) -> Cave {
    let rock_shape: Vec<RockShape> = input.split("\n").filter(|line| !line.is_empty()).map(|line| line.parse().unwrap()).collect();
    Cave::new(rock_shape)
}

fn part_one() {
    let input = day_util::read_input();
    let cave = input_to_cave(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {

    }
}
