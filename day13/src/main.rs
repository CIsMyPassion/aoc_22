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
    let sum: usize = calculate_correct_order_index_sum(&pairs);

    println!("Index sum: {sum}");
}

fn calculate_correct_order_index_sum(pairs: &Vec<ListPair>) -> usize {
    pairs.iter().enumerate().map(|(i, pair)| {
        if pair.is_correct_order() {
            Some(i + 1)
        } else {
            None
        }
    }).flatten().sum()
}

#[derive(Debug, PartialEq)]
struct ListPair {
    left: NestedList,
    right: NestedList,
}

impl ListPair {
    pub fn is_correct_order(&self) -> bool {
        self.left < self.right
    }
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct NestedList {
    list: Vec<ListItem>,
}

impl FromStr for NestedList {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("[") && s.ends_with("]") {
            let truncated = &s[1..s.len()-1];

            if truncated.len() == 0 {
                return Ok(NestedList { list: Vec::new() })
            }

            let parts: Vec<&str> = truncated.split(",").collect();
            let mut combined_elements = Vec::new();
            let mut nest_start = 0;
            let mut nest_level: i64 = 0;


            for i in 0..parts.len() {
                let part = parts[i];
                let new_nest = part.matches("[").count() as i64 - part.matches("]").count() as i64;
                if nest_level == 0 {
                    nest_start = i;
                }
                nest_level += new_nest;
                if new_nest > 0 {
                    if nest_level == 0 {
                        nest_start = i;
                    }
                } else if new_nest < 0 {
                    if nest_level == 0 {
                        let combined = parts[nest_start..=i].join(",");
                        combined_elements.push(combined);
                    }
                } else {
                    if nest_level == 0 {
                        combined_elements.push(part.to_owned());
                    }
                }
            }

            let parsed: Result<Vec<ListItem>, _> = combined_elements.iter().map(|element| element.parse()).collect();

            match parsed {
                Ok(list) => Ok(NestedList { list }),
                Err(_) => Err(()),
            }
        } else {
            Err(())
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ListItem {
    Number(u64),
    NestedList(NestedList),
}

impl FromStr for ListItem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') {
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

impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::NestedList(self_list), Self::NestedList(other_list)) => self_list.cmp(other_list),
            (Self::NestedList(self_list), Self::Number(other_num)) => self_list.cmp(&NestedList { list: vec![ListItem::Number(*other_num)] }),
            (Self::Number(self_num), Self::NestedList(other_list)) => NestedList { list: vec![ListItem::Number(*self_num)] }.cmp(other_list),
            (Self::Number(self_num), Self::Number(other_num)) => self_num.cmp(other_num),
        }
    }
}

impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
        let pair0 = ListPair { left: NestedList { list: Vec::from([ListItem::Number(1), ListItem::Number(1), ListItem::Number(3), ListItem::Number(1), ListItem::Number(1)]) },
                               right: NestedList { list: Vec::from([ListItem::Number(1), ListItem::Number(1), ListItem::Number(5), ListItem::Number(1), ListItem::Number(1)]) } };
        let pair1 = ListPair { left: NestedList { list: Vec::from([ListItem::NestedList(NestedList { list: Vec::from([ListItem::Number(1)]) }), ListItem::NestedList(NestedList { list: Vec::from([ListItem::Number(2), ListItem::Number(3), ListItem::Number(4)]) })]) },
                               right: NestedList { list: Vec::from([ListItem::NestedList(NestedList { list: Vec::from([ListItem::Number(1)]) }), ListItem::Number(4)]) } };
        let pair2 = ListPair { left: NestedList { list: Vec::from([ListItem::Number(9)]) },
                               right: NestedList { list: Vec::from([ListItem::NestedList(NestedList { list: Vec::from([ListItem::Number(8), ListItem::Number(7), ListItem::Number(6)]) })]) } };
        let pair3 = ListPair { left: NestedList { list: Vec::from([ListItem::NestedList(NestedList { list: Vec::from([ListItem::Number(4), ListItem::Number(4)]) }), ListItem::Number(4), ListItem::Number(4)]) },
                               right: NestedList { list: Vec::from([ListItem::NestedList(NestedList { list: Vec::from([ListItem::Number(4), ListItem::Number(4)]) }), ListItem::Number(4), ListItem::Number(4), ListItem::Number(4)]) } };
        let pair4 = ListPair { left: NestedList { list: Vec::from([ListItem::Number(7), ListItem::Number(7), ListItem::Number(7), ListItem::Number(7)]) },
                               right: NestedList { list: Vec::from([ListItem::Number(7), ListItem::Number(7), ListItem::Number(7)]) } };
        let pair5 = ListPair { left: NestedList { list: Vec::new() },
                               right: NestedList { list: Vec::from([ListItem::Number(3)]) } };
        let pair6 = ListPair { left: NestedList { list: Vec::from([ListItem::NestedList(NestedList { list: Vec::from([ListItem::NestedList(NestedList { list: Vec::new() })]) })]) },
                               right: NestedList { list: Vec::from([ListItem::NestedList(NestedList { list: Vec::new() })]) } };
        let pair7 = ListPair { left: NestedList {
            list: Vec::from([ListItem::Number(1), ListItem::NestedList(NestedList {
                list: Vec::from([ListItem::Number(2), ListItem::NestedList(NestedList {
                    list: Vec::from([ListItem::Number(3), ListItem::NestedList(NestedList {
                        list: Vec::from([ListItem::Number(4), ListItem::NestedList(NestedList {
                            list: Vec::from([ListItem::Number(5), ListItem::Number(6), ListItem::Number(7)])
                        })])
                    })])
                })])
            }), ListItem::Number(8), ListItem::Number(9)])
        }, right: NestedList {
            list: Vec::from([ListItem::Number(1), ListItem::NestedList(NestedList {
                list: Vec::from([ListItem::Number(2), ListItem::NestedList(NestedList {
                    list: Vec::from([ListItem::Number(3), ListItem::NestedList(NestedList {
                        list: Vec::from([ListItem::Number(4), ListItem::NestedList(NestedList {
                            list: Vec::from([ListItem::Number(5), ListItem::Number(6), ListItem::Number(0)])
                        })])
                    })])
                })])
            }), ListItem::Number(8), ListItem::Number(9)])
        } };

        assert_eq!(pairs[0], pair0);
        assert_eq!(pairs[1], pair1);
        assert_eq!(pairs[2], pair2);
        assert_eq!(pairs[3], pair3);
        assert_eq!(pairs[4], pair4);
        assert_eq!(pairs[5], pair5);
        assert_eq!(pairs[6], pair6);
        assert_eq!(pairs[7], pair7);

        assert_eq!(pairs.len(), 8);
    }

    #[test]
    fn part_one_test() {
        let pairs = input_to_pairs_of_lists(INPUT);

        assert_eq!(pairs[0].is_correct_order(), true);
        assert_eq!(pairs[1].is_correct_order(), true);
        assert_eq!(pairs[2].is_correct_order(), false);
        assert_eq!(pairs[3].is_correct_order(), true);
        assert_eq!(pairs[4].is_correct_order(), false);
        assert_eq!(pairs[5].is_correct_order(), true);
        assert_eq!(pairs[6].is_correct_order(), false);
        assert_eq!(pairs[7].is_correct_order(), false);

        assert_eq!(calculate_correct_order_index_sum(&pairs), 13);
    }
}
