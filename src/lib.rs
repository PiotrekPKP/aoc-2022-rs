use colored::Colorize;
use std::fs;

pub struct AdventOfCode {
    day: u8,
    year: u16,
    pub lines: Vec<String>,
    pub content: String,
    pub test_lines: Option<Vec<String>>,
    pub test_content: Option<String>,
}

impl AdventOfCode {
    pub fn new(day: u8, year: u16) -> Self {
        let input = format!("./src/input/{}/day_{}.txt", year, day);
        let input = fs::read_to_string(input).unwrap();

        let test_input = format!("./src/input/{}/day_{}.test.txt", year, day);
        let test_input = fs::read_to_string(test_input);

        let test_lines = match &test_input {
            Ok(t) => Some(t.lines().map(|x| x.to_string()).collect()),
            Err(_) => None,
        };
        let test_content = match &test_input {
            Ok(t) => Some(t.clone()),
            Err(_) => None,
        };

        let lines = input
            .clone()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        return AdventOfCode {
            lines,
            day,
            year,
            content: input,
            test_lines,
            test_content,
        };
    }

    pub fn output<T, Z>(&self, first_part: T, second_part: Z)
    where
        T: std::fmt::Debug,
        Z: std::fmt::Debug,
    {
        println!();
        println!(
            "----- {}",
            format!("DAY {:02} / {}", &self.day, &self.year)
                .on_bright_green()
                .black()
        );
        println!("First part:  {}", format!("{:#?}", first_part).yellow());
        println!("Second part: {}", format!("{:#?}", second_part).yellow());
        println!();
    }
}

pub fn debug<T>(value: T)
where
    T: std::fmt::Debug,
{
    println!("----- {}", "DEBUGGING".on_bright_red().black());
    println!("{:#?}", value);
    println!();
}
