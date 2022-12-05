use aoc::*;
use std::collections::VecDeque;

#[derive(Debug)]
struct Action {
    amount: u32,
    from: u8,
    to: u8,
}

#[derive(Debug)]
struct Program {
    crates: Vec<VecDeque<String>>,
    actions: Vec<Action>,
}

impl Program {
    fn new(input: &String) -> Self {
        let mut crates = vec![];

        (0..determine_stack_amount(&input)).for_each(|_| crates.push(VecDeque::new()));

        let split = input.split("\n\n").collect::<Vec<&str>>();

        let crate_input = split[0];

        crate_input.lines().for_each(|line| {
            let mut temp_line = String::from(line);
            while let Some(char_pos) = temp_line.chars().position(|c| c == '[') {
                let index = determine_crate_placement(char_pos);
                crates[index].push_back(temp_line.chars().nth(char_pos + 1).unwrap().to_string());
                temp_line.replace_range(char_pos..char_pos + 3, "|_|");
            }
        });

        let actions = split[1]
            .lines()
            .map(|action_line| {
                let splitted_action_line = action_line.split(" ").collect::<Vec<&str>>();

                return Action {
                    amount: splitted_action_line[1].parse().unwrap(),
                    from: splitted_action_line[3].parse().unwrap(),
                    to: splitted_action_line[5].parse().unwrap(),
                };
            })
            .collect();

        Program { crates, actions }
    }

    fn evaluate_1(&mut self) {
        self.actions.iter().for_each(|action| {
            for _ in 0..action.amount {
                let value = self.crates[action.from as usize - 1].pop_front().unwrap();
                self.crates[action.to as usize - 1].push_front(value);
            }
        })
    }

    fn evaluate_2(&mut self) {
        self.actions.iter().for_each(|action| {
            let mut picked_up = vec![];

            (0..action.amount).for_each(|_| {
                let value = self.crates[action.from as usize - 1].pop_front().unwrap();
                picked_up.push(value);
            });

            picked_up.iter().rev().for_each(|krate| {
                self.crates[action.to as usize - 1].push_front(krate.clone());
            });
        })
    }
}

fn determine_stack_amount(input: &String) -> usize {
    let line_with_separator = input.lines().position(|l| l.trim() == "").unwrap() - 1;

    let stacks = input
        .lines()
        .nth(line_with_separator)
        .unwrap()
        .trim()
        .split("   ")
        .count();

    return stacks;
}

fn determine_crate_placement(char_pos: usize) -> usize {
    let new_position = char_pos - (char_pos as f32 / 4.).floor() as usize;

    return (new_position as f32 / 3.).floor() as usize;
}

fn get_top_crates(crates: &Vec<VecDeque<String>>) -> String {
    return crates
        .iter()
        .map(|crate_stack| {
            crate_stack
                .iter()
                .nth(0)
                .unwrap()
                .chars()
                .collect::<Vec<char>>()
                .first()
                .unwrap()
                .clone()
        })
        .collect::<String>();
}

fn main() {
    let aoc = AdventOfCode::new(5, 2022);

    let mut program = Program::new(&aoc.content);
    program.evaluate_1();
    let first_part = get_top_crates(&program.crates);

    let mut program = Program::new(&aoc.content);
    program.evaluate_2();
    let second_part = get_top_crates(&program.crates);

    aoc.output(first_part, second_part);
}
