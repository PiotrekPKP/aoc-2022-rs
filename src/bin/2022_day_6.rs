use aoc::*;
use std::collections::HashSet;

fn check_for_repetition(input: &String, letters_in_a_row: usize) -> usize {
    let chars = input.chars().collect::<Vec<char>>();

    return chars
        .windows(letters_in_a_row)
        .enumerate()
        .find(|(_i, letters)| {
            let set = HashSet::<&char>::from_iter(letters.iter());

            return set.len() == letters.len();
        })
        .unwrap()
        .0
        + letters_in_a_row;
}

fn main() {
    let aoc = AdventOfCode::new(6, 2022);

    let first_part = check_for_repetition(&aoc.content, 4);

    let second_part = check_for_repetition(&aoc.content, 14);

    aoc.output(first_part, second_part);
}
