use nalgebra::DMatrix;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string, cmp::min,
};

pub fn solve() -> u64 {
    let file_path = "assets/puzzle13/input.txt";
    let file_lines = read_to_string(file_path).unwrap();
    let file_lines_lines: Vec<&str> = file_lines.lines().collect();
    one(file_lines_lines)
}

fn one(lines: Vec<&str>) -> u64{
    let mut sum = 0;
    let mut pat = Vec::<&str>::new();
    for line in lines {
        if line != "" {
            pat.push(line);
        } else {
            let mat = DMatrix::<char>::from_fn(pat.len(), pat[0].len(), |i, j| {
                pat[i].chars().nth(j).unwrap()
            });
            // println!("testing {:?}", mat);
            sum += find_reflection(mat);
            pat = Vec::<&str>::new()
        }
    }
    sum
}

fn find_reflection(pattern: DMatrix<char>) -> u64{
    let num_cols = pattern.row(0).len();
    'outer: for col_index in 0..num_cols {
        if col_index == 0 {continue;}

        for dispersal_index in 0..min(col_index, num_cols-col_index){
            if pattern.column(col_index-1-dispersal_index) != pattern.column(col_index+dispersal_index) {
                // println!("{:?} != {:?}", pattern.column(col_index-1-dispersal_index), pattern.column(col_index+dispersal_index));
                continue 'outer;
            }
        }
        println!(" --- --- Reflection at Column {} --- ---", col_index);
        return col_index as u64;
    }

    let num_rows = pattern.column(0).len();
    'outer: for row_index in 0..num_rows {
        if row_index == 0 {continue;}
        for dispersal_index in 0..min(row_index, num_rows-row_index){
            if pattern.row(row_index-1-dispersal_index) != pattern.row(row_index+dispersal_index) {
                // println!("{:?} != {:?}", pattern.row(row_index-1-dispersal_index), pattern.row(row_index+dispersal_index));
                continue 'outer;
            }
        }
        println!(" --- --- Reflection at Row {} --- ---", row_index);
        return 100*row_index as u64;
    }

    unreachable!()
}