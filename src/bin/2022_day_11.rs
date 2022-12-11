use aoc::*;
use std::{collections::HashMap, str::FromStr};

const ROUND_AMOUNT1: u8 = 20;
const ROUND_AMOUNT2: u128 = 10_000;
const OLD_KEYWORD: &str = "old";

#[derive(Debug)]
struct Test {
    divisible_by: u128,
    pass: u8,
    fail: u8,
}

impl FromStr for Test {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let test_line = lines.next().unwrap();
        let pass_line = lines.next().unwrap();
        let fail_line = lines.next().unwrap();

        let divisible_by = test_line
            .split(" ")
            .last()
            .unwrap()
            .parse::<u128>()
            .unwrap();
        let pass = pass_line.split(" ").last().unwrap().parse::<u8>().unwrap();
        let fail = fail_line.split(" ").last().unwrap().parse::<u8>().unwrap();

        Ok(Test {
            divisible_by,
            pass,
            fail,
        })
    }
}

impl Test {
    fn run(&self, value: u128) -> u8 {
        if value % self.divisible_by == 0 {
            self.pass
        } else {
            self.fail
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: u8,
    starting_items: Vec<u128>,
    operation: String,
    test: Test,
    iterations: u128,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let id_line = lines.next().unwrap();
        let items_line = lines.next().unwrap();
        let operation_line = lines.next().unwrap();

        let (_, id) = id_line.split_once(" ").unwrap();
        let id = id.replace(":", "").parse::<u8>().unwrap();

        let (_, items) = items_line.split_once("Starting items: ").unwrap();
        let items = items
            .split(", ")
            .flat_map(|x| x.parse())
            .collect::<Vec<u128>>();

        let (_, operation) = operation_line.split_once("Operation: new = ").unwrap();

        let (_, test) = s.split_once("Test").unwrap();
        let test = test.parse::<Test>().unwrap();

        Ok(Monkey {
            id,
            starting_items: items,
            operation: operation.to_string(),
            test,
            iterations: 0,
        })
    }
}

impl Monkey {
    fn run_op(&mut self, value: u128) -> u128 {
        let mut op = self.operation.split(" ");

        let left_side = op.next().unwrap();
        let op_type = op.next().unwrap();
        let right_side = op.next().unwrap();

        let left_side = if left_side == OLD_KEYWORD {
            value
        } else {
            left_side.parse::<u128>().unwrap()
        };

        let right_side = if right_side == OLD_KEYWORD {
            value
        } else {
            right_side.parse::<u128>().unwrap()
        };

        self.iterations += 1;

        return match op_type {
            "*" => left_side * right_side,
            "+" => left_side + right_side,
            _ => unreachable!(),
        };
    }
}

fn main() {
    let aoc = AdventOfCode::new(11, 2022);

    let input = &aoc.content;

    let mut monkeys = input
        .split("\n\n")
        .flat_map(|monkey_str| monkey_str.parse::<Monkey>())
        .collect::<Vec<Monkey>>();

    let mut monkey_items = HashMap::<u8, Vec<u128>>::new();
    monkeys.iter_mut().for_each(|m| {
        m.starting_items.reverse();
        monkey_items.insert(m.id, m.starting_items.clone());
    });

    (0..ROUND_AMOUNT1).for_each(|_| {
        monkeys.iter_mut().for_each(|monkey| {
            let mut items = monkey_items.get(&monkey.id).unwrap().clone();

            while let Some(worry_level) = items.pop() {
                let new_value = monkey.run_op(worry_level) / 3;
                let pass_to = monkey.test.run(new_value);

                monkey_items.insert(monkey.id, items.clone());
                monkey_items.get_mut(&pass_to).unwrap().push(new_value);
            }
        });
    });

    let mut iterations = monkeys.iter().map(|m| m.iterations).collect::<Vec<u128>>();
    iterations.sort_by(|a, b| b.cmp(a));

    let first_part = format!(
        "{} * {} = {}",
        iterations[0],
        iterations[1],
        iterations[0] * iterations[1]
    );

    let mut monkeys = input
        .split("\n\n")
        .flat_map(|monkey_str| monkey_str.parse::<Monkey>())
        .collect::<Vec<Monkey>>();

    let mut monkey_items = HashMap::<u8, Vec<u128>>::new();
    monkeys.iter_mut().for_each(|m| {
        m.starting_items.reverse();
        monkey_items.insert(m.id, m.starting_items.clone());
    });

    let global_mod = monkeys.iter().fold(1, |a, b| a * b.test.divisible_by);

    (0..ROUND_AMOUNT2).for_each(|r| {
        monkeys.iter_mut().for_each(|monkey| {
            let mut items = monkey_items.get(&monkey.id).unwrap().clone();

            while let Some(worry_level) = items.pop() {
                let new_value = monkey.run_op(worry_level) % global_mod;
                let pass_to = monkey.test.run(new_value);

                monkey_items.insert(monkey.id, items.clone());
                monkey_items.get_mut(&pass_to).unwrap().push(new_value);
            }
        });
    });

    let mut iterations = monkeys.iter().map(|m| m.iterations).collect::<Vec<u128>>();
    iterations.sort_by(|a, b| b.cmp(a));

    let second_part = format!(
        "{} * {} = {}",
        iterations[0],
        iterations[1],
        iterations[0] * iterations[1]
    );

    aoc.output(first_part, second_part);
}
