use std::{path::Path, fs};

fn main() {
    part_one();
    part_two();
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

fn part_two() {
    let input = read_input();
    let forest = input_to_forest(&input);
    let scenic_score = forest.highest_scenic_score();

    println!("Highest scenic score: {scenic_score}");
}

#[derive(Default, Debug)]
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

    pub fn highest_scenic_score(&self) -> usize {
        let mut highest_score = 0;

        for y in 1..self.grid.len() - 1 {
            for x in 1..self.grid[y].len() - 1 {
                let score = self.calculate_score(y, x);

                if score > highest_score {
                    highest_score = score;
                }
            }
        }

        highest_score
    }

    fn calculate_score(&self, row: usize, column: usize) -> usize {
        self.up_view_distance(row, column)
            * self.down_view_distance(row, column)
            * self.left_view_distance(row, column)
            * self.right_view_distance(row, column)
    }

    fn outline(&self) -> usize {
        (self.grid.len() * 2) + ((self.grid[0].len() - 2) * 2)
    }

    fn internal(&self) -> usize {
        let mut counter = 0;
        for y in 1..self.grid.len() - 1 {
            for x in 1..self.grid[y].len() - 1 {
                if self.vertical_visible(y, x) || self.horizontal_visible(y, x) {
                    counter += 1;
                }
            }
        }

        counter
    }

    fn vertical_visible(&self, row: usize, column: usize) -> bool {
        self.check_above(row, column) || self.check_below(row, column)
    }

    fn check_above(&self, row: usize, column: usize) -> bool {
        let tree_height = self.grid[row][column].height;

        for y in 0..row {
            if self.grid[y][column].height >= tree_height {
                return false
            }
        }

        true
    }

    fn check_below(&self, row: usize, column: usize) -> bool {
        let tree_height = self.grid[row][column].height;

        for y in row+1..self.grid.len() {
            if self.grid[y][column].height >= tree_height {
                return false
            }
        }

        true
    }

    fn horizontal_visible(&self, row: usize, column: usize) -> bool {
        self.check_left(row, column) || self.check_right(row, column)
    }

    fn check_left(&self, row: usize, column: usize) -> bool {
        let tree_height = self.grid[row][column].height;

        for x in 0..column {
            if self.grid[row][x].height >= tree_height {
                return false
            }
        }

        true
    }

    fn check_right(&self, row: usize, column: usize) -> bool {
        let tree_height = self.grid[row][column].height;

        for x in column+1..self.grid[row].len() {
            if self.grid[row][x].height >= tree_height {
                return false
            }
        }

        true
    }

    fn up_view_distance(&self, row: usize, column: usize) -> usize {
        let tree_height = self.grid[row][column].height;

        for y in (0..row).rev() {
            if self.grid[y][column].height >= tree_height {
                return row - y
            }
        }

        row
    }

    fn down_view_distance(&self, row: usize, column: usize) -> usize {
        let tree_height = self.grid[row][column].height;

        for y in row+1..self.grid.len() {
            if self.grid[y][column].height >= tree_height {
                return y - row
            }
        }

        self.grid.len() - row - 1
    }

    fn left_view_distance(&self, row: usize, column: usize) -> usize {
        let tree_height = self.grid[row][column].height;

        for x in (0..column).rev() {
            if self.grid[row][x].height >= tree_height {
                return column - x
            }
        }

        column
    }

    fn right_view_distance(&self, row: usize, column: usize) -> usize {
        let tree_height = self.grid[row][column].height;

        for x in column+1..self.grid[row].len() {
            if self.grid[row][x].height >= tree_height {
                return x - column
            }
        }

        self.grid[row].len() - column - 1
    }
}

#[derive(Debug)]
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

    #[test]
    fn part_two_test() {
        let forest = input_to_forest(&INPUT_TEXT);
        let scenic_score = forest.highest_scenic_score();

        assert_eq!(scenic_score, 8);
    }
}
