use regex::Regex;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("src/day06/input")?.parse()?;
    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let times: Vec<u64> = Regex::new(r"Time:(.*)\n")
        .unwrap()
        .captures(input)
        .unwrap()
        .get(1) // index 1 is the capture group in the regex
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let distances: Vec<u64> = Regex::new(r"Distance:(.*)\z")
        .unwrap()
        .captures(input)
        .unwrap()
        .get(1) // index 1 is the capture group in the regex
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut results = vec![];

    for (&available_time, min_distance) in times.iter().zip(distances) {
        let mut count = 0u32;
        let mut success_zone = false;

        for t_button_down in 1..available_time {
            let distance = t_button_down * (available_time - t_button_down);

            if distance > min_distance {
                count += 1;
                success_zone = true;
            } else {
                // stop iterating, not working any more
                if success_zone == true {
                    break;
                }
            }
        }

        results.push(count);
    }

    let product: u32 = results.iter().product();

    println!("The answer for part 1 is: {}", product);

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let available_time: u64 = Regex::new(r"Time:(.*)\n")
        .unwrap()
        .captures(input)
        .unwrap()
        .get(1) // index 1 is the capture group in the regex
        .unwrap()
        .as_str()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();

    let min_distance: u64 = Regex::new(r"Distance:(.*)\z")
        .unwrap()
        .captures(input)
        .unwrap()
        .get(1) // index 1 is the capture group in the regex
        .unwrap()
        .as_str()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();

    let mut count = 0u32;
    let mut success_zone = false;

    for t_button_down in 1..available_time {
        let distance = t_button_down * (available_time - t_button_down);

        if distance > min_distance {
            count += 1;
            success_zone = true;
        } else {
            // stop iterating, not working any more
            if success_zone == true {
                break;
            }
        }
    }

    println!("The answer for part 2 is: {}", count);

    Ok(())
}
