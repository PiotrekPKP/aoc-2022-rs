use aoc::*;

fn main() {
    let aoc = AdventOfCode::new(2, 2015);

    let first_part = aoc
        .lines
        .iter()
        .map(|present_dimensions| {
            let dimensions = present_dimensions
                .split("x")
                .map(|dimension| dimension.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let l = dimensions[0];
            let w = dimensions[1];
            let h = dimensions[2];

            let sides = vec![l * w, w * h, h * l];
            let min = sides.iter().min().unwrap();

            2 * l * w + 2 * w * h + 2 * h * l + min
        })
        .sum::<u32>();

    let second_part = aoc
        .lines
        .iter()
        .map(|present_dimensions| {
            let dimensions = present_dimensions
                .split("x")
                .map(|dimension| dimension.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let l = dimensions[0];
            let w = dimensions[1];
            let h = dimensions[2];

            let sides = vec![l + w, w + h, h + l];
            let min = sides.iter().min().unwrap();

            2 * min + l * w * h
        })
        .sum::<u32>();

    aoc.output(first_part, second_part);
}
