use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn solve() -> u64 {
    let file_path = "assets/puzzle11/input.txt";
    let file_lines = read_to_string(file_path).unwrap();
    let file_lines_lines: Vec<&str> = file_lines.lines().collect();
    one(file_lines_lines)
}

fn one(lines: Vec<&str>) -> u64 {
    let mut sum = 0;
    let mut galaxies = HashSet::<(usize, usize)>::new();
    let mut galaxy_rows = HashSet::<usize>::new();
    let mut galaxy_cols = HashSet::<usize>::new();

    for (j, line) in lines.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.insert((i, j));
                galaxy_rows.insert(j);
                galaxy_cols.insert(i);
            }
        }
    }

    let mut gal_rows: Vec<_> = galaxy_rows.iter().collect();
    gal_rows.sort();
    let mut gal_cols: Vec<_> = galaxy_cols.iter().collect();
    gal_cols.sort();

    println!("Full Rows: {:?}", gal_rows);
    println!("Full Columns: {:?}", gal_cols);

    let mut prev_galaxies = HashSet::<(usize, usize)>::new();

    for galaxy in galaxies {
        for gal2 in &prev_galaxies {
            let col_dist = galaxy.0.abs_diff(gal2.0);
            let row_dist = galaxy.1.abs_diff(gal2.1);

            let extra_rows = row_dist
                - gal_rows
                    .iter()
                    .filter(|el| {
                        if gal2.1 > galaxy.1 {
                            return galaxy.1 <= ***el && el < &&&gal2.1;
                        } else {
                            return gal2.1 <= ***el && el < &&&galaxy.1;
                        }
                    })
                    .collect::<Vec<_>>()
                    .len();
            let extra_cols = col_dist
                - gal_cols
                    .iter()
                    .filter(|el| {
                        if gal2.0 > galaxy.0 {
                            return galaxy.0 <= ***el && el < &&&gal2.0;
                        } else {
                            return gal2.0 <= ***el && el < &&&galaxy.0;
                        }
                    })
                    .collect::<Vec<_>>()
                    .len();

            // println!("G1: {:?}", galaxy);
            // println!("G2: {:?}", gal2);
            // println!("col dist {:?}", col_dist);
            // println!("row dist {:?}", row_dist);
            // println!("extra_rows {:?}", extra_rows);
            // println!("extra_cols {:?}", extra_cols);

            sum += row_dist + extra_rows + col_dist + extra_cols;
        }
        prev_galaxies.insert(galaxy);
    }

    sum as u64
}
