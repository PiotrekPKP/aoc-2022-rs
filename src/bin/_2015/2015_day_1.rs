use aoc::*;

fn main() {
    let aoc = AdventOfCode::new(1, 2015);

    let first_part = aoc
        .content
        .chars()
        .map(|char| if char == '(' { 1 } else { -1 })
        .sum::<i32>();

    let second_part = aoc
        .content
        .chars()
        .map(|char| if char == '(' { 1 } else { -1 })
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .position(|x| x == -1)
        .unwrap()
        + 1;

    aoc.output(first_part, second_part);
}
