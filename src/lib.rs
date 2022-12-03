use std::fs;

pub struct AdventOfCode {
    day: u8,
    pub lines: Vec<String>,
}

impl AdventOfCode {
    pub fn new(day: u8) -> Self {
        let input = format!("./src/input/day_{}.txt", day);
        let input = fs::read_to_string(input).unwrap();

        let lines = input
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        return AdventOfCode { lines, day };
    }

    pub fn output(&self, first_part: u32, second_part: u32) {
        println!("---------- DAY {:02}", &self.day);
        println!("First part:  {:?}", first_part);
        println!("Second part: {:?}", second_part);
        println!();
    }
}
