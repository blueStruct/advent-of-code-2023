use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("src/day09/input")?.parse()?;
    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut sum = 0;

    // process sequence in each line
    for line in input.lines() {
        // parse original sequence
        let mut sequences: Vec<Vec<i64>> = vec![];

        let original_sequence: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        sequences.push(original_sequence);

        // generate all difference sequences
        while sequences.last().unwrap().iter().any(|x| *x != 0) {
            let last_sequence = sequences.last().unwrap();
            let first_element = last_sequence[0];
            let new_sequence = last_sequence[1..]
                .iter()
                .scan(first_element, |prev, &x| {
                    let this = x - *prev;
                    *prev = x;
                    Some(this)
                })
                .collect();
            sequences.push(new_sequence);
        }

        // extrapolate the last values for diff sequences and original sequence
        let seqs_len = sequences.len();
        sequences[seqs_len - 1].push(0);

        let mut prev_seq_last_element = 0;

        for sequence in sequences.iter_mut().rev().skip(1) {
            let last_element = *sequence.last().unwrap();
            let new_last_element = last_element + prev_seq_last_element;
            sequence.push(new_last_element);
            prev_seq_last_element = new_last_element;
        }

        // add new extrapolated for original sequence to sum
        sum += sequences[0].last().unwrap();
    }

    println!("The answer to part 1 is: {}", sum);

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
