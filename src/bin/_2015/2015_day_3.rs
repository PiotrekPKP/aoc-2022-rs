use aoc::*;
use std::collections::HashSet;

struct Program {
    current_position: (i32, i32),
    visited_houses: HashSet<(i32, i32)>,
}

impl Program {
    fn new() -> Self {
        Program {
            current_position: (0, 0),
            visited_houses: HashSet::from_iter(vec![(0, 0)]),
        }
    }

    fn new_step(&mut self, santa_move: &char) {
        let position = match santa_move {
            '^' => (self.current_position.0, self.current_position.1 + 1),
            'v' => (self.current_position.0, self.current_position.1 - 1),
            '>' => (self.current_position.0 + 1, self.current_position.1),
            '<' => (self.current_position.0 - 1, self.current_position.1),
            _ => unreachable!(),
        };

        self.current_position = position;
        self.visited_houses.insert(position);
    }
}

fn main() {
    let aoc = AdventOfCode::new(3, 2015);

    let mut santa = Program::new();
    aoc.content.chars().for_each(|char| santa.new_step(&char));
    let first_part = santa.visited_houses.len();

    let mut santa = Program::new();
    let mut robo_santa = Program::new();
    aoc.content.chars().enumerate().for_each(|(i, char)| {
        if i % 2 == 0 {
            santa.new_step(&char)
        } else {
            robo_santa.new_step(&char)
        }
    });
    let second_part = santa.visited_houses.len() + robo_santa.visited_houses.len()
        - santa
            .visited_houses
            .intersection(&robo_santa.visited_houses)
            .count();

    aoc.output(first_part, second_part);
}
