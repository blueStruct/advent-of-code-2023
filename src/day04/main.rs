use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("src/day04/input")?.parse()?;
    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut sum = 0;

    for card in input.lines() {
        let all_numbers_str = card.split_once(':').unwrap().1;
        let (winning_numbers_str, candidate_numbers_str) = all_numbers_str.split_once('|').unwrap();
        let winning_numbers: Vec<u8> = winning_numbers_str
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let candidate_numbers: Vec<u8> = candidate_numbers_str
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let count = candidate_numbers
            .iter()
            .filter(|x| winning_numbers.contains(x))
            .count();
        let points = if count == 0 {
            0
        } else {
            2i32.pow(count as u32 - 1)
        };
        sum += points;
    }

    println!("The answer for part 1 is: {}", sum);

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
