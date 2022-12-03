use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let day = env::args()
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .expect("You have to provide a day!");

    const DEFAULT_YEAR: u32 = 2022;

    let year = env::args()
        .nth(2)
        .unwrap_or(DEFAULT_YEAR.to_string())
        .parse::<u32>()
        .unwrap_or(DEFAULT_YEAR);

    println!("Fetching data...");

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            year, day
        ))
        .header(
            "Cookie",
            format!(
                "session={}",
                env::var("SESSION_COOKIE").expect("You have to provide a session cookie!")
            ),
        )
        .send()
        .expect("Failed to fetch data!")
        .text()
        .expect("Failed to parse data!");

    let path = format!("./src/input/{}/day_{}.txt", year, day);
    std::fs::write(path, res.trim_end()).expect("Failed to save data!");

    println!("Data saved!");
}
