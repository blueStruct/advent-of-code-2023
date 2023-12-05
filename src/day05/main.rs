use std::{cmp::Ordering, error::Error, fs, ops::Range};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("src/day05/input")?.parse()?;
    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(PartialEq, Eq)]
struct MappingRange {
    source_range: Range<u64>,
    dest_start: u64,
}

impl PartialOrd for MappingRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.source_range
            .start
            .partial_cmp(&other.source_range.start)
    }
}

impl Ord for MappingRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.source_range.start.cmp(&other.source_range.start)
    }
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    // find seed numbers in input and parse them as u32
    let seeds: Vec<u64> = Regex::new(r"seeds:(.*)\n")
        .unwrap()
        .captures(input)
        .unwrap()
        .get(1) // index 1 is the capture group in the regex
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // split off remaining input to process the maps
    let rem_input_with_maps = input.split_once("map:").unwrap().1;

    // split remaining input into maps
    let maps_str: Vec<&str> = Regex::new(r"\n.*map:.*\n") //
        .unwrap()
        .split(rem_input_with_maps)
        .map(|x| x.trim())
        .collect();

    // process the maps from str into data structure
    let mut mega_map: Vec<Vec<MappingRange>> = vec![];

    for map_str in maps_str {
        let mut map: Vec<MappingRange> = vec![];

        for line in map_str.lines() {
            // parse numbers in line
            let x: Vec<u64> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            // create MappingRange and add to map vector
            let source_start = x[1];
            let range_len = x[2];
            let source_end = source_start + range_len;
            map.push(MappingRange {
                source_range: source_start..source_end,
                dest_start: x[0],
            });
        }
        map.sort();
        mega_map.push(map);
    }

    // find the locations, vec of (seed, location)
    let mut locations: Vec<(u64, u64)> = vec![];

    for seed in seeds {
        let mut current_number = seed;

        'map_loop: for map in &mega_map {
            for mapping_range in map {
                // current number is in a gap between ranges
                // => number stays unchanged by this map
                if mapping_range.source_range.start > current_number {
                    continue 'map_loop;
                }

                // found in a mapping range, convert number and go to next map
                if mapping_range.source_range.contains(&current_number) {
                    let offset = current_number - mapping_range.source_range.start;
                    current_number = mapping_range.dest_start + offset;
                    continue 'map_loop;
                }
            }
            // not found in a mapping range or gap, reached end
            // => number stays unchanged by this map
        }

        // end of processing, found location
        locations.push((seed, current_number));
    }

    let (seed_to_use, nearest_location) = locations.iter().min_by(|x, y| x.1.cmp(&y.1)).unwrap();

    println!("The answer for part 1 is: {}", nearest_location);

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
