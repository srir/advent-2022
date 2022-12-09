use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug)]
struct FileTree(BTreeMap<PathBuf, usize>);

impl FileTree {
    fn total_size_below_limit(&self, limit_size: usize) -> usize {
        let FileTree(directories) = self;

        directories
            .iter()
            .filter_map(|(path, &size)| {
                if path.extension().is_some() || size > limit_size {
                    None
                } else {
                    Some(size)
                }
            })
            .sum()
    }

    fn smallest_size_above_limit(&self, total_needed_space: usize) -> Option<usize> {
        let FileTree(directories) = self;

        let used_space = *directories.get(&*PathBuf::from("/")).unwrap();
        let free_space = 70000000 - used_space;
        let additional_needed_space = total_needed_space - free_space;

        directories
            .iter()
            .filter_map(|(path, &size)| {
                if path.extension().is_some() || size < additional_needed_space {
                    None
                } else {
                    Some(size)
                }
            })
            .min()
    }
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> FileTree {
    let mut directories = BTreeMap::new();
    let mut current_dir = PathBuf::from("/");
    directories.insert(current_dir.clone(), 0);

    for line in input.lines().skip(1) {
        if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("$ cd /") {
            current_dir = PathBuf::from("/");
        } else if line.starts_with("$ cd ..") {
            if let Some(parent) = current_dir.parent() {
                current_dir = parent.to_path_buf();
            }
        } else if let Some(dir_name) = line.strip_prefix("$ cd ") {
            let dir_name = dir_name.to_string();
            let new_path = current_dir.join(dir_name);
            directories.insert(new_path.clone(), 0);
            current_dir = new_path;
        } else {
            let (type_or_size, name) = line.split_once(' ').unwrap();
            let new_path = current_dir.join(name);

            if type_or_size == "dir" {
                directories.insert(new_path, 0);
            } else {
                let size = type_or_size.parse::<usize>().unwrap();

                for ancestor in new_path.ancestors() {
                    if ancestor != new_path {
                        *directories.entry(ancestor.to_path_buf()).or_insert(0) += size;
                    }
                }
            }
        }
    }

    FileTree(directories)
}

#[aoc(day7, part1)]
fn part1(input: &FileTree) -> usize {
    input.total_size_below_limit(100000)
}

#[aoc(day7, part2)]
fn part2(input: &FileTree) -> usize {
    input.smallest_size_above_limit(30000000).unwrap()
}
