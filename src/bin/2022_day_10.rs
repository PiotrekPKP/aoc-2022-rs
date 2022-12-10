use aoc::*;
use colored::Colorize;
use std::str::FromStr;

#[derive(Debug)]
enum Action {
    AddX(Option<i32>),
    Noop,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action, amount) = s.split_once(" ").unwrap_or(("noop", "0"));

        match action {
            "noop" => Ok(Action::Noop),
            "addx" => {
                let amount = amount.parse::<i32>().unwrap();
                return Ok(Action::AddX(Some(amount)));
            }
            _ => Err(()),
        }
    }
}

struct State {
    current_cycle: u32,
    current_x: i32,
    next_x_modifier: Option<i32>,
}

impl State {
    fn new() -> Self {
        State {
            current_cycle: 0,
            current_x: 1,
            next_x_modifier: None,
        }
    }
}

#[derive(Debug)]
struct GPUState {
    current_cycle: u32,
    current_x: i32,
    next_x_modifier: Option<i32>,
    screen: [[bool; 40]; 6],
}

impl GPUState {
    fn new() -> Self {
        GPUState {
            current_cycle: 0,
            current_x: 1,
            next_x_modifier: None,
            screen: [[false; 40]; 6],
        }
    }

    fn get_current_pos(&self) -> (u32, u32) {
        let row = if self.current_cycle >= 1 && self.current_cycle <= 40 {
            0
        } else if self.current_cycle >= 41 && self.current_cycle <= 80 {
            1
        } else if self.current_cycle >= 81 && self.current_cycle <= 120 {
            2
        } else if self.current_cycle >= 121 && self.current_cycle <= 160 {
            3
        } else if self.current_cycle >= 161 && self.current_cycle <= 200 {
            4
        } else if self.current_cycle >= 201 && self.current_cycle <= 240 {
            5
        } else {
            unreachable!()
        };

        let column = self.current_cycle - 40 * row;

        return (row, column - 1);
    }
}

fn main() {
    let aoc = AdventOfCode::new(10, 2022);

    let input = &aoc.lines;

    let first_part = input
        .iter()
        .flat_map(|l| l.parse::<Action>())
        .flat_map(|action| match action {
            Action::Noop => vec![Action::Noop],
            Action::AddX(v) => vec![Action::AddX(None), Action::AddX(v)],
        })
        .scan(State::new(), |state, action| {
            state.current_cycle += 1;
            if let Some(next_x) = state.next_x_modifier {
                state.current_x += next_x;
            }

            state.next_x_modifier = match action {
                Action::Noop => None,
                Action::AddX(v) => {
                    if let Some(v) = v {
                        Some(v)
                    } else {
                        None
                    }
                }
            };

            if state.current_cycle == 20
                || state.current_cycle == 60
                || state.current_cycle == 100
                || state.current_cycle == 140
                || state.current_cycle == 180
                || state.current_cycle == 220
            {
                return Some(Some(state.current_cycle as i32 * state.current_x));
            }

            return Some(None);
        })
        .flat_map(|x| x)
        .sum::<i32>();

    input
        .iter()
        .flat_map(|l| l.parse::<Action>())
        .flat_map(|action| match action {
            Action::Noop => vec![Action::Noop],
            Action::AddX(v) => vec![Action::AddX(None), Action::AddX(v)],
        })
        .scan(GPUState::new(), |state, action| {
            state.current_cycle += 1;
            if let Some(next_x) = state.next_x_modifier {
                state.current_x += next_x;
            }

            state.next_x_modifier = match action {
                Action::Noop => None,
                Action::AddX(v) => {
                    if let Some(v) = v {
                        Some(v)
                    } else {
                        None
                    }
                }
            };

            let (y, x) = state.get_current_pos();

            state.screen[y as usize][x as usize] =
                (state.current_x - 1..=state.current_x + 1).contains(&(x as i32));

            if state.current_cycle >= 40
                && state.current_cycle <= 240
                && state.current_cycle % 40 == 0
            {
                return Some(Some(state.screen[y as usize]));
            }

            return Some(None);
        })
        .flat_map(|x| x)
        .for_each(|line| {
            line.iter().for_each(|b| {
                print!(
                    "{}",
                    if *b {
                        "  ".on_yellow()
                    } else {
                        "  ".on_black()
                    }
                )
            });

            println!();
        });

    aoc.output(first_part, "Check out this text ^^");
}
