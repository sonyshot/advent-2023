use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn solve() -> u64 {
    let file_path = "assets/puzzle15/input.txt";
    let file_lines = read_to_string(file_path).unwrap();
    let file_lines_lines: Vec<&str> = file_lines.lines().collect();
    one(file_lines_lines)
}

fn one(lines: Vec<&str>) -> u64 {
    let mut sum = 0;
    for step in lines[0].split(',') {
        sum += hash(step);
    }
    sum
}

fn hash(stringy: &str) -> u64 {
    let mut value = 0;

    for c in stringy.chars() {
        let a_value = c as u64;
        value += a_value;
        value = value*17;
        value = value%256;
    }

    value
}