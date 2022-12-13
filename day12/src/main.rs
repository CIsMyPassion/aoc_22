use std::{path::Path, fs, str::FromStr, process::exit, collections::{HashMap, HashSet, VecDeque}};

fn main() {
    part_one();
    part_two();
}

fn read_input() -> String {
    let path = Path::new("res/input");
    fs::read_to_string(path).expect("input file needed")
}

fn part_one() {
    let input = read_input();
    let map = input.parse::<Map>().unwrap();
    let shortest_path = map.find_shortest_path();

    println!("Shortest path length: {}", shortest_path.len() - 1);
}

fn part_two() {
    let input = read_input();
    let map = input.parse::<Map>().unwrap();
    let shortest_path = map.find_shortest_path_to_a();

    println!("Shortest path to a length: {}", shortest_path.len() - 1);
}

struct Map {
    grid: Vec<Vec<char>>,
    start: (usize, usize),
    target: (usize, usize),
}

impl Map {
    pub fn find_shortest_path(&self) -> Vec<(usize, usize)> {
        let mut previous_map: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut distance_map: HashMap<(usize, usize), usize> = HashMap::new();
        let mut unvisited_set: HashSet<(usize, usize)> = HashSet::new();
        let mut open_set: VecDeque<(usize, usize)> = VecDeque::new();

        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                unvisited_set.insert((x, y));
            }
        }

        let mut current_position = self.start;
        unvisited_set.remove(&current_position);
        distance_map.insert(current_position, 0);

        while current_position != self.target {
            let neighbours = self.find_fitting_neighbours(&current_position);
            for neighbour in neighbours {
                if unvisited_set.contains(&neighbour) {
                    unvisited_set.remove(&neighbour);
                    previous_map.insert(neighbour, current_position);
                    distance_map.insert(neighbour, distance_map.get(&current_position).unwrap() + 1);
                    open_set.push_back(neighbour);
                }
            }

            if previous_map.contains_key(&self.target) {
                break;
            }

            current_position = open_set.pop_front().unwrap();
        }

        self.construct_path(previous_map)
    }

    pub fn find_shortest_path_to_a(&self) -> Vec<(usize, usize)> {
        let mut previous_map: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut distance_map: HashMap<(usize, usize), usize> = HashMap::new();
        let mut unvisited_set: HashSet<(usize, usize)> = HashSet::new();
        let mut open_set: VecDeque<(usize, usize)> = VecDeque::new();

        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                unvisited_set.insert((x, y));
            }
        }

        let mut current_position = self.target;
        unvisited_set.remove(&current_position);
        distance_map.insert(current_position, 0);
        let mut shortest_a = None;

        loop {
            let neighbours = self.find_fitting_neighbours_rev(&current_position);
            for neighbour in neighbours {
                if unvisited_set.contains(&neighbour) {
                    unvisited_set.remove(&neighbour);
                    previous_map.insert(neighbour, current_position);
                    distance_map.insert(neighbour, distance_map.get(&current_position).unwrap() + 1);
                    open_set.push_back(neighbour);

                    if self.grid[neighbour.1][neighbour.0] == 'a' {
                        shortest_a = Some(neighbour);
                        break;
                    }
                }
            }

            if shortest_a.is_some() {
                break;
            }

            current_position = open_set.pop_front().unwrap();
        }

        self.construct_path_rev(previous_map, shortest_a.unwrap())
    }

    fn find_fitting_neighbours(&self, current_position: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        let current_height = self.grid[current_position.1][current_position.0];

        for y in (current_position.1 as i64)-1..=(current_position.1 as i64)+1 {
            if y >= 0 && y < self.grid.len() as i64 {
                let height = self.grid[y as usize][current_position.0];
                if (height as u8) - 1 <= current_height as u8 {
                    neighbours.push((current_position.0, y as usize));
                }
            }
        }

        for x in (current_position.0 as i64)-1..=(current_position.0 as i64)+1 {
            if x >= 0 && x < self.grid[current_position.1].len() as i64 {
                let height = self.grid[current_position.1][x as usize];
                if (height as u8) - 1 <= current_height as u8 {
                    neighbours.push((x as usize, current_position.1));
                }
            }
        }

        neighbours
    }

    fn find_fitting_neighbours_rev(&self, current_position: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        let current_height = self.grid[current_position.1][current_position.0];

        for y in (current_position.1 as i64)-1..=(current_position.1 as i64)+1 {
            if y >= 0 && y < self.grid.len() as i64 {
                let height = self.grid[y as usize][current_position.0];
                if height as u8 >= (current_height as u8) - 1 {
                    neighbours.push((current_position.0, y as usize));
                }
            }
        }

        for x in (current_position.0 as i64)-1..=(current_position.0 as i64)+1 {
            if x >= 0 && x < self.grid[current_position.1].len() as i64 {
                let height = self.grid[current_position.1][x as usize];
                if height as u8 >= (current_height as u8) - 1 {
                    neighbours.push((x as usize, current_position.1));
                }
            }
        }

        neighbours
    }

    fn construct_path(&self, previous_map: HashMap<(usize, usize), (usize, usize)>) -> Vec<(usize, usize)> {
        let mut path = Vec::from([self.target]);

        let mut previous = self.target;

        while previous_map.get(&previous).is_some() {
            let current = previous_map.get(&previous).unwrap();
            path.push(*current);
            previous = *current;
        }

        path.reverse();
        path
    }

    fn construct_path_rev(&self, previous_map: HashMap<(usize, usize), (usize, usize)>, start_point: (usize, usize)) -> Vec<(usize, usize)> {
        let mut path = Vec::from([start_point]);

        let mut previous = start_point;

        while previous_map.get(&previous).is_some() {
            let current = previous_map.get(&previous).unwrap();
            path.push(*current);
            previous = *current;
        }

        path
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split("\n").filter(|line| !line.is_empty());
        let mut start = (0, 0);
        let mut target = (1, 1);
        let grid = lines.enumerate().map(|(y, line)| line.chars().enumerate().map(|(x, char)| {
            if char.is_uppercase() {
                match char {
                    'S' => {
                        start = (x, y);
                        'a'
                    },
                    'E' => {
                        target = (x, y);
                        'z'
                    },
                    _ => exit(1),
                }
            } else {
                char
            }
        }).collect()).collect();

        Ok(Self { grid, start, target })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

    #[test]
    fn parse_test() {
        let map = INPUT.parse::<Map>().unwrap();
        assert_eq!(map.start, (0, 0));
        assert_eq!(map.target, (5, 2));
    }

    #[test]
    fn part_one_test() {
        let map = INPUT.parse::<Map>().unwrap();
        let shortest_path = map.find_shortest_path();
        assert_eq!(shortest_path.len() - 1, 31);
    }

    #[test]
    fn part_two_test() {
        let map = INPUT.parse::<Map>().unwrap();
        let shortest_path = map.find_shortest_path_to_a();
        assert_eq!(shortest_path.len() - 1, 29);
    }
}
