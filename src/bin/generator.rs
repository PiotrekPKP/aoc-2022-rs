use dotenv::dotenv;
use std::{env, fs};

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

    if res.contains("Please don't repeatedly request this endpoint before it unlocks!") {
        println!("This day is not yet available!");
        return;
    }

    fs::create_dir_all(format!("./src/input/{}", year)).unwrap();
    let path = format!("./src/input/{}/day_{}.txt", year, day);
    fs::write(path, res.trim_end()).expect("Failed to save data!");

    generate_boilerplate(day, year);

    println!("Data saved!");
}

fn generate_boilerplate(day: u32, year: u32) {
    let path = format!("./src/bin/{}_day_{}.rs", year, day);
    let content = fs::read_to_string("./src/template/day.txt").unwrap();

    let content = content
        .replace("<DAY>", &day.to_string())
        .replace("<YEAR>", &year.to_string());

    fs::write(path, content).expect("Failed to save data!");
}
