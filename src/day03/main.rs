use regex::{Match, Regex};
use std::{collections::HashSet, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("src/day03/input")?.parse()?;
    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    #[derive(PartialEq, Eq, Hash, Debug)]
    struct Number {
        value: u32,
        x: usize,
        y: usize,
    }

    let numbers_re = Regex::new(r"\d+").unwrap();
    let symbols_re = Regex::new(r"[^\d\.\n]").unwrap();
    let mut numbers_vec: Vec<Vec<Match>> = vec![];
    let mut symbols_vec: Vec<Vec<Match>> = vec![];
    let mut relevant_numbers_set: HashSet<Number> = HashSet::new();

    // find numbers and symbols in each line and put them in corresponding vector
    input.lines().for_each(|line| {
        let numbers: Vec<Match> = numbers_re.find_iter(line).collect();
        numbers_vec.push(numbers);
        let symbols: Vec<Match> = symbols_re.find_iter(line).collect();
        symbols_vec.push(symbols);
    });

    // iterate over found symbols
    for (symbol_y, symbol_line) in symbols_vec.iter().enumerate() {
        for symbol in symbol_line {
            let symbol_x = symbol.start();

            // iterate over numbers in relevant lines [y-1, y, y+1]
            let start_y = symbol_y.checked_sub(1).unwrap_or_default(); // not less than 0
            let end_y = usize::min(numbers_vec.len() - 1, symbol_y + 1); // not more than numbers_vec.len()

            for (number_y, number_line) in numbers_vec[start_y..=end_y].iter().enumerate() {
                for number in number_line {
                    // check if symbol is close to number in x-direction, y is already filtered to relevant only
                    if symbol_x >= number.start().checked_sub(1).unwrap_or_default()
                        && symbol_x <= number.end() + 1
                    {
                        relevant_numbers_set.insert(Number {
                            value: number.as_str().parse().unwrap(),
                            x: number.start(),
                            y: number_y + start_y,
                        });
                    }
                }
            }
        }
    }
    let mut test_count = 0;

    // calc sum
    let sum: u32 = relevant_numbers_set
        .iter()
        .map(|x| {
            if x.y == numbers_vec.len() - 1 {
                dbg!(x.value);
                test_count += 1;
            }
            x.value
        })
        .sum();

    dbg!(test_count);
    println!("answer for part 1: {}", sum);

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
