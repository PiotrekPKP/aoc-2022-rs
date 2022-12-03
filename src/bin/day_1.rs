use aoc::*;

fn main() {
    let aoc = AdventOfCode::new(1);

    let first_part = aoc
        .content
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .flat_map(|line| line.parse::<u32>())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    let mut second_part_vec = aoc
        .content
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .flat_map(|line| line.parse::<u32>())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    second_part_vec.sort_by(|a, b| b.cmp(a));

    let second_part = second_part_vec.iter().take(3).sum::<u32>();

    aoc.output(first_part, second_part);
}
