use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
use Dir::*;

#[derive(Debug)]
struct Move(Dir, usize);

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s.split_once(' ').ok_or("Could not split")?;

        let dist = dist.parse::<usize>().map_err(|_| "Could not parse dist")?;

        let dir = match dir {
            "U" => Ok(Up),
            "D" => Ok(Down),
            "L" => Ok(Left),
            "R" => Ok(Right),
            _ => Err("Could not parse dir"),
        }?;

        Ok(Move(dir, dist))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coords {
    x: isize,
    y: isize,
}

impl Coords {
    fn new(x: isize, y: isize) -> Self {
        Coords { x, y }
    }
}

#[derive(Debug)]
struct Rope {
    moves: Vec<Move>,
    visited_coords: HashSet<Coords>,
    next_step: usize,
    curr_head: Coords,
    curr_tail: Coords,
}

impl Rope {
    fn new(moves: Vec<Move>) -> Self {
        let curr_tail = Coords::new(0, 0);
        Rope {
            moves,
            visited_coords: HashSet::from([curr_tail]),
            next_step: 0,
            curr_head: Coords::new(0, 0),
            curr_tail,
        }
    }

    fn step(&mut self) {
        let Move(dir, num_steps) = &self.moves[self.next_step];

        for _ in 0..*num_steps {
            match dir {
                Up => {
                    self.curr_head.y += 1;
                }
                Down => {
                    self.curr_head.y -= 1;
                }
                Left => {
                    self.curr_head.x -= 1;
                }
                Right => {
                    self.curr_head.x += 1;
                }
            };

            let x_diff = self.curr_head.x.abs_diff(self.curr_tail.x);
            let y_diff = self.curr_head.y.abs_diff(self.curr_tail.y);

            if y_diff == 0 && x_diff >= 2 {
                self.curr_tail.x += if self.curr_head.x > self.curr_tail.x {
                    1
                } else {
                    -1
                };
            } else if x_diff == 0 && y_diff >= 2 {
                self.curr_tail.y += if self.curr_head.y > self.curr_tail.y {
                    1
                } else {
                    -1
                };
            } else if (x_diff >= 1 && y_diff >= 2) || (y_diff >= 1 && x_diff >= 2) {
                self.curr_tail.x += if self.curr_head.x > self.curr_tail.x {
                    1
                } else {
                    -1
                };
                self.curr_tail.y += if self.curr_head.y > self.curr_tail.y {
                    1
                } else {
                    -1
                };
            }

            self.visited_coords.insert(self.curr_tail);
        }

        self.next_step += 1;
    }

    fn step_all(&mut self) {
        while self.next_step < self.moves.len() {
            self.step();
        }
    }

    fn count_visited_coords(&self) -> usize {
        self.visited_coords.len()
    }
}

fn parse_input(input: &str) -> Rope {
    Rope::new(input.lines().map(|l| l.parse().unwrap()).collect_vec())
}

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    let mut rope = parse_input(input);

    rope.step_all();
    rope.count_visited_coords()
}

struct LongRope {
    moves: Vec<Move>,
    visited_coords: HashSet<Coords>,
    next_step: usize,
    knots: Vec<Coords>,
}

impl LongRope {
    fn new(moves: Vec<Move>) -> Self {
        let curr_loc = Coords::new(0, 0);
        LongRope {
            moves,
            visited_coords: HashSet::from([curr_loc]),
            next_step: 0,
            knots: (0..=9).map(|_| curr_loc.clone()).collect_vec(),
        }
    }

    fn step(&mut self) {
        let Move(dir, num_steps) = &self.moves[self.next_step];

        for _ in 0..*num_steps {
            match dir {
                Up => {
                    self.knots[0].y += 1;
                }
                Down => {
                    self.knots[0].y -= 1;
                }
                Left => {
                    self.knots[0].x -= 1;
                }
                Right => {
                    self.knots[0].x += 1;
                }
            };

            for i in 1..=9 {
                let x_diff = self.knots[i - 1].x.abs_diff(self.knots[i].x);
                let y_diff = self.knots[i - 1].y.abs_diff(self.knots[i].y);

                if y_diff == 0 && x_diff >= 2 {
                    self.knots[i].x += if self.knots[i - 1].x > self.knots[i].x {
                        1
                    } else {
                        -1
                    };
                } else if x_diff == 0 && y_diff >= 2 {
                    self.knots[i].y += if self.knots[i - 1].y > self.knots[i].y {
                        1
                    } else {
                        -1
                    };
                } else if (x_diff >= 1 && y_diff >= 2) || (y_diff >= 1 && x_diff >= 2) {
                    self.knots[i].x += if self.knots[i - 1].x > self.knots[i].x {
                        1
                    } else {
                        -1
                    };
                    self.knots[i].y += if self.knots[i - 1].y > self.knots[i].y {
                        1
                    } else {
                        -1
                    };
                }
            }

            self.visited_coords.insert(self.knots[9]);
        }

        self.next_step += 1;
    }

    fn step_all(&mut self) {
        while self.next_step < self.moves.len() {
            self.step();
        }
    }

    fn count_visited_coords(&self) -> usize {
        self.visited_coords.len()
    }
}

fn parse_input_2(input: &str) -> LongRope {
    LongRope::new(input.lines().map(|l| l.parse().unwrap()).collect_vec())
}

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    let mut long_rope = parse_input_2(input);

    long_rope.step_all();
    long_rope.count_visited_coords()
}
