use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("src/day10/input")?.parse()?;
    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    // definitions
    #[derive(Debug)]
    enum Dir {
        North,
        East,
        South,
        West,
    }

    fn go(current_pos: &mut (usize, usize), dir: Dir) {
        match dir {
            Dir::South => current_pos.0 += 1,
            Dir::North => current_pos.0 -= 1,
            Dir::East => current_pos.1 += 1,
            Dir::West => current_pos.1 -= 1,
        }
    }

    // inits
    let mut grid: Vec<Vec<char>> = vec![];
    let mut start = (0, 0);
    let mut pipe_length = 1;

    // parse input into grid and find start
    for (y, line) in input.lines().enumerate() {
        grid.push(
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (y, x);
                    }
                    c
                })
                .collect(),
        );
    }

    // find next pipe from start and move
    let mut current_pos = start;
    let mut came_from;

    // can we go south?
    if ['|', 'J', 'L'].contains(&grid[current_pos.0 + 1][current_pos.1]) {
        go(&mut current_pos, Dir::South);
        came_from = Dir::North;
    // can we go east?
    } else if ['-', 'J', '7'].contains(&grid[current_pos.0][current_pos.1 + 1]) {
        go(&mut current_pos, Dir::East);
        came_from = Dir::West;
    // north and west remaining, go north
    } else {
        go(&mut current_pos, Dir::North);
        came_from = Dir::South;
    }

    pipe_length += 1;

    // follow the pipe
    let mut endless_counter = 0; // to avoid infinite loop, just in case

    while endless_counter < 1_000_000 {
        endless_counter += 1;

        // stop condition
        if current_pos == start {
            break;
        }

        // get current element
        let current_element = grid[current_pos.0][current_pos.1];

        // move
        pipe_length += 1;

        match (current_element, came_from) {
            ('|', Dir::North) => {
                go(&mut current_pos, Dir::South);
                came_from = Dir::North;
            }
            ('|', Dir::South) => {
                go(&mut current_pos, Dir::North);
                came_from = Dir::South;
            }
            ('-', Dir::West) => {
                go(&mut current_pos, Dir::East);
                came_from = Dir::West;
            }
            ('-', Dir::East) => {
                go(&mut current_pos, Dir::West);
                came_from = Dir::East;
            }
            ('L', Dir::North) => {
                go(&mut current_pos, Dir::East);
                came_from = Dir::West;
            }
            ('L', Dir::East) => {
                go(&mut current_pos, Dir::North);
                came_from = Dir::South;
            }
            ('J', Dir::North) => {
                go(&mut current_pos, Dir::West);
                came_from = Dir::East;
            }
            ('J', Dir::West) => {
                go(&mut current_pos, Dir::North);
                came_from = Dir::South;
            }
            ('7', Dir::South) => {
                go(&mut current_pos, Dir::West);
                came_from = Dir::East;
            }
            ('7', Dir::West) => {
                go(&mut current_pos, Dir::South);
                came_from = Dir::North;
            }
            ('F', Dir::South) => {
                go(&mut current_pos, Dir::East);
                came_from = Dir::West;
            }
            ('F', Dir::East) => {
                go(&mut current_pos, Dir::South);
                came_from = Dir::North;
            }
            _ => unreachable!(),
        }
    }

    println!("The answer to part 1 is: {}", pipe_length / 2);

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
