use regex::Regex;
use reqwest::header::COOKIE;
use std::{collections::HashSet, error::Error, fs};

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
    part1and2(&input, 2);
    part1and2(&input, 1_000_000);

    Ok(())
}

fn part1and2(input: &str, scale_factor: usize) -> u128 {
    let mut galaxies: Vec<(usize, usize)> = vec![];

    // parse input, find galaxies
    let mut non_empty_cols = HashSet::new();
    let mut non_empty_lines = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                galaxies.push((y, x));
                non_empty_cols.insert(x);
                non_empty_lines.insert(y);
            }
        }
    }

    // calc cumulative sums of empty cols and rows
    let max_y = non_empty_lines.iter().max().unwrap();
    let max_x = non_empty_cols.iter().max().unwrap();

    let cum_sum_empty_lines: Vec<usize> = (0..=*max_y)
        .scan(0, |acc, i| {
            if !non_empty_lines.contains(&i) {
                *acc += scale_factor - 1;
            }
            Some(*acc)
        })
        .collect();

    let cum_sum_empty_cols: Vec<usize> = (0..=*max_x)
        .scan(0, |acc, i| {
            if !non_empty_cols.contains(&i) {
                *acc += scale_factor - 1;
            }
            Some(*acc)
        })
        .collect();

    // expand
    for (y, x) in galaxies.iter_mut() {
        *y += cum_sum_empty_lines[*y];
        *x += cum_sum_empty_cols[*x];
    }

    // calc distances
    let mut distances: Vec<usize> = vec![];

    for (i, galaxy_a) in galaxies.iter().enumerate() {
        for galaxy_b in galaxies[(i + 1)..].iter() {
            distances.push(galaxy_a.0.abs_diff(galaxy_b.0) + galaxy_a.1.abs_diff(galaxy_b.1));
        }
    }

    let answer = distances.iter().map(|x| *x as u128).sum::<u128>();

    println!("The answer to this part is: {}", answer);

    answer
}

#[test]
fn examples() {
    let example_input = "
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    ";

    assert_eq!(part1and2(example_input, 2), 374);
    assert_eq!(part1and2(example_input, 10), 1030);
    assert_eq!(part1and2(example_input, 100), 8410);
}
