use std::str::FromStr;

use aoc::*;

const ELEMENTS: usize = 1000;

#[derive(Debug)]
enum Action {
    TurnOn(((i32, i32), (i32, i32))),
    TurnOff(((i32, i32), (i32, i32))),
    Toggle(((i32, i32), (i32, i32))),
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("turn on") {
            let new_str = &s["turn on".len() + 1..];
            let coords = get_coords(new_str);

            return Ok(Action::TurnOn(coords));
        } else if s.contains("turn off") {
            let new_str = &s["turn off".len() + 1..];
            let coords = get_coords(new_str);

            return Ok(Action::TurnOff(coords));
        } else if s.contains("toggle") {
            let new_str = &s["toggle".len() + 1..];
            let coords = get_coords(new_str);

            return Ok(Action::Toggle(coords));
        }

        Err(String::from("Unreachable."))
    }
}

fn get_coords(string: &str) -> ((i32, i32), (i32, i32)) {
    let (from_str, to_str) = string.split_once(" through ").unwrap();
    let (from_x, from_y) = from_str.split_once(",").unwrap();
    let (to_x, to_y) = to_str.split_once(",").unwrap();

    let f_x: i32 = from_x.parse().unwrap();
    let f_y: i32 = from_y.parse().unwrap();
    let t_x: i32 = to_x.parse().unwrap();
    let t_y: i32 = to_y.parse().unwrap();

    ((f_x, f_y), (t_x, t_y))
}

fn get_range(from: &(i32, i32), to: &(i32, i32)) -> Vec<(i32, i32)> {
    let mut coords = vec![];

    for x in from.0..=to.0 {
        for y in from.1..=to.1 {
            coords.push((x, y));
        }
    }

    coords
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum LightState {
    On,
    Off,
}

#[derive(Debug)]
struct Board1 {
    lights: [[LightState; ELEMENTS]; ELEMENTS],
}

#[derive(Debug)]
struct Board2 {
    lights: [[u8; ELEMENTS]; ELEMENTS],
}

impl Board1 {
    fn new() -> Self {
        Board1 {
            lights: [[LightState::Off; ELEMENTS]; ELEMENTS],
        }
    }

    fn execute(&mut self, action: &Action) {
        match action {
            Action::TurnOn((from, to)) => {
                get_range(from, to).iter().for_each(|coord| {
                    self.lights[coord.0 as usize][coord.1 as usize] = LightState::On;
                });
            }
            Action::TurnOff((from, to)) => {
                get_range(from, to).iter().for_each(|coord| {
                    self.lights[coord.0 as usize][coord.1 as usize] = LightState::Off;
                });
            }
            Action::Toggle((from, to)) => {
                get_range(from, to).iter().for_each(|coord| {
                    let state = self.lights[coord.0 as usize][coord.1 as usize];
                    self.lights[coord.0 as usize][coord.1 as usize] = if state == LightState::On {
                        LightState::Off
                    } else {
                        LightState::On
                    };
                });
            }
        }
    }
}

impl Board2 {
    fn new() -> Self {
        Board2 {
            lights: [[0; ELEMENTS]; ELEMENTS],
        }
    }

    fn execute(&mut self, action: &Action) {
        match action {
            Action::TurnOn((from, to)) => {
                get_range(from, to).iter().for_each(|coord| {
                    self.lights[coord.0 as usize][coord.1 as usize] += 1;
                });
            }
            Action::TurnOff((from, to)) => {
                get_range(from, to).iter().for_each(|coord| {
                    self.lights[coord.0 as usize][coord.1 as usize] =
                        if self.lights[coord.0 as usize][coord.1 as usize] > 0 {
                            self.lights[coord.0 as usize][coord.1 as usize] - 1
                        } else {
                            0
                        };
                });
            }
            Action::Toggle((from, to)) => {
                get_range(from, to).iter().for_each(|coord| {
                    self.lights[coord.0 as usize][coord.1 as usize] += 2;
                });
            }
        }
    }
}

fn main() {
    let aoc = AdventOfCode::new(6, 2015);

    let mut board = Board1::new();
    aoc.lines
        .iter()
        .map(|line| Action::from_str(line).unwrap())
        .for_each(|action| board.execute(&action));
    let first_part = board
        .lights
        .iter()
        .flatten()
        .filter(|light_state| **light_state == LightState::On)
        .count();

    let mut board = Board2::new();
    aoc.lines
        .iter()
        .map(|line| Action::from_str(line).unwrap())
        .for_each(|action| board.execute(&action));
    let second_part = board
        .lights
        .iter()
        .flatten()
        .map(|x| *x as u32)
        .sum::<u32>();

    aoc.output(first_part, second_part);
}
