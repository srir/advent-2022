use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Instr {
    AddX(isize),
    Noop,
}
use Instr::*;

impl FromStr for Instr {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let maybe_split = s.split_once(' ');

        match (maybe_split, s) {
            (Some(("addx", inc)), _) => Ok(AddX(inc.parse().unwrap())),
            (None, "noop") => Ok(Noop),
            _ => Err("could not parse"),
        }
    }
}

#[derive(Debug, Clone)]
struct Program {
    instructions: Vec<Instr>,
    pc: usize,
    cycle: usize,
    x: isize,
    display: Vec<Vec<bool>>,
}

impl Program {
    fn new(instructions: Vec<Instr>) -> Self {
        Program {
            instructions,
            pc: 0,
            cycle: 0,
            x: 1,
            display: vec![(); 6].iter().map(|_| vec![false; 40]).collect_vec(),
        }
    }

    fn draw_pixel(&mut self) {
        let pos = self.cycle - 1;
        let row = pos / 40;
        let col = pos % 40;

        if (col as isize) >= self.x - 1 && (col as isize) <= self.x + 1 {
            self.display[row][col] = true;
        }
    }

    fn run_calculating_signal_strengths(&mut self, cycles: &[usize]) -> isize {
        let important_cycles = HashSet::<usize>::from_iter(cycles.iter().cloned());
        let mut strength_map = HashMap::<usize, isize>::new();
        let mut pending_action: Option<Instr> = None;

        while self.pc < self.instructions.len() {
            self.cycle += 1;

            if important_cycles.contains(&self.cycle) {
                strength_map.insert(self.cycle, self.x * (self.cycle as isize));
            }

            self.draw_pixel();

            if let Some(AddX(v)) = pending_action {
                self.x += v;
                pending_action = None;
            } else {
                let instr = &self.instructions[self.pc];

                match instr {
                    Noop => {}
                    addx => {
                        pending_action = Some(addx.clone());
                    }
                }
                self.pc += 1;
            }
        }

        strength_map.values().sum()
    }

    fn render_display(&self) -> String {
        self.display
            .iter()
            .map(|row| row.iter().map(|b| if *b { '#' } else { '.' }).join(""))
            .join("\n")
    }
}

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Program {
    Program::new(input.lines().map(|l| l.parse().unwrap()).collect_vec())
}

#[aoc(day10, part1)]
fn part1(program: &Program) -> isize {
    let mut program = program.clone();
    let cycles = vec![20usize, 60, 100, 140, 180, 220];

    program.run_calculating_signal_strengths(&cycles[..])
}
#[aoc(day10, part2)]
fn part2(program: &Program) -> isize {
    let mut program = program.clone();
    let cycles = vec![];

    program.run_calculating_signal_strengths(&cycles[..]);

    let output = program.render_display();
    println!("{}\n", output);

    0
}
