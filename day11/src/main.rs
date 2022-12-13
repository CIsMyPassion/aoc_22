use std::{path::Path, fs, str::FromStr, collections::VecDeque};

fn main() {
    part_one();
    part_two();
}

fn read_input() -> String {
    let path = Path::new("res/input");
    fs::read_to_string(path).expect("input file needed")
}

fn input_to_pack(input: &str) -> Pack {
    let blocks = input.split("\n\n");
    Pack::new(blocks.map(|block| block.parse().unwrap()).collect())
}

fn part_one() {
    let input = read_input();
    let mut pack = input_to_pack(&input);
    pack_rounds(&mut pack, 3, 20);

    println!("Monkey business: {}", pack.monkey_business());
}

fn part_two() {
    let input = read_input();
    let mut pack = input_to_pack(&input);
    pack_rounds(&mut pack, 1, 10000);

    println!("Monkey business: {}", pack.monkey_business());
}

fn pack_rounds(pack: &mut Pack, panic_divider: usize, rounds: usize) {
    for _ in 0..rounds {
        pack.round(panic_divider);
    }
}

struct Pack {
    monkeys: Vec<Monkey>,
}

impl Pack {
    pub fn new(monkeys: Vec<Monkey>) -> Self {
        Self { monkeys }
    }

    pub fn monkey_business(&self) -> usize {
        let mut inspection_counts: Vec<usize> = self.monkeys.iter().map(|monkey| monkey.inspection_counter).collect();
        inspection_counts.sort();

        inspection_counts.iter().rev().take(2).product()
    }

    pub fn round(&mut self, panic_divider: usize) {
        let common_multiplier = self.common_multiple();

        for i in 0..self.monkeys.len() {
            let inspected_list = self.monkeys[i].inspect_items(panic_divider, common_multiplier);
            self.distribute_items(inspected_list);
        }
    }

    fn distribute_items(&mut self, mut inspected_list: VecDeque<(usize, usize)>) {
        while !inspected_list.is_empty() {
            let (item, target) = inspected_list.pop_front().unwrap();
            self.monkeys[target].items.push_back(item);
        }
    }

    fn common_multiple(&self) -> usize {
        self.monkeys.iter().map(|monkey| monkey.test.divisible_by).product()
    }
}

struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: Test,
    inspection_counter: usize,
}

impl Monkey {
    pub fn inspect_items(&mut self, panic_divider: usize, common_multiple: usize) -> VecDeque<(usize, usize)> {
        let mut inspected_list = VecDeque::new();

        while !self.items.is_empty() {
            inspected_list.push_back(self.inspect_item(panic_divider, common_multiple));
        }

        inspected_list
    }

    pub fn inspect_item(&mut self, panic_divider: usize, common_multiple: usize) -> (usize, usize) {
        let item = self.items.pop_front().unwrap();
        let new_value = (self.operation.apply(item) / panic_divider) % common_multiple;
        let target = self.test.evaluate(new_value);

        self.inspection_counter += 1;

        (new_value, target)
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split("\n").filter(|line| !line.is_empty()).collect();
        if lines.len() != 6 {
            Err(())
        } else {
            let items: VecDeque<usize> = lines[1].split(" ").skip(4).map(|item| item.replace(",", "").parse().unwrap()).collect();
            let operation = lines[2].parse().unwrap();
            let test = lines[3..=5].join("\n").parse().unwrap();

            Ok(Self { items, operation, test, inspection_counter: 0 })
        }
    }
}

#[derive(Debug, PartialEq)]
enum Operation {
    Add(Number),
    Mult(Number),
}

impl Operation {
    pub fn apply(&self, item: usize) -> usize {
        match self {
            Operation::Add(number) => {
                match number {
                    Number::Num(num) => item + num,
                    Number::Old => item + item,
                }
            },
            Operation::Mult(number) => {
                match number {
                    Number::Num(num) => item * num,
                    Number::Old => item * item,
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Number {
    Num(usize),
    Old
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pieces: Vec<&str> = s.trim().split(" ").collect();
        let number = match pieces[5] {
            "old" => Number::Old,
            num => Number::Num(num.parse().unwrap()),
        };

        match pieces[4] {
            "*" => Ok(Operation::Mult(number)),
            "+" => Ok(Operation::Add(number)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Test {
    divisible_by: usize,
    true_target: usize,
    false_target: usize,
}

impl Test {
    pub fn evaluate(&self, item: usize) -> usize {
        if item % self.divisible_by == 0 {
            self.true_target
        } else {
            self.false_target
        }
    }
}

impl FromStr for Test {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<Vec<&str>> = s.split("\n").map(|line| line.trim().split(" ").collect()).collect();
        let divisible_by = lines[0][3].parse().unwrap();
        let true_target = lines[1][5].parse().unwrap();
        let false_target = lines[2][5].parse().unwrap();

        Ok(Self { divisible_by, true_target, false_target })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;

    #[test]
    fn parser_test() {
        let pack = input_to_pack(INPUT);
        assert_eq!(pack.monkeys.len(), 4);
        assert_eq!(pack.monkeys[0].items, [79, 98]);
        assert_eq!(pack.monkeys[0].operation, Operation::Mult(Number::Num(19)));
        assert_eq!(pack.monkeys[0].test, Test { divisible_by: 23, true_target: 2, false_target: 3 });
    }

    #[test]
    fn part_one_test() {
        let mut pack = input_to_pack(INPUT);
        pack_rounds(&mut pack, 3, 20);
        assert_eq!(pack.monkey_business(), 10605);
    }

    #[test]
    fn part_two_test() {
        let mut pack = input_to_pack(INPUT);
        pack_rounds(&mut pack, 1, 10000);
        assert_eq!(pack.monkey_business(), 2713310158);
    }
}
