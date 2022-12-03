use aoc::*;
use std::collections::HashSet;

fn main() {
    let aoc = AdventOfCode::new(3);

    let first_part = aoc
        .lines
        .iter()
        .map(|line| {
            let half = (line.len() as f64 / 2.).ceil();

            let first_half = &line[..half as usize];
            let second_half = &line[half as usize..];

            return vec![first_half.to_string(), second_half.to_string()];
        })
        .map(|halfs| {
            let first_half = halfs[0].chars().collect::<Vec<char>>();
            let second_half = halfs[1].chars().collect::<Vec<char>>();

            let first_set = HashSet::<char>::from_iter(first_half.iter().cloned());
            let second_set = HashSet::<char>::from_iter(second_half.iter().cloned());

            return first_set.intersection(&second_set).next().unwrap().clone();
        })
        .map(|c| {
            return if c as u32 >= 97 {
                c as u32 - 96
            } else {
                c as u32 - 38
            };
        })
        .sum::<u32>();

    let second_part = aoc
        .lines
        .chunks(3)
        .map(|group| {
            let first = group[0].chars().collect::<Vec<char>>();
            let second = group[1].chars().collect::<Vec<char>>();
            let third = group[2].chars().collect::<Vec<char>>();

            let first_set = HashSet::<char>::from_iter(first.iter().cloned());
            let second_set = HashSet::<char>::from_iter(second.iter().cloned());
            let third_set = HashSet::<char>::from_iter(third.iter().cloned());

            let first_second = first_set.intersection(&second_set).collect::<Vec<&char>>();
            let second_third = second_set.intersection(&third_set).collect::<Vec<&char>>();

            return first_second
                .iter()
                .filter(|c| second_third.contains(c))
                .map(|c| {
                    return if **c as u32 >= 97 {
                        **c as u32 - 96
                    } else {
                        **c as u32 - 38
                    };
                })
                .sum::<u32>();
        })
        .sum::<u32>();

    aoc.output(first_part, second_part);
}
