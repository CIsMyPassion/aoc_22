use puzzle::*;

mod puzzle;

fn main() {
    part_one();
}

fn part_one() {
    let input = day_util::read_input_safe("day17");
    let push_list = PushDirection::parse_to_vec(&input);
    let mut chamber = Chamber::new(7, push_list);
        
    while chamber.dropped_rocks() < 2022 {
        chamber.drop_rock();
    }
    
    println!("Highest rock: {}", chamber.highest_rock());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_INPUT: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
"#;

    #[test]
    fn fine_test() {
        let push_list = PushDirection::parse_to_vec(TEST_INPUT);
        let mut chamber = Chamber::new(7, push_list);
        
        while chamber.dropped_rocks() < 11 {
            chamber.drop_rock();
        }
        
        assert_eq!(chamber.highest_rock(), 18);
    }

    #[test]
    fn part_one_test() {
        let push_list = PushDirection::parse_to_vec(TEST_INPUT);
        let mut chamber = Chamber::new(7, push_list);
        
        while chamber.dropped_rocks() < 2022 {
            chamber.drop_rock();
        }
        
        assert_eq!(chamber.highest_rock(), 3068);
    }
}