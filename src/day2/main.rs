use std::{error::Error, fs};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("src/day2/input")?.parse()?;
    let red_re = Regex::new(r"(\d+) red").unwrap();
    let green_re = Regex::new(r"(\d+) green").unwrap();
    let blue_re = Regex::new(r"(\d+) blue").unwrap();
    const MAX_RED: u32 = 12; 
    const MAX_GREEN: u32 = 13; 
    const MAX_BLUE: u32 = 14; 

    let sum: usize = input
        .lines()
        .enumerate()
        .map(|(i, game)| {
            let cube_sets = game.split(";");
            for cube_set in cube_sets {
                let red = get_number_from_string(&red_re, cube_set);
                let green = get_number_from_string(&green_re, cube_set);
                let blue = get_number_from_string(&blue_re, cube_set);
                if red > MAX_RED || green > MAX_GREEN || blue > MAX_BLUE {
                    return 0;
                }
            }
            i + 1
        })
        .sum();

    println!("{}", sum);
    Ok(())
}


fn get_number_from_string(re: &Regex, s: &str) -> u32 {
    if let Some(x) = re.captures(s) {
        x.get(1).unwrap().as_str().parse::<u32>().unwrap()
    } else {
        0
    }
}
