use aoc_runner_derive::aoc;
use std::borrow::BorrowMut;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
enum Node {
    File {
        name: String,
        path: String,
        size: usize,
    },
    Directory {
        name: String,
        path: String,
        contents: BTreeMap<String, Node>,
    },
}
use Node::*;

#[derive(Debug)]
struct Tree {
    root: Node,
}

#[derive(Debug, PartialEq, Eq)]
enum ParseMode {
    Command,
    Output,
}
use ParseMode::*;

fn parse_input(input: &str) -> Tree {
    let mut mode = Command;
    let mut nodes = Vec::<Node>::new();
    let mut parents = BTreeMap::<String, &Node>::new();

    let root = Directory {
        name: "/".to_string(),
        path: "/".to_string(),
        contents: BTreeMap::new(),
    };
    nodes.push(root);

    let mut tree = Tree { root };
    let mut current_node = &tree.root;

    for line in input.lines().skip(1) {
        println!("xcxc line {}", line);
        println!("xcxc current_node {:?}", current_node);

        if line.starts_with("$ ls") {
            mode = Output;
        } else if line.starts_with("$ cd /") {
            mode = Command;
            current_node = &tree.root;
        } else if line.starts_with("$ cd ..") {
            mode = Command;

            println!("xcxc current_node {:?}", current_node);

            current_node = match current_node {
                File { path, .. } => parents.get(path).unwrap(),
                Directory { path, .. } => parents.get(path).unwrap(),
            }
        } else if let Some(dir_name) = line.strip_prefix("$ cd ") {
            mode = Command;
            if let Directory { mut contents, .. } = current_node.borrow_mut() {
                if let Directory {
                    path: curr_path, ..
                } = current_node.clone()
                {
                    let dir_name = dir_name.to_string();
                    let new_path = curr_path + "/" + dir_name.as_str();
                    let new_node = Directory {
                        name: dir_name.clone(),
                        path: new_path.clone(),
                        contents: BTreeMap::new(),
                    };
                    contents.insert(dir_name.clone(), new_node);

                    parents.insert(new_path, &new_node);
                } else {
                    panic!("unexpected");
                }
            } else {
                panic!("Unexpected cd while current node was a file");
            }
        } else if let Directory { mut contents, .. } = current_node.borrow_mut() {
            assert_eq!(mode, Output);
            if let Directory {
                path: curr_path, ..
            } = current_node.clone()
            {
                let curr_path = curr_path.to_string();
                let (type_or_size, name) = line.split_once(' ').unwrap();

                if type_or_size == "dir" {
                    contents.insert(
                        name.to_string(),
                        Directory {
                            name: name.to_string(),
                            path: curr_path + "/" + name,
                            contents: BTreeMap::new(),
                        },
                    );
                } else {
                    let size = type_or_size.parse::<usize>().unwrap();

                    contents.insert(
                        name.to_string(),
                        File {
                            name: name.to_string(),
                            path: curr_path + "/" + name,
                            size,
                        },
                    );
                }
            }
        } else {
            panic!("Unexpected output mode when not in a directory")
        }
    }

    tree
}

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let tree = parse_input(input);

    0
}
