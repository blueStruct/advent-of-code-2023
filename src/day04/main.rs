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
        // parse numbers from input
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

        // how many winning numbers are among the second set of numbers?
        let count = candidate_numbers
            .iter()
            .filter(|x| winning_numbers.contains(x))
            .count();

        // calc points
        let points = if count == 0 {
            0
        } else {
            2i32.pow(count as u32 - 1)
        };

        // add points to overall sum
        sum += points;
    }

    println!("The answer for part 1 is: {}", sum);

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut instances: Vec<u32> = vec![1; input.lines().count()]; // one original card for each input line

    for (card_nr, card) in input.lines().enumerate() {
        // parse numbers from input
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

        // how many winning numbers are among the second set of numbers?
        let count = candidate_numbers
            .iter()
            .filter(|x| winning_numbers.contains(x))
            .count();

        // which of the next cards will receive copies? + avoid index overflow
        let end_index = usize::min(input.lines().count() - 1, card_nr + count);

        // how many copies will be added to the next cards?
        let instances_current_card = instances[card_nr];

        // add copies of scratchcards
        for index in (card_nr + 1)..=(end_index) {
            instances[index] += instances_current_card;
        }
    }

    println!(
        "The answer for part 2 is: {}",
        instances.iter().sum::<u32>()
    );

    Ok(())
}
