use colored::Colorize;
use std::fs;

pub struct AdventOfCode {
    day: u8,
    pub lines: Vec<String>,
    pub content: String,
}

impl AdventOfCode {
    pub fn new(day: u8, year: u16) -> Self {
        let input = format!("./src/input/{}/day_{}.txt", year, day);
        let input = fs::read_to_string(input).unwrap();

        let lines = input
            .clone()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        return AdventOfCode {
            lines,
            day,
            content: input,
        };
    }

    pub fn output<T, Z>(&self, first_part: T, second_part: Z)
    where
        T: std::fmt::Debug,
        Z: std::fmt::Debug,
    {
        println!();
        println!(
            "---------- {}",
            format!("DAY {:02}", &self.day).on_bright_green().black()
        );
        println!("First part:  {}", format!("{:?}", first_part).yellow());
        println!("Second part: {}", format!("{:?}", second_part).yellow());
        println!();
    }
}

pub fn debug<T>(value: T)
where
    T: std::fmt::Debug,
{
    println!("------- {}", "DEBUGGING".on_bright_red().black());
    println!("{:?}", value);
}
