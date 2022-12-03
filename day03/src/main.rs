use std::{path::Path, fs, collections::HashSet};

fn main() {
    part_one();
    part_two();
}

fn read_input() -> Vec<Rucksack> {
    let path = Path::new("res/input");
    let contetns = fs::read_to_string(path).expect("input file needed");
    let lines = contetns.split("\n");

    lines.filter(|line| !line.is_empty()).map(|line| line_to_rucksack(line)).collect()
}

fn line_to_rucksack(line: &str) -> Rucksack {
    let (first, second) = line.split_at(line.len() / 2);
    assert_eq!(first.len(), second.len());

    let first_vec = first.chars().map(|c| letter_to_item(c)).collect();
    let second_vec = second.chars().map(|c| letter_to_item(c)).collect();

    Rucksack::new(first_vec, second_vec)
}

fn letter_to_item(c: char) -> Item {
    let case = match c.is_uppercase() {
        true => Case::Upper,
        false => Case::Lower
    };

    let letter = (c.to_ascii_lowercase() as u8 - 96).try_into().expect("wrong letter");

    Item::new(case, letter)
}

fn part_one() {
    let rucksacks = read_input();

    let mut total_priority = 0;

    rucksacks.iter().for_each(|r| {
        total_priority += r.find_matching_item().prioraty();
    });

    println!("Total priority: {}", total_priority);
}

fn part_two() {
    let rucksacks = read_input();
    let mut total_priority = 0;
    rucksacks.chunks(3).map(|slice| {
        find_group_item(slice)
    }).for_each(|item| {
        total_priority += item.prioraty();
    });

    println!("Total group priority: {}", total_priority);
}

fn find_group_item(group: &[Rucksack]) -> Item {
    assert_eq!(group.len(), 3);
    let mut first_set: HashSet<Item> = HashSet::from_iter(group[0].first_compartment.iter().cloned());
    first_set.extend::<HashSet<Item>>(HashSet::from_iter(group[0].second_compartment.iter().cloned()));
    let mut second_set: HashSet<Item> = HashSet::from_iter(group[1].first_compartment.iter().cloned());
    second_set.extend::<HashSet<Item>>(HashSet::from_iter(group[1].second_compartment.iter().cloned()));
    let mut third_set: HashSet<Item> = HashSet::from_iter(group[2].first_compartment.iter().cloned());
    third_set.extend::<HashSet<Item>>(HashSet::from_iter(group[2].second_compartment.iter().cloned()));

    let intersection: HashSet<Item> = first_set.intersection(&second_set).map(|item| *item).collect();
    let second_intersection: Vec<&Item> = intersection.intersection(&third_set).collect();

    assert_eq!(second_intersection.len(), 1);
    *second_intersection[0]
}

#[derive(Debug)]
struct Rucksack {
    first_compartment: Vec<Item>,
    second_compartment: Vec<Item>
}

impl Rucksack {
    pub fn new(first_compartment: Vec<Item>, second_compartment: Vec<Item>) -> Self {
        Rucksack { first_compartment, second_compartment }
    }

    pub fn find_matching_item(&self) -> Item {
        let mut found = Item::new(Case::Lower, Letter::A);
        self.first_compartment.iter().for_each(|f| {
            self.second_compartment.iter().for_each(|s| {
                if f == s {
                    found = *f;
                }
            });
        });

        found
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Item {
    case: Case,
    letter: Letter,
}

impl Item {
    pub fn new(case: Case, letter: Letter) -> Self {
        Item { case, letter }
    }

    pub fn prioraty(&self) -> u32 {
        match self.case {
            Case::Lower => {
                self.letter as u32
            },
            Case::Upper => {
                self.letter as u32 + Letter::Z as u32
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Case {
    Upper,
    Lower
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Letter {
    A =  1,
    B =  2,
    C =  3,
    D =  4,
    E =  5,
    F =  6,
    G =  7,
    H =  8,
    I =  9,
    J = 10,
    K = 11,
    L = 12,
    M = 13,
    N = 14,
    O = 15,
    P = 16,
    Q = 17,
    R = 18,
    S = 19,
    T = 20,
    U = 21,
    V = 22,
    W = 23,
    X = 24,
    Y = 25,
    Z = 26
}

impl TryFrom<u8> for Letter {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Letter::A as u8 => Ok(Letter::A),
            x if x == Letter::B as u8 => Ok(Letter::B),
            x if x == Letter::C as u8 => Ok(Letter::C),
            x if x == Letter::D as u8 => Ok(Letter::D),
            x if x == Letter::E as u8 => Ok(Letter::E),
            x if x == Letter::F as u8 => Ok(Letter::F),
            x if x == Letter::G as u8 => Ok(Letter::G),
            x if x == Letter::H as u8 => Ok(Letter::H),
            x if x == Letter::I as u8 => Ok(Letter::I),
            x if x == Letter::J as u8 => Ok(Letter::J),
            x if x == Letter::K as u8 => Ok(Letter::K),
            x if x == Letter::L as u8 => Ok(Letter::L),
            x if x == Letter::M as u8 => Ok(Letter::M),
            x if x == Letter::N as u8 => Ok(Letter::N),
            x if x == Letter::O as u8 => Ok(Letter::O),
            x if x == Letter::P as u8 => Ok(Letter::P),
            x if x == Letter::Q as u8 => Ok(Letter::Q),
            x if x == Letter::R as u8 => Ok(Letter::R),
            x if x == Letter::S as u8 => Ok(Letter::S),
            x if x == Letter::T as u8 => Ok(Letter::T),
            x if x == Letter::U as u8 => Ok(Letter::U),
            x if x == Letter::V as u8 => Ok(Letter::V),
            x if x == Letter::W as u8 => Ok(Letter::W),
            x if x == Letter::X as u8 => Ok(Letter::X),
            x if x == Letter::Y as u8 => Ok(Letter::Y),
            x if x == Letter::Z as u8 => Ok(Letter::Z),
            _ => Err(())
        }
    }
}
