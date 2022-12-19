pub mod puzzle;

use std::collections::HashSet;

use puzzle::*;

fn main() {
    part_one();
}

fn input_to_sensors(input: &str) -> Vec<Sensor> {
    let lines = input.split("\n").filter(|line| !line.is_empty());
    lines.map(|line| line.parse().unwrap()).collect()
}

fn collect_covered_spaces_in_line(sensors: &Vec<Sensor>, line: i64) -> usize {
    let sensor_coverage = sensors.iter().map(|sensor| sensor.covered_in_line(line)).flatten();
    let mut covered_set = HashSet::new();
    
    sensor_coverage.for_each(|cover| cover.for_each(|pos| { covered_set.insert(pos); }));
    
    sensors.iter().for_each(|sensor| {
        if covered_set.contains(&sensor.closest_beacon().x()) && sensor.closest_beacon().y() == line {
            covered_set.remove(&sensor.closest_beacon().x());
        }
    });
    
    covered_set.len()
}

fn part_one() {
    let input = day_util::read_input_safe("day15");
    let sensors = input_to_sensors(&input);
    
    let covered_count = collect_covered_spaces_in_line(&sensors, 2000000);
    println!("Covered in line: {covered_count}");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_INPUT: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;

    #[test]
    fn part_one_test() {
        let sensors = input_to_sensors(TEST_INPUT);
        let cover_count = collect_covered_spaces_in_line(&sensors, 10);

        assert_eq!(cover_count, 26);
    }
}