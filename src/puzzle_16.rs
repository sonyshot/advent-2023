use nalgebra::DMatrix;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    ops::Add,
};

pub fn solve() -> u64 {
    let file_path = "assets/puzzle16/input.txt";
    let file_lines = read_to_string(file_path).unwrap();
    let file_lines_lines: Vec<&str> = file_lines.lines().collect();
    two(file_lines_lines)
}

fn one(lines: Vec<&str>) -> u64 {
    let mut sum = 0;
    let mat = DMatrix::<char>::from_fn(lines.len(), lines[0].len(), |j, i| {
        lines[i].chars().nth(j).unwrap()
    });

    let mut energized_tiles = HashSet::<(usize, usize, Direction)>::new();

    follow_path(&mut energized_tiles, &mat, (0, 0, Direction::Right));

    let mut final_tiles = HashSet::<(usize, usize)>::new();
    energized_tiles.iter().for_each(|el| {
        final_tiles.insert((el.0, el.1));
    });

    final_tiles.len() as u64
}

fn two(lines: Vec<&str>) -> u64 {
    let mut sum = 0;
    let mat = DMatrix::<char>::from_fn(lines.len(), lines[0].len(), |j, i| {
        lines[i].chars().nth(j).unwrap()
    });
    let num_rows = mat.column(0).len();
    let num_cols = mat.row(0).len();

    // println!("===========FROM THE LEFT==========");
    for start in 0..num_rows {
        let mut energized_tiles = HashSet::<(usize, usize, Direction)>::new();
        follow_path(&mut energized_tiles, &mat, (0, start, Direction::Right));

        let mut final_tiles = HashSet::<(usize, usize)>::new();
        energized_tiles.iter().for_each(|el| {
            final_tiles.insert((el.0, el.1));
        });
        if final_tiles.len() > sum {
            sum = final_tiles.len();
        }
    }

    // println!("===========FROM THE Right==========");
    for start in 0..num_rows {
        let mut energized_tiles = HashSet::<(usize, usize, Direction)>::new();
        follow_path(&mut energized_tiles, &mat, (num_cols-1, start, Direction::Left));

        let mut final_tiles = HashSet::<(usize, usize)>::new();
        energized_tiles.iter().for_each(|el| {
            final_tiles.insert((el.0, el.1));
        });
        if final_tiles.len() > sum {
            sum = final_tiles.len();
        }
    }

    // println!("===========FROM Above==========");
    for start in 0..num_cols {
        let mut energized_tiles = HashSet::<(usize, usize, Direction)>::new();
        follow_path(&mut energized_tiles, &mat, (start, 0, Direction::Down));

        let mut final_tiles = HashSet::<(usize, usize)>::new();
        energized_tiles.iter().for_each(|el| {
            final_tiles.insert((el.0, el.1));
        });
        if final_tiles.len() > sum {
            sum = final_tiles.len();
        }
    }

    // println!("===========FROM BELOW==========");
    for start in 0..num_cols {
        let mut energized_tiles = HashSet::<(usize, usize, Direction)>::new();
        follow_path(&mut energized_tiles, &mat, (start, num_rows-1, Direction::Up));

        let mut final_tiles = HashSet::<(usize, usize)>::new();
        energized_tiles.iter().for_each(|el| {
            final_tiles.insert((el.0, el.1));
        });
        if final_tiles.len() > sum {
            sum = final_tiles.len();
        }
    }

    sum as u64
}

fn follow_path(
    energy: &mut HashSet<(usize, usize, Direction)>,
    mirrors: &DMatrix<char>,
    start: (usize, usize, Direction),
) {
    let mut pos = start;
    loop {
        if energy.contains(&pos) {
            // println!("Done at {:?}", pos);
            break;
        }
        // println!("adding {:?}", pos);
        energy.insert(pos.clone());
        let delta = next_direction(mirrors.get((pos.0, pos.1)).unwrap(), pos.2);
        if let Some(second) = delta.1 {
            match second {
                Direction::Up => {
                    if let Some(d) = pos.1.checked_sub(1) {
                        follow_path(energy, mirrors, (pos.0, d, Direction::Up));
                    }
                }
                Direction::Down => {
                    if pos.1 + 1 < mirrors.column(0).len() {
                        follow_path(energy, mirrors, (pos.0, pos.1 + 1, Direction::Down));
                    }
                }
                Direction::Left => {
                    if let Some(d) = pos.0.checked_sub(1) {
                        follow_path(energy, mirrors, (d, pos.1, Direction::Left));
                    }
                }
                Direction::Right => {
                    if pos.0 + 1 < mirrors.row(0).len() {
                        follow_path(energy, mirrors, (pos.0 + 1, pos.1, Direction::Right));
                    }
                }
            }
        }
        match delta.0 {
            Direction::Up => {
                if let Some(d) = pos.1.checked_sub(1) {
                    pos = (pos.0, d, Direction::Up);
                } else {
                    break;
                }
            }
            Direction::Down => {
                if pos.1 + 1 >= mirrors.column(0).len() {
                    break;
                }
                pos = (pos.0, pos.1 + 1, Direction::Down);
            }
            Direction::Left => {
                if let Some(d) = pos.0.checked_sub(1) {
                    pos = (d, pos.1, Direction::Left);
                } else {
                    break;
                }
            }
            Direction::Right => {
                // println!("right");
                if pos.0 + 1 >= mirrors.row(0).len() {
                    break;
                }
                // println!("ok");
                // println!("before {:?}", pos);
                pos = (pos.0 + 1, pos.1, Direction::Right);
                // println!("after {:?}", pos);
            }
        }
    }
}

fn next_direction(tile: &char, dir: Direction) -> (Direction, Option<Direction>) {
    // println!("at tile {:?}", tile);
    match *tile {
        '/' => match dir {
            Direction::Right => (Direction::Up, None),
            Direction::Left => (Direction::Down, None),
            Direction::Down => (Direction::Left, None),
            Direction::Up => (Direction::Right, None),
        },
        '\\' => match dir {
            Direction::Right => (Direction::Down, None),
            Direction::Left => (Direction::Up, None),
            Direction::Down => (Direction::Right, None),
            Direction::Up => (Direction::Left, None),
        },
        '|' => match dir {
            Direction::Right => (Direction::Down, Some(Direction::Up)),
            Direction::Left => (Direction::Down, Some(Direction::Up)),
            Direction::Down => (Direction::Down, None),
            Direction::Up => (Direction::Up, None),
        },
        '-' => match dir {
            Direction::Right => (Direction::Right, None),
            Direction::Left => (Direction::Left, None),
            Direction::Down => (Direction::Right, Some(Direction::Left)),
            Direction::Up => (Direction::Right, Some(Direction::Left)),
        },
        _ => (dir, None),
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
