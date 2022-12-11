use std::{path::Path, fs};

fn main() {
    part_one();
}

fn read_input() -> String {
    let path = Path::new("res/input");
    fs::read_to_string(path).expect("input file needed")
}

fn input_to_forest(input: &str) -> Forest {
    let lines = input.split("\n").filter(|line| !line.is_empty());
    let grid = lines.map(|line| line_to_tree_line(line)).collect();
    Forest::new(grid)
}

fn line_to_tree_line(line: &str) -> Vec<Tree> {
    line.chars().map(|c| Tree::new(c.to_digit(10).unwrap() as u8)).collect()
}

fn part_one() {
    let input = read_input();
    let forest = input_to_forest(&input);
    let visible = forest.calculate_visible();

    println!("Visible trees: {visible}");
}

#[derive(Default)]
struct Forest {
    grid: Vec<Vec<Tree>>
}

impl Forest {
    pub fn new(grid: Vec<Vec<Tree>>) -> Self {
        Self { grid }
    }

    pub fn calculate_visible(&self) -> usize {
        self.outline() + self.internal()
    }

    fn outline(&self) -> usize {
        (self.grid.len() * 2) + ((self.grid[0].len() - 2) * 2)
    }

    fn internal(&self) -> usize {
        let mut counter = 0;
        for y in 1..self.grid.len() - 1 {
            for x in 1..self.grid[y].len() - 1 {
                if self.vertical(y, x) || self.horizontal(y, x) {
                    counter += 1;
                }
            }
        }

        counter
    }

    fn vertical(&self, row: usize, column: usize) -> bool {
        let tree_height = self.grid[row][column].height;
        dbg!(tree_height);

        for y in 0..row {
            dbg!(y);
            dbg!(self.grid[y][column].height);
            if self.grid[y][column].height >= tree_height {
                return false
            }
        }

        for y in row+1..self.grid.len() {
            dbg!(y);
            dbg!(self.grid[y][column].height);
            if self.grid[y][column].height >= tree_height {
                return false
            }
        }

        true
    }

    fn horizontal(&self, row: usize, column: usize) -> bool {
        let tree_height = self.grid[row][column].height;

        for x in 0..column {
            if self.grid[row][x].height >= tree_height {
                return false
            }
        }

        for x in column+1..self.grid[row].len() {
            if self.grid[row][x].height >= tree_height {
                return false
            }
        }

        true
    }
}

struct Tree {
    height: u8,
}

impl Tree {
    pub fn new(height: u8) -> Self {
        Self { height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = r#"30373
25512
65332
33549
35390
"#;

    #[test]
    fn part_one_test() {
        let forest = input_to_forest(&INPUT_TEXT);
        let visible = forest.calculate_visible();

        assert_eq!(visible, 21);
    }
}
