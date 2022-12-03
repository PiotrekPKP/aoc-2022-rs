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

fn parse_outcome(str: &str) -> Outcome {
    match str {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => unreachable!(),
    }
}

fn get_outcome(figure_1: &Figure, figure_2: &Figure) -> Outcome {
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

fn get_figure(figure: &Figure, outcome: &Outcome) -> Figure {
    match (figure, outcome) {
        (Figure::Rock, Outcome::Draw)
        | (Figure::Scissors, Outcome::Win)
        | (Figure::Paper, Outcome::Lose) => Figure::Rock,
        (Figure::Paper, Outcome::Draw)
        | (Figure::Rock, Outcome::Win)
        | (Figure::Scissors, Outcome::Lose) => Figure::Paper,
        (Figure::Scissors, Outcome::Draw)
        | (Figure::Paper, Outcome::Win)
        | (Figure::Rock, Outcome::Lose) => Figure::Scissors,
    }
}

fn main() {
    let aoc = AdventOfCode::new(2, 2022);

    let first_part = aoc
        .lines
        .iter()
        .map(|game_move| {
            let (opponent, me) = game_move.split_once(" ").unwrap();
            let opponent = parse_figure(opponent);
            let me = parse_figure(me);

            return get_outcome(&opponent, &me).get_score() as i32 + me.get_score() as i32;
        })
        .sum::<i32>();

    let second_part = aoc
        .lines
        .iter()
        .map(|game_move| {
            let (opponent, outcome) = game_move.split_once(" ").unwrap();
            let opponent = parse_figure(opponent);
            let outcome = parse_outcome(outcome);

            return get_figure(&opponent, &outcome).get_score() as i32 + outcome.get_score() as i32;
        })
        .sum::<i32>();

    aoc.output(first_part, second_part);
}
