use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|item| item.parse().unwrap()).collect())
        .collect()
}

fn elves_by_calories_desc(elves: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut sorted_elves = elves.to_vec();

    sorted_elves.sort_by_key(|elf| elf.iter().sum::<u32>());
    sorted_elves.reverse();

    sorted_elves
}

#[aoc(day1, part1)]
fn part1(elves: &[Vec<u32>]) -> u32 {
    let sorted_elves = elves_by_calories_desc(elves);

    sorted_elves.first().unwrap().iter().sum()
}

#[aoc(day1, part2)]
fn part2(elves: &[Vec<u32>]) -> u32 {
    let sorted_elves = elves_by_calories_desc(elves);

    sorted_elves
        .iter()
        .take(3)
        .map(|elf| elf.iter().sum::<u32>())
        .sum()
}
