use std::{path::Path, fs, process::exit};

const CYCLES: [u64; 6] = [20, 60, 100, 140, 180, 220];

fn main() {
    part_one();
}

fn read_input() -> String {
    let path = Path::new("res/input");
    fs::read_to_string(path).expect("input file needed")
}

fn input_to_instructions(input: &str) -> Vec<Instruction> {
    let lines = input.split("\n").filter(|line| !line.is_empty());
    lines.map(|line| line_to_instruction(line)).collect()
}

fn line_to_instruction(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split(" ").collect();

    match parts[0] {
        "noop" => Instruction::Noop,
        "addx" => Instruction::Addx(Addx::new(parts[1].parse().unwrap())),
        _ => exit(1),
    }
}

fn part_one() {
    let input = read_input();
    let instructions = input_to_instructions(&input);
    let mut communicator = Communicator::new(instructions);
    let signal_strength_sum = signal_strength_sum(&mut communicator, &CYCLES);

    println!("Signal strength sum: {signal_strength_sum}");
}

fn signal_strength_sum(communicator: &mut Communicator, sum_cycles: &[u64]) -> i64 {
    let mut signal_sum = 0;

    while communicator.instruction_counter < communicator.instructions.len() {
        if sum_cycles.contains(&(communicator.cycle + 1)) {
            signal_sum += communicator.signal_strength();
            dbg!(communicator.x);
            dbg!(communicator.cycle);
            dbg!(communicator.signal_strength());
            dbg!(signal_sum);
        }
        communicator.process_instruction();
    }

    signal_sum
}

#[derive(Debug)]
struct Addx {
    v: i64,
}

impl Addx {
    pub fn new(v: i64) -> Self {
        Addx { v }
    }
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(Addx),
}

struct Communicator {
    x: i64,
    cycle: u64,
    instruction_counter: usize,
    instruction_cycle: usize,
    instructions: Vec<Instruction>,
}

impl Communicator {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Communicator { x: 1, cycle: 0, instruction_counter: 0, instruction_cycle: 0, instructions }
    }

    pub fn process_instruction(&mut self) {
        let current_instruction = &self.instructions[self.instruction_counter];

        self.instruction_cycle += 1;
        dbg!(current_instruction);

        match current_instruction {
            Instruction::Noop => {
                if self.instruction_cycle >= 1 {
                    self.instruction_cycle = 0;
                }
            },
            Instruction::Addx(addx) => {
                if self.instruction_cycle >= 2 {
                    self.instruction_cycle = 0;
                    self.x += addx.v;
                }
            }
        }

        self.cycle += 1;
        if self.instruction_cycle == 0 {
            self.instruction_counter += 1;
        }
    }

    pub fn signal_strength(&self) -> i64 {
        self.x * (self.cycle + 1) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#;

    #[test]
    fn part_one_test() {
        let instructions = input_to_instructions(INPUT_TEXT);
        let mut communicator = Communicator::new(instructions);
        let signal_strength_sum = signal_strength_sum(&mut communicator, &CYCLES);

        assert_eq!(signal_strength_sum, 13140);
    }
}
