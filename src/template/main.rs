use regex::Regex;
use reqwest::header::COOKIE;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    // get day
    let bin_name = std::env::args().nth(0).unwrap();
    let day = Regex::new(r"day(\d+)")
        .unwrap()
        .captures(&bin_name)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    // read cached input from file
    let file_path = format!("src/day{}/input", day);

    let input = if let Ok(x) = fs::read_to_string(&file_path) {
        x.trim().to_string()
    } else {
        // or get from internet
        let session_cookie: String = fs::read_to_string("src/session_cookie")?.trim().parse()?;
        let client = reqwest::blocking::Client::new();
        let input_text = client
            .get(format!("https://adventofcode.com/2023/day/{}/input", day))
            .header(COOKIE, format!("session={}", session_cookie))
            .send()?
            .text()?
            .trim()
            .to_owned();
        fs::write(&file_path, &input_text)?;
        input_text
    };

    // solve problems
    part1(&input);
    // part2(&input);

    Ok(())
}

fn part1(input: &str) -> usize {
    3
}

#[test]
fn examples() {
    let example_input = "
        #.#.### 1,1,3
        .#...#....###. 1,1,3
        .#.###.#.###### 1,3,1,6
        ####.#...#... 4,1,1
        #....######..#####. 1,6,5
        .###.##....# 3,2,1
    ";

    assert_eq!(part1(example_input), 374);
}
