use regex::Regex;
use reqwest::header::COOKIE;
use std::{error::Error, fs, iter::Map};

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
    part2(&input);

    Ok(())
}

fn part1(input: &str) -> u32 {
    let input = input.replace("\n", "");
    let input = input.trim();
    let steps = input.split(',');

    let sum: u32 = steps
        .map(|str_seq| {
            str_seq
                .as_bytes()
                .iter()
                .fold(0, |state, ch| (state + *ch as u32) * 17 % 256)
        })
        .sum();

    println!("The answer to part 1 is: {}", sum);

    sum
}

fn part2(input: &str) -> u32 {
    enum Case {
        RMV,
        SET,
    }

    // split input into steps
    let input = input.replace("\n", "");
    let input = input.trim();
    let steps = input.split(',');

    // create "HASHMAP"
    let mut boxes: Vec<Vec<(&str, u32)>> = vec![vec![]; 256];

    // iterate over steps
    'steps: for step in steps {
        // parse step information
        let case;
        let new_label;
        let mut new_lens_nr = 0; 

        if let Some(caps_set) = Regex::new(r"(\w+)=(\d+)").unwrap().captures(step) {
            case = Case::SET;
            new_label = caps_set.get(1).unwrap().as_str();
            new_lens_nr = caps_set.get(2).unwrap().as_str().parse().unwrap();
        } else if let Some(caps_del) = Regex::new(r"(\w+)-").unwrap().captures(step) {
            case = Case::RMV;
            new_label = caps_del.get(1).unwrap().as_str();
        } else {
            unreachable!()
        }

        // use right box by getting hash
        let box_nr = new_label
            .as_bytes()
            .iter()
            .fold(0, |state, ch| (state + *ch as usize) * 17 % 256);

        let box_vec = &mut boxes[box_nr];

        // execute step
        match case {
            // set lens, for example: rn=1
            Case::SET => {
                // replace label if already existing
                for (label, lens_nr) in box_vec.iter_mut() {
                    if label == &new_label {
                        *lens_nr = new_lens_nr;
                        continue 'steps;
                    }
                }

                // add new label if not already existing
                box_vec.push((new_label, new_lens_nr));
            }
            // remove lens, for example: cm-
            Case::RMV => {
                // find index of label if existing
                let mut index_to_remove: Option<usize> = None;

                for (i, (label, _)) in box_vec.iter_mut().enumerate() {
                    if label == &new_label {
                        index_to_remove = Some(i);
                        break;
                    }
                }

                // remove label if found
                if let Some(i) = index_to_remove {
                    box_vec.remove(i);
                }
            }
        }
    }

    // calc focusing power
    let sum: u32 = boxes
        .iter()
        .enumerate()
        .map(|(box_nr, box_vec)| {
            box_vec
                .iter()
                .enumerate()
                .map(|(box_slot, (_, focal_length))| {
                    (box_nr as u32 + 1) * (box_slot as u32 + 1) * focal_length
                })
                .sum::<u32>()
        })
        .sum();

    println!("The answer to part 2 is: {}", sum);

    sum
}

#[test]
fn example1() {
    let example_input = "
        rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    ";

    assert_eq!(part1(example_input), 1320);
}

#[test]
fn example2() {
    let example_input = "
        rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    ";

    assert_eq!(part2(example_input), 145);
}
