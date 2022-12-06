use aoc::*;
use std::collections::HashSet;

fn check_for_repetition(input: &String, letters_in_a_row: usize) -> usize {
    return input
        .as_bytes()
        .windows(letters_in_a_row)
        .position(|letters| letters.iter().collect::<HashSet<_>>().len() == letters_in_a_row)
        .unwrap()
        + letters_in_a_row;
}

fn main() {
    let aoc = AdventOfCode::new(6, 2022);

    let first_part = check_for_repetition(&aoc.content, 4);

    let second_part = check_for_repetition(&aoc.content, 14);

    aoc.output(first_part, second_part);
}
