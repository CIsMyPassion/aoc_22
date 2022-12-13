use std::{fs, path::Path, str::FromStr};

fn main() {
    part_one();
}

fn read_input() -> String {
    let path = Path::new("res/input");
    fs::read_to_string(path).expect("input file needed")
}

fn input_to_pairs_of_lists(input: &str) -> Vec<ListPair> {
    input.split("\n\n").map(|block| block.parse()).flatten().collect()
}

fn part_one() {
    let input = read_input();
    let pairs = input_to_pairs_of_lists(&input);
}

#[derive(Debug)]
struct ListPair {
    left: NestedList,
    right: NestedList,
}

impl FromStr for ListPair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split("\n").filter(|line| !line.is_empty()).collect();
        if lines.len() != 2 {
            Err(())
        } else {
            let left = lines[0].parse();
            let right = lines[1].parse();

            if left.is_ok() && right.is_ok() {
                Ok(ListPair { left: left.unwrap(), right: right.unwrap() })
            } else {
                Err(())
            }
        }
    }
}

#[derive(Debug)]
struct NestedList {
    list: Vec<ListItem>,
}

impl FromStr for NestedList {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("[") && s.ends_with("]") {
            let truncated = &s[1..s.len()-1];
            // TODO: rework this to work with nested lists containing more than one element
            let elements = truncated.split(",");

            let parsed: Result<Vec<ListItem>, _> = elements.map(|element| element.parse()).collect();

            match parsed {
                Ok(list) => Ok(NestedList { list }),
                Err(_) => Err(()),
            }
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
enum ListItem {
    Number(u64),
    NestedList(NestedList),
}

impl FromStr for ListItem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') {
            dbg!(s);
            match s.parse() {
                Ok(list) => Ok(ListItem::NestedList(list)),
                Err(_) => Err(()),
            }
        } else {
            match s.parse() {
                Ok(num) => Ok(ListItem::Number(num)),
                Err(_) => Err(()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#;

    #[test]
    fn parse_test() {
        let pairs = input_to_pairs_of_lists(INPUT);
        dbg!(&pairs);
        assert_eq!(pairs.len(), 8);
    }

    #[test]
    fn part_one_test() {
        let pairs = input_to_pairs_of_lists(INPUT);
    }
}
