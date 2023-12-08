use regex::Regex;
use std::{collections::HashMap, error::Error, fs, process::exit};

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("src/day08/input")?.parse()?;
    // part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    // parse navigation instructions and node map
    let (nav_str, mut node_str) = input.split_once("\n").unwrap();
    node_str = node_str.trim();

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum LeftRight {
        Left,
        Right,
    }

    let nav_sequence: Vec<LeftRight> = nav_str
        .chars()
        .map(|x| match x {
            'L' => LeftRight::Left,
            _ => LeftRight::Right,
        })
        .collect();

    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();
    let node_re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    for (_, [key, left_value, right_value]) in node_re.captures_iter(node_str).map(|x| x.extract())
    {
        node_map.insert(key, (left_value, right_value));
    }

    // follow the sequence
    let mut current_node = "AAA";
    let mut count = 0;

    while current_node != "ZZZ" {
        let current_instr = nav_sequence[count % nav_sequence.len()];

        current_node = {
            if current_instr == LeftRight::Left {
                node_map.get(current_node).unwrap().0
            } else {
                node_map.get(current_node).unwrap().1
            }
        };

        count += 1;
    }

    println!("The answer to part 1 is: {}", count);

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> { // TODO: unsolved because of bad performance
    // parse navigation instructions and node map
    let (nav_str, mut node_str) = input.split_once("\n").unwrap();
    node_str = node_str.trim();

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum LeftRight {
        Left,
        Right,
    }

    let nav_sequence: Vec<LeftRight> = nav_str
        .chars()
        .map(|x| match x {
            'L' => LeftRight::Left,
            _ => LeftRight::Right,
        })
        .collect();

    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();
    let node_re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    for (_, [key, left_value, right_value]) in node_re.captures_iter(node_str).map(|x| x.extract())
    {
        node_map.insert(key, (left_value, right_value));
    }

    // follow the sequence
    let mut current_nodes: Vec<&str> = node_map
        .keys()
        .filter(|x| x.ends_with("A"))
        .map(|x| *x)
        .collect();

    let mut count = 0;

    loop {
        if current_nodes.iter().all(|x| x.ends_with("Z")) {
            break;
        }

        let current_instr = &nav_sequence[count % nav_sequence.len()];

        for current_node in current_nodes.iter_mut() {
            *current_node = {
                if current_instr == &LeftRight::Left {
                    node_map.get(current_node).unwrap().0
                } else {
                    node_map.get(current_node).unwrap().1
                }
            };
        }

        count += 1;
        dbg!(count);
    }

    println!("The answer to part 2 is: {}", count);

    Ok(())
}
