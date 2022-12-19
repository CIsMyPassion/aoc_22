use std::cell::RefCell;
use std::rc::Rc;
use crate::puzzle::*;
use crate::puzzle::cave::Cave;
use crate::puzzle::bottomless_cave::BottomlessCave;
use crate::puzzle::infinite_cave::InfiniteCave;

mod puzzle;

const SAND_SPAWN_POINT: Position = Position(500, 0);

fn main() {
    part_one();
    part_two();
}

fn input_to_bottomless_cave(input: &str) -> BottomlessCave {
    let rock_shape: Vec<RockShape> = input.split("\n").filter(|line| !line.is_empty()).map(|line| line.parse().unwrap()).collect();
    BottomlessCave::new(rock_shape)
}

fn input_to_infinite_cave(input: &str) -> InfiniteCave {
    let rock_shape: Vec<RockShape> = input.split("\n").filter(|line| !line.is_empty()).map(|line| line.parse().unwrap()).collect();
    InfiniteCave::new(rock_shape)
}

fn drop_until_full(cave: Rc<RefCell<dyn Cave>>) {
    loop {
        let drop_sand_value = cave.borrow_mut().drop_sand(SAND_SPAWN_POINT);
        if !drop_sand_value {
            break;
        }
    }
}

fn part_one() {
    let input = day_util::read_input();
    let cave = Rc::new(RefCell::new(input_to_bottomless_cave(&input)));

    drop_until_full(cave.clone());
    let sand_count = cave.borrow().sand_count();
    println!("Sand count: {sand_count}");
}

fn part_two() {
    let input = day_util::read_input();
    let cave = Rc::new(RefCell::new(input_to_infinite_cave(&input)));

    drop_until_full(cave.clone());
    let sand_count = cave.borrow().sand_count();
    println!("Sand count: {sand_count}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::cave::Cave;

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

        let bottomless_cave = input_to_bottomless_cave(TEST_INPUT);

        assert_eq!(bottomless_cave.left_bound(), 494);
        assert_eq!(bottomless_cave.right_bound(), 503);
        assert_eq!(bottomless_cave.lower_bound(), 9);
        assert_eq!(bottomless_cave.sand_count(), 0);
        assert_eq!(bottomless_cave.rock_count(), 20);

        let infinite_cave = input_to_infinite_cave(TEST_INPUT);

        assert_eq!(infinite_cave.floor_level(), 11);
        assert_eq!(infinite_cave.rock_count(), 20);
    }

    #[test]
    fn part_one_test() {
        let cave = Rc::new(RefCell::new(input_to_bottomless_cave(TEST_INPUT)));

        drop_until_full(cave.clone());
        assert_eq!(cave.borrow().sand_count(), 24);
    }

    #[test]
    fn part_two_test() {
        let cave = Rc::new(RefCell::new(input_to_infinite_cave(TEST_INPUT)));

        drop_until_full(cave.clone());
        assert_eq!(cave.borrow().sand_count(), 93);
    }
}
