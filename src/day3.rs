use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

fn elem_priority(c: &char) -> u32 {
    if c.is_ascii_lowercase() {
        *c as u32 - 96
    } else {
        *c as u32 - 64 + 26
    }
}

struct Rucksack(HashSet<char>, HashSet<char>);

impl FromStr for Rucksack {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s1, s2) = s.split_at(s.len() / 2);

        Ok(Rucksack(s1.chars().collect(), s2.chars().collect()))
    }
}

impl Rucksack {
    fn priority_shared_elem(&self) -> Option<u32> {
        let Rucksack(l, r) = self;

        let shared = l.intersection(r).next()?;

        Some(elem_priority(shared))
    }
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let rucksacks = input
        .lines()
        .map(|line| line.parse::<Rucksack>().unwrap())
        .collect::<Vec<Rucksack>>();

    rucksacks
        .iter()
        .map(|r| r.priority_shared_elem().unwrap_or(0))
        .sum()
}

struct ElfGroup(HashSet<char>, HashSet<char>, HashSet<char>);

impl FromStr for ElfGroup {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elves = s.lines().collect::<Vec<_>>();

        Ok(ElfGroup(
            elves[0].chars().collect(),
            elves[1].chars().collect(),
            elves[2].chars().collect(),
        ))
    }
}

impl ElfGroup {
    fn priority_shared_elem(&self) -> Option<u32> {
        let ElfGroup(f, s, t) = self;

        let first_two = f.intersection(s).cloned().collect::<HashSet<_>>();

        let shared = first_two.intersection(t).next()?;

        Some(elem_priority(shared))
    }
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let chunks = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|c| c.collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let elf_groups = chunks
        .iter()
        .map(|c| c.join("\n").parse::<ElfGroup>().unwrap())
        .collect::<Vec<_>>();

    elf_groups
        .iter()
        .map(|e| e.priority_shared_elem().unwrap_or(0))
        .sum()
}
