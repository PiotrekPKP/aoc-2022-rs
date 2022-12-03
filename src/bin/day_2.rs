use aoc::*;

enum Figure {
    Rock,
    Paper,
    Scissors,
}

impl Figure {
    fn get_score(&self) -> u8 {
        match self {
            Figure::Rock => 1,
            Figure::Paper => 2,
            Figure::Scissors => 3,
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn get_score(&self) -> u8 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

fn parse_figure(str: &str) -> Figure {
    match str {
        "A" => Figure::Rock,
        "B" => Figure::Paper,
        "C" => Figure::Scissors,
        "X" => Figure::Rock,
        "Y" => Figure::Paper,
        "Z" => Figure::Scissors,
        _ => unreachable!(),
    }
}

fn parse_outcome(figure_1: &Figure, figure_2: &Figure) -> Outcome {
    match (figure_1, figure_2) {
        (Figure::Rock, Figure::Rock)
        | (Figure::Paper, Figure::Paper)
        | (Figure::Scissors, Figure::Scissors) => Outcome::Draw,
        (Figure::Rock, Figure::Paper)
        | (Figure::Paper, Figure::Scissors)
        | (Figure::Scissors, Figure::Rock) => Outcome::Win,
        (Figure::Rock, Figure::Scissors)
        | (Figure::Paper, Figure::Rock)
        | (Figure::Scissors, Figure::Paper) => Outcome::Lose,
    }
}

fn main() {
    let aoc = AdventOfCode::new(2);

    let first_part = aoc
        .lines
        .iter()
        .map(|game_move| {
            let (opponent, me) = game_move.split_once(" ").unwrap();
            let opponent = parse_figure(opponent);
            let me = parse_figure(me);

            return parse_outcome(&opponent, &me).get_score() as i32 + me.get_score() as i32;
        })
        .sum::<i32>();

    let second_part = 2;

    aoc.output(first_part, second_part);
}
