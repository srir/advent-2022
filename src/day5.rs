use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Crate {
    label: String,
}

#[derive(Debug, Copy, Clone)]
struct MoveCommand {
    num_crates: u32,
    from_stack: usize,
    to_stack: usize,
}

impl FromStr for MoveCommand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let toks = s.split_whitespace().collect::<Vec<_>>();

        Ok(MoveCommand {
            num_crates: toks[1].parse().unwrap(),
            from_stack: toks[3].parse::<usize>().unwrap() - 1,
            to_stack: toks[5].parse::<usize>().unwrap() - 1,
        })
    }
}

#[derive(Debug)]
struct State(Vec<Vec<Crate>>);

impl FromStr for State {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().rev().collect::<Vec<_>>();

        let num_stacks = lines[0]
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .max()
            .unwrap();

        let mut stacks = vec![(); num_stacks]
            .iter()
            .map(|_| Vec::new())
            .collect::<Vec<Vec<_>>>();

        for &line in &lines[1..] {
            let mut labels_idx = 0;
            let mut line_idx = 1;
            let mut labels: Vec<Option<Crate>> = vec![None; num_stacks];

            while line_idx < line.len() {
                let maybe_label = line.get(line_idx..line_idx + 1).unwrap();

                if maybe_label != " " {
                    labels[labels_idx] = Some(Crate {
                        label: maybe_label.to_string(),
                    })
                }

                labels_idx += 1;
                line_idx += 4;
            }

            for (i, label) in labels.iter().enumerate() {
                if let Some(c) = label {
                    stacks[i].push(c.clone());
                }
            }
        }

        Ok(State(stacks))
    }
}

impl State {
    fn get_message(&self) -> String {
        let State(crates) = self;

        crates
            .iter()
            .map(|stack| stack.last().unwrap().label.clone())
            .join("")
    }

    fn step(&mut self, command: &MoveCommand) {
        let State(crates) = self;
        let &MoveCommand {
            num_crates,
            from_stack,
            to_stack,
        } = command;

        for _ in 0..num_crates {
            let crate_ = crates[from_stack].pop().unwrap();
            crates[to_stack].push(crate_);
        }
    }

    fn step_many(&mut self, commands: &[MoveCommand]) {
        for command in commands {
            self.step(command);
        }
    }

    fn multi_step(&mut self, command: &MoveCommand) {
        let State(crates) = self;
        let &MoveCommand {
            num_crates,
            from_stack,
            to_stack,
        } = command;

        let mut moved_crates: Vec<Crate> = Vec::new();

        for _ in 0..num_crates {
            let crate_ = crates[from_stack].pop().unwrap();
            moved_crates.push(crate_);
        }

        moved_crates.reverse();

        for crate_ in moved_crates {
            crates[to_stack].push(crate_);
        }
    }

    fn multi_step_many(&mut self, commands: &[MoveCommand]) {
        for command in commands {
            self.multi_step(command);
        }
    }
}

#[aoc(day5, part1)]
fn part1(input: &str) -> String {
    let (init_state_str, commands_str) = input.split_once("\n\n").unwrap();

    let mut state = init_state_str.parse::<State>().unwrap();

    let commands = commands_str
        .lines()
        .map(|l| l.parse::<MoveCommand>().unwrap())
        .collect::<Vec<_>>();

    state.step_many(&commands);

    state.get_message()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> String {
    let (init_state_str, commands_str) = input.split_once("\n\n").unwrap();

    let mut state = init_state_str.parse::<State>().unwrap();

    let commands = commands_str
        .lines()
        .map(|l| l.parse::<MoveCommand>().unwrap())
        .collect::<Vec<_>>();

    state.multi_step_many(&commands);

    state.get_message()
}
