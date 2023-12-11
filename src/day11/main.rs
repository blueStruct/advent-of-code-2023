use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("src/day11/example_input")?.parse()?;
    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
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

    // calc cumulative sums
    let mut max_y = non_empty_lines.iter().max();
    let mut max_x = non_empty_cols.iter().max();

    let cum_sum_empty_lines: Vec<usize> = (0..max_y)
        .scan(0, |acc, i| {
            if !non_empty_lines.contains(i) {
                *acc += 1;
            }
            Some(acc)
        })
        .collect();

    let cum_sum_empty_cols: Vec<usize> = (0..max_x)
        .scan(0, |acc, i| {
            if !non_empty_cols.contains(i) {
                *acc += 1;
            }
            Some(acc)
        })
        .collect();

    // expand
    for (y, x) in galaxies.iter_mut() {
        *y += cum_sum_empty_lines[y];
        *x += cum_sum_empty_cols[x];
    }

    // calc distances
    let mut distances: Vec<usize> = vec![];

    for (i, galaxy_a) in galaxies.iter().enumerate() {
        for galaxy_b in galaxies[i + 1..].iter() {
            distances.push(galaxy_a.0.abs_diff(galaxy_b.0) + galaxy_a.1.abs_diff(galaxy_b.1) + 1);
        }
    }

    println!("The answer to part 1 is: {}", distances.iter().sum());

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
