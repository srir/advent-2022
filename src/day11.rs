use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u128>,
    operation: fn(u128) -> u128,
    test: fn(u128) -> bool,
    true_monkey: usize,
    false_monkey: usize,
    inspected_item_count: u128,
}

impl Monkey {
    fn new(
        items: Vec<u128>,
        operation: fn(u128) -> u128,
        test: fn(u128) -> bool,
        true_monkey: usize,
        false_monkey: usize,
    ) -> Self {
        Monkey {
            items,
            operation,
            test,
            true_monkey,
            false_monkey,
            inspected_item_count: 0,
        }
    }

    fn inspect_and_throw(&mut self, with_relief: bool, base: u128) -> Option<(u128, usize)> {
        if self.items.is_empty() {
            None
        } else {
            let inspected_item = self.items.drain(0..1).next().unwrap();
            // worry level
            let inspected_item = (self.operation)(inspected_item);
            // got bored
            let inspected_item = if with_relief {
                inspected_item / 3
            } else {
                inspected_item
            };

            let inspected_item = inspected_item % base;

            self.inspected_item_count += 1;

            if (self.test)(inspected_item.clone()) {
                Some((inspected_item, self.true_monkey))
            } else {
                Some((inspected_item, self.false_monkey))
            }
        }
    }
}

struct MonkeyGroup {
    monkeys: Vec<Monkey>,
    base: u128,
}

impl MonkeyGroup {
    fn new(monkeys: Vec<Monkey>) -> Self {
        let mut base = 1;
        for monkey in &monkeys[..] {
            base = (monkey.operation)(base);
        }

        MonkeyGroup { monkeys, base }
    }

    fn play_round(&mut self, with_relief: bool) {
        for i in 0..self.monkeys.len() {
            while let Some((item, monkey_num)) =
                self.monkeys[i].inspect_and_throw(with_relief, self.base)
            {
                self.monkeys[monkey_num].items.push(item);
            }
        }
    }

    fn monkey_business(&self) -> u128 {
        self.monkeys
            .iter()
            .map(|m| m.inspected_item_count)
            .sorted()
            .rev()
            .take(2)
            .product()
    }
}

#[aoc_generator(day11)]
fn parse_input(_input: &str) -> Vec<Monkey> {
    vec![
        Monkey::new(
            vec![54, 61, 97, 63, 74],
            |item| item * 7,
            |item| item % 17 == 0,
            5,
            3,
        ),
        Monkey::new(
            vec![61, 70, 97, 64, 99, 83, 52, 87],
            |item| item + 8,
            |item| item % 2 == 0,
            7,
            6,
        ),
        Monkey::new(
            vec![60, 67, 80, 65],
            |item| item * 13,
            |item| item % 5 == 0,
            1,
            6,
        ),
        Monkey::new(
            vec![61, 70, 76, 69, 82, 56],
            |item| item + 7,
            |item| item % 3 == 0,
            5,
            2,
        ),
        Monkey::new(vec![79, 98], |item| item + 2, |item| item % 7 == 0, 0, 3),
        Monkey::new(
            vec![72, 79, 55],
            |item| item + 1,
            |item| item % 13 == 0,
            2,
            1,
        ),
        Monkey::new(vec![63], |item| item + 4, |item| item % 19 == 0, 7, 4),
        Monkey::new(
            vec![72, 51, 93, 63, 80, 86, 81],
            |item| item * item,
            |item| item % 11 == 0,
            0,
            4,
        ),
    ]
}

#[aoc(day11, part1)]
fn part1(monkeys: &[Monkey]) -> u128 {
    let mut group = MonkeyGroup::new(monkeys.to_vec());

    for _ in 0..20 {
        group.play_round(true);
    }

    group.monkey_business()
}

#[aoc(day11, part2)]
fn part2(monkeys: &[Monkey]) -> u128 {
    let mut group = MonkeyGroup {
        monkeys: monkeys.to_vec(),
        base: 9699690,
    };

    for _ in 0..10000 {
        group.play_round(false);
    }

    group.monkey_business()
}
