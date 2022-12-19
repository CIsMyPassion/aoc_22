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

fn drop_until_max(cave: &mut Cave) {
    loop {
        if !cave.drop_sand(SAND_SPAWN_POINT) {
            break;
        }
    }
}

fn part_one() {
    let input = day_util::read_input();
    let mut cave = input_to_cave(&input);

    drop_until_max(&mut cave);
    let sand_count = cave.sand_count();
    println!("Sand count: {sand_count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn parse_test() {
        let rock_shapes: Vec<RockShape> = TEST_INPUT.split("\n").filter(|line| !line.is_empty()).map(|line| line.parse().unwrap()).collect();
        let known_rock_shapes = vec![RockShape::new(vec![Position(498, 4), Position(498, 6), Position(496, 6)]),
            RockShape::new(vec![Position(503, 4), Position(502, 4), Position(502, 9), Position(494, 9)])];

        assert_eq!(rock_shapes.len(), known_rock_shapes.len());
        assert_eq!(rock_shapes[0], known_rock_shapes[0]);
        assert_eq!(rock_shapes[1], known_rock_shapes[1]);

        let cave = input_to_cave(TEST_INPUT);

        assert_eq!(cave.left_bound(), 494);
        assert_eq!(cave.right_bound(), 503);
        assert_eq!(cave.lower_bound(), 9);
        assert_eq!(cave.sand_count(), 0);
        assert_eq!(cave.rock_count(), 20);
    }

    #[test]
    fn part_one_test() {
        let mut cave = input_to_cave(TEST_INPUT);

        drop_until_max(&mut cave);
        assert_eq!(cave.sand_count(), 24);
    }
}
