use core::fmt::Write;
use regex::Regex;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("src/day1/input")?.parse()?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let calibration_values: Vec<u32> = input
        .lines()
        .map(|line| -> u32 {
            let first_digit = line
                .chars()
                .find(|x| x.is_digit(10))
                .expect("Could not find first digit");
            let last_digit = line
                .chars()
                .rev()
                .find(|x| x.is_digit(10))
                .expect("Could not find last digit");
            let mut number = String::new();
            let _ = write!(number, "{}{}", first_digit, last_digit);
            number.parse::<u32>().unwrap()
        })
        .collect();

    let sum: u32 = calibration_values.iter().sum();
    println!("{}", sum);

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let re = Regex::new(r"[1-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let calibration_values: Vec<u32> = input
        .lines()
        .map(|line| -> u32 {
            println!();
            dbg!(line);
            let mut digits = re.find_iter(line);
            let first_digit = digits.nth(0).unwrap().as_str();
            let first_digit_parsed = parse_digit(first_digit);

            let last_digit_parsed = if let Some(x) = digits.last() {
                parse_digit(x.as_str())
            } else {
                first_digit_parsed
            };

            let mut number = String::new();
            let _ = write!(number, "{}{}", first_digit_parsed, last_digit_parsed);
            dbg!(number.parse::<u32>().unwrap())
        })
        .collect();

    let sum: u32 = calibration_values.iter().sum();
    println!("{}", sum);

    Ok(())
}

fn parse_digit(digit: &str) -> u8 {
    let digits: Vec<String> = (1..=9).map(|x| x.to_string()).collect();
    match digit {
        x if digits.contains(&x.to_string()) => x.parse::<u8>().unwrap(),
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => { unreachable!() },
    }
}
