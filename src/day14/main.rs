use indexmap::IndexSet;
use regex::Regex;
use reqwest::header::COOKIE;
use std::{error::Error, fs};

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

fn part1(input: &str) -> usize {
    let input = input.trim();

    // build grid and move rounded rocks
    let mut grid: Vec<Vec<char>> = vec![];
    let width = input.lines().nth(0).unwrap().chars().count();
    let mut next_free_row_per_col: Vec<usize> = vec![0; width];
    let mut line_count = 0;

    for (y, line) in input.lines().enumerate() {
        grid.push(vec![]);
        line_count += 1;

        for (x, ch) in line.trim().chars().enumerate() {
            match ch {
                'O' => {
                    let move_to = next_free_row_per_col[x];
                    if move_to < y {
                        grid[move_to][x] = 'O';
                        grid[y].push('.');
                        next_free_row_per_col[x] = move_to + 1;
                    } else {
                        grid[y].push('O');
                        next_free_row_per_col[x] = y + 1;
                    }
                }
                '#' => {
                    grid[y].push('#');
                    next_free_row_per_col[x] = y + 1;
                }
                '.' => {
                    grid[y].push('.');
                }
                _ => {}
            }
        }
    }

    // calc total load
    let mut total_load = 0;

    for (y, line) in grid.iter().enumerate() {
        for ch in line {
            if *ch == 'O' {
                total_load += line_count - y;
            }
        }
    }

    println!("The answer to part 1 is: {}", total_load);

    total_load
}

fn part2(input: &str) -> usize {
    let input = input.trim();
    const N_CYCLES: usize = 1_000_000_000;

    // caching
    let mut cache: IndexSet<Vec<Vec<char>>> = IndexSet::new();

    // build initial grid
    let mut grid: Vec<Vec<char>> = vec![];
    let mut final_grid: &Vec<Vec<char>> = &vec![];

    for line in input.lines() {
        grid.push(line.trim().chars().collect());
    }

    // cycles
    for current_cycle in 0..N_CYCLES {
        // check if already seen this grid config
        // => done, from now on everything will repeat
        if let Some(cycle_start_repetition) = cache.get_index_of(&grid) {
            // find repetition length and remaining cycles
            let len_rep = current_cycle - cycle_start_repetition;
            let rem_cycles = N_CYCLES - current_cycle;

            // find final index
            let offset = rem_cycles % len_rep;
            let final_index = cycle_start_repetition + offset;

            // find final config
            final_grid = cache.get_index(final_index).unwrap();
            break;
        }

        // insert grid into cache
        cache.insert(grid.clone());

        // roll north
        for x in 0..grid[0].len() {
            let mut target_y = 0usize;

            for y in 0..grid.len() {
                match grid[y][x] {
                    'O' => {
                        if target_y < y {
                            grid[target_y][x] = 'O';
                            grid[y][x] = '.';
                        }
                        target_y += 1;
                    }
                    '#' => {
                        target_y = y + 1;
                    }
                    _ => {}
                }
            }
        }

        // roll west
        for y in 0..grid.len() {
            let mut target_x = 0usize;

            for x in 0..grid[0].len() {
                match grid[y][x] {
                    'O' => {
                        if target_x < x {
                            grid[y][target_x] = 'O';
                            grid[y][x] = '.';
                        }
                        target_x += 1;
                    }
                    '#' => {
                        target_x = x + 1;
                    }
                    _ => {}
                }
            }
        }

        // roll south
        for x in 0..grid[0].len() {
            let mut target_y = grid.len() - 1;

            for y in (0..grid.len()).rev() {
                match grid[y][x] {
                    'O' => {
                        if target_y > y {
                            grid[target_y][x] = 'O';
                            grid[y][x] = '.';
                        }
                        if target_y > 0 {
                            target_y -= 1;
                        }
                    }
                    '#' if y > 0 => {
                        target_y = y - 1;
                    }
                    _ => {}
                }
            }
        }

        // roll east
        for y in 0..grid.len() {
            let mut target_x = grid[0].len() - 1;

            for x in (0..grid[0].len()).rev() {
                match grid[y][x] {
                    'O' => {
                        if target_x > x {
                            grid[y][target_x] = 'O';
                            grid[y][x] = '.';
                        }
                        if target_x > 0 {
                            target_x -= 1;
                        }
                    }
                    '#' if x > 0 => {
                        target_x = x - 1;
                    }
                    _ => {}
                }
            }
        }
    }

    // calc total load
    let mut total_load = 0;

    for (y, line) in final_grid.iter().enumerate() {
        for ch in line {
            if *ch == 'O' {
                total_load += final_grid.len() - y;
            }
        }
    }

    println!("The answer to part 2 is: {}", total_load);

    total_load
}

#[test]
fn example1() {
    let example_input = "
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    ";

    assert_eq!(part1(example_input), 136);
}

#[test]
fn example2() {
    let example_input = "
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    ";

    assert_eq!(part2(example_input), 64);
}
