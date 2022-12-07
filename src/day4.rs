use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

struct IRange(u32, u32);

impl IRange {
    fn contains_range(&self, other: &IRange) -> bool {
        let IRange(my_l, my_r) = self;
        let IRange(other_l, other_r) = other;

        my_l <= other_l && other_r <= my_r
    }

    fn overlaps_range(&self, other: &IRange) -> bool {
        let IRange(my_l, my_r) = self;
        let IRange(other_l, other_r) = other;

        my_l <= other_r && my_r >= other_l
    }
}

struct ElfPair(IRange, IRange);

impl FromStr for ElfPair {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once(',').ok_or("Could not split")?;

        let (ll_s, lr_s) = l.split_once('-').ok_or("Could not split")?;
        let (rl_s, rr_s) = r.split_once('-').ok_or("Could not split")?;

        let (ll, lr) = (ll_s.parse().unwrap(), lr_s.parse().unwrap());
        let (rl, rr) = (rl_s.parse().unwrap(), rr_s.parse().unwrap());

        Ok(ElfPair(IRange(ll, lr), IRange(rl, rr)))
    }
}

impl ElfPair {
    fn is_subsumed(&self) -> bool {
        let ElfPair(l, r) = self;

        l.contains_range(r) || r.contains_range(l)
    }

    fn is_overlapping(&self) -> bool {
        let ElfPair(l, r) = self;

        l.overlaps_range(r) || r.overlaps_range(l)
    }
}

#[aoc_generator(day4)]
fn parse_ranges(input: &str) -> Vec<ElfPair> {
    input
        .lines()
        .map(|l| l.parse::<ElfPair>().unwrap())
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[ElfPair]) -> u32 {
    input.iter().filter(|ep| ep.is_subsumed()).count() as u32
}

#[aoc(day4, part2)]
fn part2(input: &[ElfPair]) -> u32 {
    input.iter().filter(|ep| ep.is_overlapping()).count() as u32
}
