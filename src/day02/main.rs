use regex::Regex;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("src/day02/input")?.parse()?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let red_re: Regex = Regex::new(r"(\d+) red").unwrap();
    let green_re: Regex = Regex::new(r"(\d+) green").unwrap();
    let blue_re: Regex = Regex::new(r"(\d+) blue").unwrap();

    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

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

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let red_re: Regex = Regex::new(r"(\d+) red").unwrap();
    let green_re: Regex = Regex::new(r"(\d+) green").unwrap();
    let blue_re: Regex = Regex::new(r"(\d+) blue").unwrap();

    let sum: i32 = input
        .lines()
        .map(|game| {
            let cube_sets = game.split(";");
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for cube_set in cube_sets {
                let red = get_number_from_string(&red_re, cube_set);
                let green = get_number_from_string(&green_re, cube_set);
                let blue = get_number_from_string(&blue_re, cube_set);

                max_red = i32::max(max_red, red);
                max_green = i32::max(max_green, green);
                max_blue = i32::max(max_blue, blue);
            }

            max_red * max_green * max_blue
        })
        .sum();

    println!("{}", sum);
    Ok(())
}

fn get_number_from_string(re: &Regex, s: &str) -> i32 {
    if let Some(x) = re.captures(s) {
        x.get(1).unwrap().as_str().parse::<i32>().unwrap()
    } else {
        0
    }
}
