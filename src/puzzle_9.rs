use regex::Regex;
use std::{
    collections::{hash_map::Keys, HashMap, HashSet},
    fs::read_to_string,
};
use nalgebra::{DMatrix, DVector, SMatrix};

pub fn solve() -> i64 {
    let file_path = "assets/puzzle9/input.txt";
    let file_lines = read_to_string(file_path).unwrap();
    let file_lines_lines: Vec<&str> = file_lines.lines().collect();
    one(file_lines_lines)
}

fn one(lines: Vec<&str>) -> i64 {
    let mut sum = 0;
    lines.iter().for_each(|el| {
        let ys: DVector<f64> = DVector::from_vec(el.split(' ').map(|numba| numba.parse::<f64>().unwrap()).collect());
        let xs = DMatrix::<f64>::from_fn(ys.len(), ys.len(), |i, j| {
            let (i, j) = (i as f64, j as i32);
            i.powi(j)
        });
        // println!("{}",xs);
        // println!("{}", ys);
        let coefs = xs.lu().solve(&ys).unwrap();

        let x = -1;
        let sol = coefs.dot(&DVector::<f64>::from_fn(ys.len(), |j, _| (x as f64).powi(j as i32)));
        sum += coefs.dot(&DVector::<f64>::from_fn(ys.len(), |j, _| (x as f64).powi(j as i32))).round() as i64;
    });

    sum
}

