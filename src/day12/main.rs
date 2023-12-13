use regex::{Match, Matches, Regex};
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
    // part2(&input);

    Ok(())
}

fn part1(input: &str) -> usize {
    let input = input.trim();
    let mut overall_sum = 0;

    for line in input.lines() {
        let line = line.trim();

        // parse line
        let (springs, nums_str) = line.split_once(" ").unwrap();
        let nums: Vec<usize> = nums_str.split(",").map(|x| x.parse().unwrap()).collect();

        // find every valid combination while replacing question marks
        let combinations = create_valid_combinations(vec![springs.to_owned()], &nums);

        overall_sum += combinations.len();
    }

    println!("The answer to part 1 is: {}", overall_sum);

    overall_sum
}

fn create_valid_combinations(v: Vec<String>, nums: &Vec<usize>) -> Vec<String> {
    if v[0].contains('?') {
        let v2 = v
            .iter()
            .flat_map(|s| {
                let mut cands: Vec<String> = vec![];
                let cand1 = s.replacen("?", "#", 1);
                let cand2 = s.replacen("?", ".", 1);

                if check_for_plausibility(&cand1, nums) {
                    cands.push(cand1);
                }

                if check_for_plausibility(&cand2, nums) {
                    cands.push(cand2);
                }

                cands
            })
            .collect();
        return create_valid_combinations(v2, nums);
    } else {
        return v;
    };
}

fn check_for_plausibility(s: &str, nums: &Vec<usize>) -> bool {
    let mut question_mark_index = s.find('?').unwrap_or(s.len());
    if question_mark_index != s.len() {
        question_mark_index = s[0..question_mark_index]
            .rfind('.')
            .unwrap_or(0);
    }
    let re = Regex::new(r"#+").unwrap();
    let matches: Vec<Match> = re.find_iter(&s[0..question_mark_index]).collect();

    if question_mark_index == s.len() && matches.len() != nums.len() {
        return false;
    }

    for (spring_group, num) in matches.iter().zip(nums) {
        if spring_group.len() != *num {
            return false;
        }
    }

    true
}

#[test]
fn examples() {
    let example_input = "
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    ";

    assert_eq!(part1(example_input), 21);
}
