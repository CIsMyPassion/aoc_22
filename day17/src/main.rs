use puzzle::*;

mod puzzle;

fn main() {
    //part_one();
    part_two();
}

fn part_one() {
    let input = day_util::read_input_safe("day17");
    let push_list = PushDirection::parse_to_vec(&input);
    let mut chamber = Chamber::new(7);
    let height = height_for_dropped_rocks(&mut chamber, push_list, 2022);
    
    println!("Highest rock: {}", height);
}

fn part_two() {
    let input = day_util::read_input_safe("day17");
    let push_list = PushDirection::parse_to_vec(&input);
    let mut chamber = Chamber::new(7);
    let height = height_for_dropped_rocks(&mut chamber, push_list, 1000000000000);
    
    println!("Highest rock: {}", height);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_INPUT: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
"#;

    #[test]
    fn part_one_test() {
        let push_list = PushDirection::parse_to_vec(TEST_INPUT);
        let mut chamber = Chamber::new(7);
        let height = height_for_dropped_rocks(&mut chamber, push_list, 2022);
        
        assert_eq!(height, 3068);
    }

    #[test]
    fn part_two_test() {
        let push_list = PushDirection::parse_to_vec(TEST_INPUT);
        let mut chamber = Chamber::new(7);
        let height = height_for_dropped_rocks(&mut chamber, push_list, 1000000000000);
        
        assert_eq!(height, 1514285714288);
    }
}