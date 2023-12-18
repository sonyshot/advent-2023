use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn solve() -> u64 {
    let file_path = "assets/puzzle12/input.txt";
    let file_lines = read_to_string(file_path).unwrap();
    let file_lines_lines: Vec<&str> = file_lines.lines().collect();
    one(file_lines_lines)
}

fn one(lines: Vec<&str>) -> u64 {
    let mut sum = 0;

    for line in lines {
        //cmon gordon... this can be regex stuff
        let temp: Vec<&str> = line.split(' ').collect();
        let (issue, nums) = (temp[0], temp[1]);

        let parsed_nums: Vec<u64> = nums.split(',').map(|el| el.parse::<u64>().unwrap()).collect();

        let q_count = issue.chars().filter(|c| c == &'?').count();

        let mut lower_sum = 0;
        for trial in 0..2_u64.pow(q_count as u32) {
            if is_valid(create_permutation(issue, trial), &parsed_nums) {
                sum += 1;
                lower_sum += 1;
            }
        }
        println!("issue: {} - sum: {}", issue, lower_sum);
    }

    sum
}

fn two(lines: Vec<&str>) -> u64 {
    let mut sum = 0;

    for line in lines {
        //cmon gordon... this can be regex stuff
        let temp: Vec<&str> = line.split(' ').collect();
        let (issue, nums) = (temp[0], temp[1]);

        let parsed_nums: Vec<u64> = nums.split(',').map(|el| el.parse::<u64>().unwrap()).collect();

        let issue_copies: Vec<&str> = (0..5).map(|_| issue).collect();
        let real_issue = issue_copies.join("?");
        let trimmed = real_issue.trim_matches('.');

        let q_count = real_issue.chars().filter(|c| c == &'?').count();
        let real_nums = parsed_nums.repeat(5);

        for trial in 0..2_u64.pow(q_count as u32) {
            if is_valid(create_permutation(&trimmed, trial), &real_nums) {
                sum += 1;
            }
        }
    }

    sum
}

fn reduce_complexity(line: String, config: &[u64]) -> String {
    let spl = line.split('.');

    for (i, section) in spl.enumerate() {

    }

    String::new()
}

fn create_permutation(line: &str, perm: u64) -> String {
    //assume perm is a binary representing a functional/broken spring on each bit
    let mut output = String::with_capacity(line.len());

    let mut q_count = 0;

    for c in line.chars() {
        if c == '?' {
            if perm & (1 << q_count) != 0 {
                output.push('.');
            } else {
                output.push('#');
            }
            q_count += 1;
        } else {
            output.push(c);
        }
    }

    // println!("{} with {} is {}", line, perm, output);

    output
}

fn is_valid(line: String, config: &[u64]) -> bool {
    let mut config_iter = config.iter();

    let mut next_seq = config_iter.next();

    let spl = line.split('.');

    for group in spl {
        // print!("{:?},", group);
        if group.len() == 0 {
            continue;
        } else {
            if let Some(length) = next_seq {
                if *length == group.len() as u64 {
                    next_seq = config_iter.next();
                } else {
                    // println!("Failed: following group size didn't match {} != {}", length, group.len());
                    return false;
                }
            } else {
                // println!("Failed: ran out of nonzero groups");
                return false;
            }
        }
    }
    if next_seq != None {
        // println!("Failed: too many nonzero groups");
        return false;
    }    
    true
}
