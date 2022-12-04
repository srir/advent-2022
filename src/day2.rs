use aoc_runner_derive::aoc;
use std::str::FromStr;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}
use Shape::*;

impl FromStr for Shape {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err("Could not parse"),
        }
    }
}

impl Shape {
    fn opponent_for_outcome(&self, outcome: &Outcome) -> Shape {
        match (outcome, self) {
            (Draw, _) => self.clone(),
            (Lose, Rock) => Scissors,
            (Lose, Paper) => Rock,
            (Lose, Scissors) => Paper,
            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win,
}
use Outcome::*;

impl FromStr for Outcome {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Lose),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err("Could not parse"),
        }
    }
}

struct Round(Shape, Shape);

impl FromStr for Round {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent, mine) = s.split_once(' ').ok_or("Could not parse")?;

        Ok(Round(opponent.parse()?, mine.parse()?))
    }
}

impl Round {
    fn score(&self) -> u32 {
        let Round(opponent_move, my_move) = self;

        let move_score = match my_move {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };

        let outcome_score = match (opponent_move, my_move) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => 0,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
            (Scissors, Rock) | (Paper, Scissors) | (Rock, Paper) => 6,
        };

        move_score + outcome_score
    }
}

struct Round2(Shape, Outcome);

impl FromStr for Round2 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent, mine) = s.split_once(' ').ok_or("Could not parse")?;

        Ok(Round2(opponent.parse()?, mine.parse()?))
    }
}

impl Round2 {
    fn score(&self) -> u32 {
        let Round2(opponent_move, outcome) = self;

        let my_move = opponent_move.opponent_for_outcome(outcome);

        let move_score = match my_move {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };

        let outcome_score = match (opponent_move, my_move) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => 0,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
            (Scissors, Rock) | (Paper, Scissors) | (Rock, Paper) => 6,
        };

        move_score + outcome_score
    }
}

#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
    let rounds = input.lines().map(|l| l.parse::<Round>().unwrap());

    rounds.map(|round| round.score()).sum()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> u32 {
    let rounds = input.lines().map(|l| l.parse::<Round2>().unwrap());

    rounds.map(|round| round.score()).sum()
}
