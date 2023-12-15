use regex::Regex;
use reqwest::header::COOKIE;
use std::{collections::HashMap, error::Error, fs};

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
    let input = input.trim();
    let mut sum = 0;

    for pattern in input.split("\n\n") {
        let mut vertical_candidates: HashMap<usize, usize> = HashMap::new();
        let mut horiz_line_duplicates: HashMap<&str, Vec<usize>> = HashMap::new();

        for (y, line) in pattern.lines().enumerate() {
            let line = line.trim();

            // put line into HashMap to find horizontal reflection line
            if let Some(l) = horiz_line_duplicates.get_mut(line) {
                l.push(y);
            } else {
                let mut s = vec![];
                s.push(y);
                horiz_line_duplicates.insert(line, s);
            }

            // find potential vertical reflection line
            let line_chars: Vec<char> = line.chars().collect();

            for i in 0..(line_chars.len() - 1) {
                if line_chars[i] == line_chars[i + 1] {
                    vertical_candidates
                        .entry(i)
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                }
            }
        }

        // count how often a line has appeared, and if it is two times, calc the reflection axis index
        let mut cand_horizontal_axis: HashMap<usize, usize> = HashMap::new();

        for (_line, indices) in horiz_line_duplicates.iter().filter(|x| x.1.len() == 2) {
            let reflection_index = (indices[0] + indices[1]) / 2;
            cand_horizontal_axis
                .entry(reflection_index)
                .and_modify(|counter| *counter += 2)
                .or_insert(2);
        }

        // refine vertical candidates
        let vertical_candidates = vertical_candidates
            .iter()
            .filter(|x| *x.1 == pattern.lines().count());

        let vertical_candidates_count = vertical_candidates.clone().count();

        let vertical_candidates: Vec<(&usize, &usize)> = if vertical_candidates_count > 1 {
            let first_line: Vec<char> = pattern.lines().nth(0).unwrap().chars().collect();

            vertical_candidates
                .filter(|(&index, _)| {
                    if index == 0 || index + 2 >= first_line.len() {
                        false
                    } else {
                        first_line[index - 1] == first_line[index + 2]
                    }
                })
                .collect()
        } else {
            vertical_candidates.collect()
        };

        // find best horizontal and vertical axis candidates
        let (horizontal_axis_index, horizontal_axis_index_count) = cand_horizontal_axis
            .iter()
            .max_by(|(_i_this, count_this), (_i_other, count_other)| count_this.cmp(count_other))
            .unwrap_or((&0, &0));

        let (vertical_axis_index, vertical_axis_index_count) = vertical_candidates
            .iter()
            .max_by(|(_i_this, count_this), (_i_other, count_other)| count_this.cmp(count_other))
            .unwrap_or(&(&0, &0));

        // add best candidate to sum
        if *horizontal_axis_index_count >= 6 {
            sum += 100 * (horizontal_axis_index + 1);
        } else {
            sum += **vertical_axis_index + 1;
        }
    }

    println!("The answer to part 1 is: {}", sum);

    sum
}

#[test]
fn examples() {
    let example_input = "
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..# 
    ";

    assert_eq!(part1(example_input), 405);
}
