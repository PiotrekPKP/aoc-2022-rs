use aoc::*;

fn main() {
    let aoc = AdventOfCode::new(4, 2022);

    let first_part = aoc
        .lines
        .iter()
        .map(|line| {
            let (pair1, pair2) = line.split_once(",").unwrap();
            let (pair1start, pair1end) = pair1.split_once("-").unwrap();
            let (pair2start, pair2end) = pair2.split_once("-").unwrap();

            let pair1start = pair1start.parse::<u32>().unwrap();
            let pair1end = pair1end.parse::<u32>().unwrap();
            let pair2start = pair2start.parse::<u32>().unwrap();
            let pair2end = pair2end.parse::<u32>().unwrap();

            ((pair1start, pair1end), (pair2start, pair2end))
        })
        .filter(|pairs| {
            let (pair1, pair2) = pairs;
            let (pair1start, pair1end) = pair1;
            let (pair2start, pair2end) = pair2;

            return (pair1start <= pair2start && pair1end >= pair2end)
                || (pair2start <= pair1start && pair2end >= pair1end);
        })
        .count();

    let second_part = aoc
        .lines
        .iter()
        .map(|line| {
            let (pair1, pair2) = line.split_once(",").unwrap();
            let (pair1start, pair1end) = pair1.split_once("-").unwrap();
            let (pair2start, pair2end) = pair2.split_once("-").unwrap();

            let pair1start = pair1start.parse::<u32>().unwrap();
            let pair1end = pair1end.parse::<u32>().unwrap();
            let pair2start = pair2start.parse::<u32>().unwrap();
            let pair2end = pair2end.parse::<u32>().unwrap();

            ((pair1start, pair1end), (pair2start, pair2end))
        })
        .filter(|pairs| {
            let (pair1, pair2) = pairs;
            let (pair1start, pair1end) = pair1;
            let (pair2start, pair2end) = pair2;

            let complete_overlap = (pair1start <= pair2start && pair1end >= pair2end)
                || (pair2start <= pair1start && pair2end >= pair1end);

            return complete_overlap
                || (pair1start..=pair1end).contains(&pair2start)
                || (pair2start..=pair2end).contains(&pair1start)
                || (pair1start..=pair1end).contains(&pair2end)
                || (pair2start..=pair2end).contains(&pair1end);
        })
        .count();

    aoc.output(first_part, second_part);
}
