use aoc::*;
use fancy_regex::Regex;

fn main() {
    let aoc = AdventOfCode::new(5, 2015);

    let first_part = aoc
        .lines
        .iter()
        .filter(|line| {
            let mut vowels = 0;
            let mut double = false;
            let mut forbidden = false;

            for (i, char) in line.chars().enumerate() {
                if char == 'a' || char == 'e' || char == 'i' || char == 'o' || char == 'u' {
                    vowels += 1;
                }

                if i > 0 && line.chars().nth(i - 1).unwrap() == char {
                    double = true;
                }

                if i > 0 && line.chars().nth(i - 1).unwrap() == 'a' && char == 'b' {
                    forbidden = true;
                }

                if i > 0 && line.chars().nth(i - 1).unwrap() == 'c' && char == 'd' {
                    forbidden = true;
                }

                if i > 0 && line.chars().nth(i - 1).unwrap() == 'p' && char == 'q' {
                    forbidden = true;
                }

                if i > 0 && line.chars().nth(i - 1).unwrap() == 'x' && char == 'y' {
                    forbidden = true;
                }
            }

            vowels >= 3 && double && !forbidden
        })
        .count();

    let second_part = aoc
        .lines
        .iter()
        .filter(|line| {
            let pair = Regex::new(r"(..).*\1").unwrap();
            let repeat = Regex::new(r"(.).\1").unwrap();

            pair.is_match(line).unwrap() && repeat.is_match(line).unwrap()
        })
        .count();

    aoc.output(first_part, second_part);
}
