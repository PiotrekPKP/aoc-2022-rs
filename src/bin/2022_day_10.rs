use aoc::*;
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

            state.next_x_modifier = None;
            match action {
                Action::Noop => {}
                Action::AddX(v) => {
                    if let Some(v) = v {
                        state.next_x_modifier = Some(v);
                    }
                }
            }

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

    let second_part = 2;

    aoc.output(first_part, second_part);
}
