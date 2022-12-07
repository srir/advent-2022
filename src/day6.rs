use aoc_runner_derive::aoc;
use std::collections::HashSet;

fn find_n_distinct(input: &str, n: usize) -> Option<usize> {
    for (starting_index, window) in input.as_bytes().windows(n).enumerate() {
        if HashSet::<u8>::from_iter(window.iter().cloned()).len() == n {
            return Some(starting_index + n);
        }
    }

    None
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    find_n_distinct(input, 4).unwrap()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    find_n_distinct(input, 14).unwrap()
}
