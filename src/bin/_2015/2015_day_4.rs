use aoc::*;

fn main() {
    let aoc = AdventOfCode::new(4, 2015);

    let first_part = (0..)
        .find(|i| {
            let hash = md5::compute(format!("{}{}", aoc.content, i));
            hash[0] == 0 && hash[1] == 0 && hash[2] < 16
        })
        .unwrap();

    let second_part = (0..)
        .find(|i| {
            let hash = md5::compute(format!("{}{}", aoc.content, i));
            hash[0] == 0 && hash[1] == 0 && hash[2] == 0
        })
        .unwrap();

    aoc.output(first_part, second_part);
}
