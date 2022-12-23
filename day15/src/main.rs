pub mod puzzle;

use std::{collections::HashSet, ops::RangeInclusive};

use puzzle::*;

const FREQUENCY_MULTIPLIER: i64 = 4_000_000;

fn main() {
    part_one();
    part_two();
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

fn combined_ranges_for_row(sensors: &Vec<Sensor>, line: i64) -> Vec<RangeInclusive<i64>> {
    let mut sensor_coverage: Vec<_> = sensors.iter().flat_map(|sensor| sensor.covered_in_line(line)).collect();
    sensor_coverage.sort_by_key(|cover| *cover.start());
    
    let mut combined = Vec::new();
    combined.push(sensor_coverage[0].clone());
    
    for i in 1..sensor_coverage.len() {
        let last_index = combined.len() - 1;
        let last_range = &combined[last_index];
        let new_range = &sensor_coverage[i];
        
        if last_range.end() >= new_range.start() {
            if new_range.end() > last_range.end() {
                combined[last_index] = *last_range.start()..=*new_range.end();
            }
        } else {
            combined.push(new_range.clone());
        }
    }
    
    combined
}

fn find_uncovered_coord(combined_range_rows: Vec<Vec<RangeInclusive<i64>>>, start: i64, end: i64) -> Coordinates {
    
    for row_index in 0..combined_range_rows.len() {
        let row = &combined_range_rows[row_index];
        if row.len() > 1 {
            dbg!(&row);
            for i in 0..row.len() {
                let sub_range = &row[i];
                if *sub_range.end() >= start && *sub_range.end() <= end {
                    return Coordinates::new(sub_range.end() + 1, row_index as i64);
                }
            }
        }
    }

    Coordinates::new(0, 0)
}

fn tuning_frequency(coord: &Coordinates) -> i64 {
    coord.x() * FREQUENCY_MULTIPLIER + coord.y()
}

fn part_one() {
    let input = day_util::read_input_safe("day15");
    let sensors = input_to_sensors(&input);
    
    let covered_count = collect_covered_spaces_in_line(&sensors, 2000000);
    println!("Covered in line: {covered_count}");
}
fn part_two() {
    let input = day_util::read_input_safe("day15");
    let sensors = input_to_sensors(&input);
    let combined_cover_rows: Vec<Vec<RangeInclusive<i64>>> = (0..4_000_000).into_iter().map(|line| combined_ranges_for_row(&sensors, line)).collect();
    let uncovered_coord = find_uncovered_coord(combined_cover_rows, 0, 4_000_000);
    let tuning_frequency = tuning_frequency(&uncovered_coord);
        
    println!("Frequency: {tuning_frequency}");
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

    #[test]
    fn part_two_test() {
        let sensors = input_to_sensors(TEST_INPUT);
        let combined_cover_rows: Vec<Vec<RangeInclusive<i64>>> = (0..20).into_iter().map(|line| combined_ranges_for_row(&sensors, line)).collect();
        let uncovered_coord = find_uncovered_coord(combined_cover_rows, 0, 20);
        let tuning_frequency = tuning_frequency(&uncovered_coord);
        
        assert_eq!(uncovered_coord.x(), 14);
        assert_eq!(uncovered_coord.y(), 11);
        assert_eq!(tuning_frequency, 56000011);
    }
}