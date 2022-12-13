use std::{path::Path, fs, process::exit, collections::VecDeque};

const CYCLES: [u64; 6] = [20, 60, 100, 140, 180, 220];

fn main() {
    part_one();
    part_two();
}

fn read_input() -> String {
    let path = Path::new("res/input");
    fs::read_to_string(path).expect("input file needed")
}

fn input_to_instructions(input: &str) -> VecDeque<Instruction> {
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

fn part_two() {
    let input = read_input();
    let instructions = input_to_instructions(&input);
    let mut communicator = Communicator::new(instructions);
    let screen_output = communicator.print_screen();

    println!("Screen output:\n{screen_output}");
}

fn signal_strength_sum(communicator: &mut Communicator, sum_cycles: &[u64]) -> i64 {
    let mut signal_sum = 0;

    loop {
        if sum_cycles.contains(&communicator.cycle) {
            signal_sum += communicator.signal_strength();
        }

        if !communicator.step() {
            break;
        }
    }

    signal_sum
}

#[derive(Debug, Clone, Copy)]
struct Addx {
    v: i64,
}

impl Addx {
    pub fn new(v: i64) -> Self {
        Addx { v }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(Addx),
}

impl Instruction {
    fn cycles(self) -> usize {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}

struct Communicator {
    x: i64,
    cycle: u64,
    instructions: VecDeque<Instruction>,
    current: Option<(Instruction, usize)>,
}

impl Communicator {
    pub fn new(instructions: VecDeque<Instruction>) -> Self {
        let mut comm = Communicator { x: 1, cycle: 1, instructions, current: None };
        comm.decode();
        comm
    }

    fn decode(&mut self) {
        self.current = self.instructions.pop_front().map(|ins| (ins, ins.cycles()));
    }

    fn step(&mut self) -> bool {
        if self.current.is_none() {
            return false;
        }

        let (ins, cycles_left) = self.current.as_mut().unwrap();
        *cycles_left -= 1;
        if *cycles_left == 0 {
            match ins {
                Instruction::Noop => {},
                Instruction::Addx(addx) => self.x += addx.v,
            }
            self.decode();
        }
        self.cycle += 1;
        true
    }

    pub fn signal_strength(&self) -> i64 {
        self.x * self.cycle as i64
    }

    pub fn print_screen(&mut self) -> String {
        let mut output = "".to_owned();

        loop {
            let mut pixel = self.get_pixel();

            if self.cycle % 40 == 0 {
                pixel += "\n";
            }

            if !self.step() {
                break;
            }

            output += &pixel;

        }

        output
    }

    fn get_pixel(&self) -> String {
        let x_pos = (self.cycle - 1) % 40;
        let sprite_pos = self.x-1..=self.x+1;

        if sprite_pos.contains(&(x_pos as i64)) {
            "#".to_owned()
        } else {
            ".".to_owned()
        }
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

    const PART_TWO_OUTPUT: &str = r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#;

    #[test]
    fn part_one_test() {
        let instructions = input_to_instructions(INPUT_TEXT);
        let mut communicator = Communicator::new(instructions);
        let signal_strength_sum = signal_strength_sum(&mut communicator, &CYCLES);

        assert_eq!(signal_strength_sum, 13140);
    }

    #[test]
    fn part_two_test() {
        let instructions = input_to_instructions(INPUT_TEXT);
        let mut communicator = Communicator::new(instructions);
        let screen_output = communicator.print_screen();

        assert_eq!(&screen_output, PART_TWO_OUTPUT);
    }
}
