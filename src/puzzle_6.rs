use regex::Regex;
use std::{
    fs::read_to_string,
    iter::zip
};

pub fn solve() -> u32 {
    let file_path = "assets/puzzle6/input.txt";
    let file_lines = read_to_string(file_path).unwrap();
    let file_lines_lines: Vec<&str> = file_lines.lines().collect();
    two(file_lines_lines) as u32
}

fn one(file_lines: Vec<&str>) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let times: Vec<u32> = re
        .captures_iter(file_lines[0])
        .map(|x| x.get(0).unwrap().as_str().parse::<u32>().unwrap())
        .collect();
    let distances: Vec<u32> = re
        .captures_iter(file_lines[1])
        .map(|x| x.get(0).unwrap().as_str().parse::<u32>().unwrap())
        .collect();

    zip(times, distances)
        .map(|pair| {
            let a0 = 1.0;
            let sol1 = math_is_cooler(-a0, a0 * (pair.0 as f64), -(pair.1 as f64)).0;
            pair.0 + 1 - 2 * (sol1.ceil() as u32)
        })
        .reduce(|acc, dist| acc * dist)
        .unwrap()
}

fn two(file_lines: Vec<&str>) -> u64 {
    let a0 = 1.0;
    let tT: u64 = 46828479;
    let dist: u64 = 347152214061471;
    let sol1 = math_is_cooler(-a0, a0 * (tT as f64), -(dist as f64)).0;
    tT + 1 - 2 * (sol1.ceil() as u64)
}

fn math_is_cooler(a: f64, b: f64, c: f64) -> (f64, f64) {
    let sol1 = (-b + (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);
    let sol2 = (-b - (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);
    (sol1, sol2)
}
