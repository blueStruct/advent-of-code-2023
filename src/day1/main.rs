use core::fmt::Write;
use std::{fs, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("src/day1/input")?.parse()?;

    let calibration_values: Vec<u32> = input.lines().map(|line| -> u32 {
        let first_digit = line
            .chars()
            .skip_while(|x| !x.is_digit(10))
            .nth(0)
            .expect("Could not find first digit");
        let last_digit = line
            .chars()
            .rev()
            .skip_while(|x| !x.is_digit(10))
            .nth(0)
            .expect("Could not find last digit");
        let mut number = String::new();
        let _ = write!(number, "{}{}", first_digit, last_digit);
        number.parse::<u32>().unwrap()
    }).collect();

    let sum: u32 = calibration_values.iter().sum();
    println!("{}", sum);

    Ok(())
}
