use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    error::Error,
    fs,
    ops::Range,
};

use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("src/day05/input")?.parse()?;
    part1(&input)?;
    part2_pso(&input)?;

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

    let (_seed_to_use, nearest_location) = locations.iter().min_by(|x, y| x.1.cmp(&y.1)).unwrap();

    println!("The answer for part 1 is: {}", nearest_location);

    Ok(())
}

fn part2_pso(input: &str) -> Result<(), Box<dyn Error>> {
    // find seed numbers in input and parse them as u32
    let raw_seeds: Vec<u64> = Regex::new(r"seeds:(.*)\n")
        .unwrap()
        .captures(input)
        .unwrap()
        .get(1) // index 1 is the capture group in the regex
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let seed_ranges: Vec<&[u64]> = raw_seeds.chunks(2).collect();

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

    // particle swarm optimization for each seed range
    struct Particle {
        x: u64,
        best_x: (u64, u64),
        v: i64,
    }

    let cost_function = |seed| {
        let mut i = seed;

        'map_loop: for map in &mega_map {
            for mapping_range in map {
                // current number is in a gap between ranges
                // => number stays unchanged by this map
                if mapping_range.source_range.start > i {
                    continue 'map_loop;
                }

                // found in a mapping range, convert number and go to next map
                if mapping_range.source_range.contains(&i) {
                    let offset = i - mapping_range.source_range.start;
                    i = mapping_range.dest_start + offset;
                    continue 'map_loop;
                }
            }
            // not found in a mapping range or gap, reached end
            // => number stays unchanged by this map
        }

        i
    };

    let mut best_location_per_range: Vec<(u64, u64)> = vec![];
    let mut rng = rand::thread_rng();

    for seed_range in seed_ranges {
        let seed_range_start = seed_range[0];
        let seed_range_length = seed_range[1];
        let seed_range_end = seed_range_start + seed_range_length;
        let position_dist = Uniform::from(seed_range_start..seed_range_end);
        let velocity_dist = Uniform::from(-(seed_range_length as i64)..(seed_range_length as i64));

        let mut particles: Vec<Particle> = vec![];
        let mut best_swarm_position = (0u64, u32::MAX as u64);
        let mut steps_best_swarm_position_unchanged = 0;
        let nr_particles = 10000;

        // init particles
        for _ in 0..nr_particles {
            let x = position_dist.sample(&mut rng);
            let x_cost = cost_function(x);
            let v = velocity_dist.sample(&mut rng);

            let particle = Particle {
                x,
                best_x: (x, x_cost),
                v,
            };

            particles.push(particle);

            if x_cost < best_swarm_position.1 {
                best_swarm_position = (x, x_cost);
            }
        }

        // search optimum
        while steps_best_swarm_position_unchanged < 10 {
            for particle in &mut particles {
                let r_p: f32 = rng.gen();
                let r_g: f32 = rng.gen();
                let diff_particle = particle
                    .best_x
                    .0
                    .checked_sub(particle.x)
                    .unwrap_or_default();
                let diff_swarm = best_swarm_position
                    .0
                    .checked_sub(particle.x)
                    .unwrap_or_default();

                let new_velocity = (0.5 * particle.v as f32 // inertia
                    + 1.5 * r_p * (diff_particle) as f32 // towards particle best
                    + 2.5 * r_g * (diff_swarm) as f32) // towards swarm best
                    as i64;
                let mut new_position =
                    u64::max(seed_range_start, (particle.x as i64 + new_velocity) as u64);
                new_position = u64::min(seed_range_end - 1, new_position);

                let cost_new_position = cost_function(new_position);

                particle.v = new_velocity;
                particle.x = new_position;

                if cost_new_position < particle.best_x.1 {
                    particle.best_x = (new_position, cost_new_position);
                }

                if cost_new_position < best_swarm_position.1 {
                    best_swarm_position = (new_position, cost_new_position);
                    steps_best_swarm_position_unchanged = 0;
                } else {
                    steps_best_swarm_position_unchanged += 1;
                }
            }
        }

        best_location_per_range.push(best_swarm_position);
    }

    let (best_candidate_pso_seed, _best_candidate_pso_location) = best_location_per_range
        .iter()
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();

    // fine search
    let start = best_candidate_pso_seed - 1_000_000;
    let end = best_candidate_pso_seed + 1_000_000;
    let mut best_location = u64::MAX;

    for seed in start..end {
        let location = cost_function(seed);
        best_location = u64::min(location, best_location);
    }

    println!("The answer for part 2 is: {}", best_location);

    Ok(())
}

fn part2_naive(input: &str) -> Result<(), Box<dyn Error>> {
    // find seed numbers in input and parse them as u32
    let raw_seeds: Vec<u64> = Regex::new(r"seeds:(.*)\n")
        .unwrap()
        .captures(input)
        .unwrap()
        .get(1) // index 1 is the capture group in the regex
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let seed_pairs: Vec<&[u64]> = raw_seeds.chunks(2).collect();
    let mut seeds: HashSet<u64> = HashSet::new();

    for seed_pair in seed_pairs {
        let seed_range_start = seed_pair[0];
        let seed_range_length = seed_pair[1];
        let seed_range_end = seed_range_start + seed_range_length;

        for seed in seed_range_start..seed_range_end {
            seeds.insert(seed);
        }
    }

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
    let mut seeds_locations: HashMap<u64, u64> = HashMap::new();

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
        seeds_locations.insert(seed, current_number);
    }

    let (_seed_to_use, nearest_location) =
        seeds_locations.iter().min_by(|x, y| x.1.cmp(&y.1)).unwrap();

    println!("The answer for part 2 is: {}", nearest_location);

    Ok(())
}

#[derive(PartialEq, Eq)]
struct ReverseMappingRange {
    dest_range: Range<u64>,
    source_start: u64,
}

impl PartialOrd for ReverseMappingRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.dest_range.start.partial_cmp(&other.dest_range.start)
    }
}

impl Ord for ReverseMappingRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dest_range.start.cmp(&other.dest_range.start)
    }
}
fn part2_reverse_search(input: &str) -> Result<(), Box<dyn Error>> {
    // find seed numbers in input and parse them as u32
    let raw_seeds: Vec<u64> = Regex::new(r"seeds:(.*)\n")
        .unwrap()
        .captures(input)
        .unwrap()
        .get(1) // index 1 is the capture group in the regex
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut seed_pairs: Vec<&[u64]> = raw_seeds.chunks(2).collect();
    seed_pairs.sort();

    // split off remaining input to process the maps
    let rem_input_with_maps = input.split_once("map:").unwrap().1;

    // split remaining input into maps
    let maps_str: Vec<&str> = Regex::new(r"\n.*map:.*\n") //
        .unwrap()
        .split(rem_input_with_maps)
        .map(|x| x.trim())
        .collect();

    // process the maps from str into data structure
    let mut mega_map: Vec<Vec<ReverseMappingRange>> = vec![];

    for map_str in maps_str {
        let mut map: Vec<ReverseMappingRange> = vec![];

        for line in map_str.lines() {
            // parse numbers in line
            let x: Vec<u64> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            // create MappingRange and add to map vector
            let dest_start = x[0];
            let range_len = x[2];
            let dest_end = dest_start + range_len;
            map.push(ReverseMappingRange {
                dest_range: dest_start..dest_end,
                source_start: x[1],
            });
        }
        map.sort();
        mega_map.push(map);
    }

    // iterate over possible locations starting from lowest
    // determine if necessary seeds are available
    'location_loop: for location in 0..u64::MAX {
        let mut i = location;

        'map_loop: for map in mega_map.iter().rev() {
            for reverse_mapping_range in map {
                // current number is in a gap between ranges
                // => number stays unchanged by this map
                if reverse_mapping_range.dest_range.start > i {
                    continue 'map_loop;
                }

                // found in a mapping range, convert number and go to next map
                if reverse_mapping_range.dest_range.contains(&i) {
                    let offset = i - reverse_mapping_range.dest_range.start;
                    i = reverse_mapping_range.source_start + offset;
                    continue 'map_loop;
                }
            }
            // not found in a mapping range or gap, reached end
            // => number stays unchanged by this map
        }

        // end of processing, check if necessary seed is available
        for seed_pair in &seed_pairs {
            let start = seed_pair[0];
            let length = seed_pair[1];
            let end = start + length;

            // necessary seed in gap, not available, test next location
            if i < seed_pair[0] {
                continue 'location_loop;
            }

            // found the location
            if (start..end).contains(&i) {
                println!("The answer for part 2 is: {}", location);
                break 'location_loop;
            }
        }
    }

    Ok(())
}
