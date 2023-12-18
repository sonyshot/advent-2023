use nalgebra::DMatrix;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn solve() -> u64 {
    let file_path = "assets/puzzle14/input.txt";
    let file_lines = read_to_string(file_path).unwrap();
    let file_lines_lines: Vec<&str> = file_lines.lines().collect();
    one(file_lines_lines)
}

fn one(lines: Vec<&str>) -> u64 {
    let mut sum = 0;
    let mat = DMatrix::<char>::from_fn(lines.len(), lines[0].len(), |i, j| {
        lines[i].chars().nth(j).unwrap()
    });

    for col in mat.column_iter() {
        let num_rows = col.len();
        let mut strut_index = 0;
        let mut num_rocks = 0;
        for (i, el) in col.iter().enumerate() {
            if *el == '#'{
                num_rocks = 0;
                strut_index = i+1;
            } else if *el == 'O' {
                sum += num_rows-strut_index-num_rocks;
                num_rocks += 1;
            }
        }
    }

    sum as u64
}
