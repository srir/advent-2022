use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
struct Grid {
    input: Vec<Vec<u8>>,
    visible_trees: Vec<Vec<bool>>,
}

impl FromStr for Grid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect_vec()
            })
            .collect::<Vec<_>>();
        Ok(Grid::new(&input[..]))
    }
}

impl Grid {
    fn new(input: &[Vec<u8>]) -> Self {
        let visible_trees = input
            .iter()
            .map(|row| row.iter().map(|_| false).collect())
            .collect();

        let mut grid = Self {
            input: input.to_vec(),
            visible_trees,
        };

        grid._mark_visible_from_top();
        grid._mark_visible_from_bottom();
        grid._mark_visible_from_left();
        grid._mark_visible_from_right();

        grid
    }

    fn _mark_visible_from_top(&mut self) {
        let mut max_in_col = vec![0; self.input[0].len()];

        for (y, row) in self.input.iter().enumerate() {
            for (x, &tree) in row.iter().enumerate() {
                if max_in_col[x] < tree {
                    self.visible_trees[y][x] = true;
                }

                if tree > max_in_col[x] {
                    max_in_col[x] = tree;
                }
            }
        }
    }

    fn _mark_visible_from_bottom(&mut self) {
        let mut max_in_col = vec![0; self.input[0].len()];

        for (y, row) in self.input.iter().enumerate().rev() {
            for (x, &tree) in row.iter().enumerate() {
                if max_in_col[x] < tree {
                    self.visible_trees[y][x] = true;
                }

                if tree > max_in_col[x] {
                    max_in_col[x] = tree;
                }
            }
        }
    }

    fn _mark_visible_from_left(&mut self) {
        let num_rows = self.input.len();
        let num_cols = self.input[0].len();
        let mut max_in_row = vec![0; num_rows];

        for x in 0..num_cols {
            for y in 0..num_rows {
                let tree = self.input[y][x];
                if max_in_row[y] < tree {
                    self.visible_trees[y][x] = true;
                }

                if tree > max_in_row[y] {
                    max_in_row[y] = tree;
                }
            }
        }
    }

    fn _mark_visible_from_right(&mut self) {
        let num_rows = self.input.len();
        let num_cols = self.input[0].len();
        let mut max_in_row = vec![0; num_rows];

        for x in (0..num_cols).rev() {
            for y in 0..num_rows {
                let tree = self.input[y][x];
                if max_in_row[y] < tree {
                    self.visible_trees[y][x] = true;
                }

                if tree > max_in_row[y] {
                    max_in_row[y] = tree;
                }
            }
        }
    }

    fn count_visible_trees(&self) -> usize {
        self.visible_trees
            .iter()
            .map(|row| row.iter().filter(|&&c| c).count())
            .sum()
    }

    fn calculate_score(&self, x0: usize, y0: usize) -> usize {
        let num_rows = self.input.len();
        let num_cols = self.input[0].len();
        let tree = self.input[y0][x0];
        let mut score_up = 0;
        let mut score_down = 0;
        let mut score_left = 0;
        let mut score_right = 0;

        for y in (0..y0).rev() {
            score_up += 1;
            if self.input[y][x0] >= tree {
                break;
            }
        }

        for y in (y0 + 1)..num_rows {
            score_down += 1;
            if self.input[y][x0] >= tree {
                break;
            }
        }

        for x in (0..x0).rev() {
            score_left += 1;
            if self.input[y0][x] >= tree {
                break;
            }
        }

        for x in (x0 + 1)..num_cols {
            score_right += 1;
            if self.input[y0][x] >= tree {
                break;
            }
        }

        score_up * score_down * score_left * score_right
    }

    fn calculate_max_score(&self) -> usize {
        let num_rows = self.input.len();
        let num_cols = self.input[0].len();

        let mut max_score = 0usize;

        for y in 0..num_rows {
            for x in 0..num_cols {
                max_score = max_score.max(self.calculate_score(x, y))
            }
        }

        max_score
    }
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let grid = input.parse::<Grid>().unwrap();

    grid.count_visible_trees()
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    let grid = input.parse::<Grid>().unwrap();

    grid.calculate_max_score()
}
