use core::fmt;
use std::{path::Path, fs, process::exit, collections::HashMap, rc::Rc, cell::RefCell};

fn main() {
    part_one();
}

fn read_input() -> String {
    let path = Path::new("res/input");
    fs::read_to_string(path).expect("input file needed")
}

fn convert_input(input: &str) -> Vec<InputLine> {
    input.split("\n").filter(|line| !line.is_empty()).map(|line| line_to_input(line)).collect()
}

fn construct_nodes(input: &Vec<InputLine>) -> NodeHandle {
    let root = Rc::new(RefCell::new(Node::default()));
    let mut node = root.clone();

    for line in input {
        match line {
            InputLine::Command(cmd) => match cmd {
                Command::Ls => {

                },
                Command::Cd(path) => match path.0.as_str() {
                    "/" => {

                    },
                    ".." => {
                        let parent = node.borrow().parent.clone().unwrap();
                        node = parent;
                    },
                    _ => {
                        let child = node.borrow_mut().children.entry(path.0.to_owned()).or_default().clone();
                        node = child;
                    }
                },
            },
            InputLine::Entry(entry) => match entry {
                Entry::Dir(dir) => {
                    let entry = node.borrow_mut().children.entry(dir.0.to_owned()).or_default().clone();
                    entry.borrow_mut().parent = Some(node.clone());
                },
                Entry::File(file) => {
                    let entry = node.borrow_mut().children.entry(file.0.to_owned()).or_default().clone();
                    entry.borrow_mut().size = file.1;
                    entry.borrow_mut().parent = Some(node.clone());
                }
            }
        }
    }

    root
}

fn dir_sum_under_limit(root: NodeHandle, limit: usize) -> usize {
    all_dirs(root).map(|d| d.borrow().total_size()).filter(|&s| s <= limit).sum::<usize>()
}

fn part_one() {
    let input_lines = convert_input(&read_input());
    let root = construct_nodes(&input_lines);
    let sum = dir_sum_under_limit(root, 100000);

    println!("Sum: {sum}");
}

fn line_to_input(line: &str) -> InputLine {
    let parts: Vec<_> = line.split_whitespace().collect();

    match parts[..] {
        ["$", "cd", name] => InputLine::Command(Command::Cd(Cd(name.to_owned()))),
        ["$", "ls"] => InputLine::Command(Command::Ls),
        ["dir", name] => InputLine::Entry(Entry::Dir(Dir(name.to_owned()))),
        [size, name] => InputLine::Entry(Entry::File(File(name.to_owned(), size.parse().unwrap()))),
        _ => exit(1),
    }
}

fn all_dirs(n: NodeHandle) -> Box<dyn Iterator<Item = NodeHandle>> {
    let children: Vec<_> = n.borrow().children.values().cloned().collect();

    Box::new(
        std::iter::once(n).chain(children.into_iter().filter_map(|c| {
            if c.borrow().is_dir() {
                Some(all_dirs(c))
            } else {
                None
            }
        })
        .flatten(),
        ),
    )
}

#[derive(Debug)]
enum InputLine {
    Command(Command),
    Entry(Entry),
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Cd),
}

#[derive(Debug)]
struct Cd (String);

#[derive(Debug)]
enum Entry {
    Dir(Dir),
    File(File),
}

#[derive(Debug)]
struct Dir (String);

#[derive(Debug)]
struct File (String, usize);

type NodeHandle = Rc<RefCell<Node>>;

#[derive(Default)]
struct Node {
    size: usize,
    children: HashMap<String, NodeHandle>,
    parent: Option<NodeHandle>,
}

impl Node {
    fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }

    fn total_size(&self) -> usize {
        self.children.values().map(|child| child.borrow().total_size()).sum::<usize>() + self.size
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("size", &self.size)
            .field("children", &self.children)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn part_one_test() {
        let input_lines = convert_input(&TEST_INPUT);
        let root = construct_nodes(&input_lines);
        let sum = dir_sum_under_limit(root, 100000);

        assert_eq!(sum, 95437);
    }
}
