use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("src/day01/input")?.parse()?;
    part1(&input);
    part2(&input);

    Ok(())
}


fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}


fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
