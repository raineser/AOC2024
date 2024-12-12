use anyhow::{Context, self};
use std::{collections::HashMap, fs, hash::Hash, io::{BufRead, BufReader}};
use std::cmp;

struct Data {
    grid: HashMap<(i32,i32), char>,
    rows: i32,
    cols: i32
}

impl Data {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day4.txt")?;
        let reader = BufReader::new(file);
        let mut grid = HashMap::new();
        let mut rows = 0;
        let mut cols = 0;
        for (row, line) in reader.lines().enumerate() {
            let line = line?;
            for (col, c) in line.chars().enumerate() {
                grid.insert((row as i32, col as i32), c);
                cols = cmp::max(cols, col);
            }
            rows = cmp::max(rows, row);
        }
        Ok(Self{grid: grid, rows: rows as i32 + 1, cols: cols as i32 + 1})
    }
}

pub fn part_one() -> anyhow::Result<i32> {
    let data = Data::parse_input()?;
    let mut res = 0;
    res += rows(&data);
    res += cols(&data);
    res += diag_left(&data);
    res += diag_right(&data);
    Ok(res)
}

fn rows(data: &Data) -> i32 {
    let mut res = 0;
    for i in 0..data.rows {
        for j in 0..data.cols {
            match data.grid.get(&(i,j)) {
                Some('X') => {
                    let remaining = ['M', 'A', 'S'];
                    let valid = (j+1..j+4).zip(remaining.iter()).all(|(k, c)| {
                        match data.grid.get(&(i, k)) {
                            Some(entry) if entry == c => {true},
                            _ => {false}
                        }
                    });
                    if valid {
                        
                        res += 1;
                    }
                }, 
                Some('S') => {
                    let remaining = ['A', 'M', 'X'];
                    let valid = (j+1..j+4).zip(remaining.iter()).all(|(k, c)| {
                        match data.grid.get(&(i, k)) {
                            Some(entry) if entry == c => {true},
                            _ => {false}
                        }
                    });
                    if valid {
                        
                        res += 1;
                    }
                }, 
                _ => {}
            }
        }
    }
    res
}

fn cols(data: &Data) -> i32 {
    let mut res = 0;
    for j in 0..data.cols {
        for i in 0..data.rows {
            match data.grid.get(&(i,j)) {
                Some('X') => {
                    let remaining = ['M', 'A', 'S'];
                    let valid = (i+1..i+4).zip(remaining.iter()).all(|(k, c)| {
                        match data.grid.get(&(k, j)) {
                            Some(entry) if entry == c => {true},
                            _ => {false}
                        }
                    });
                    if valid {
                        println!("{i}, {j}");
                        res += 1;
                    }
                }, 
                Some('S') => {
                    let remaining = ['A', 'M', 'X'];
                    let valid = (i+1..i+4).zip(remaining.iter()).all(|(k, c)| {
                        match data.grid.get(&(k, j)) {
                            Some(entry) if entry == c => {true},
                            _ => {false}
                        }
                    });
                    if valid {
                        println!("{i}, {j}");
                        res += 1;
                    }
                }, 
                _ => {}
            }
        }
    }
    res
}

fn diag_right(data: &Data) -> i32 {
    let mut res = 0;
    for i in 0..data.rows {
        for j in 0..data.cols {
            match data.grid.get(&(i,j)) {
                Some('X') => {
                    let remaining = ['M', 'A', 'S'];
                    let valid = (1..4).zip(remaining.iter()).all(|(k, c)| {
                        match data.grid.get(&(i + k, j + k)) {
                            Some(entry) if entry == c => {true},
                            _ => {false}
                        }
                    });
                    if valid {
                        res += 1;
                    }
                },
                Some('S') =>  {
                    let remaining = ['A', 'M', 'X'];
                    let valid = (1..4).zip(remaining.iter()).all(|(k, c)| {
                        match data.grid.get(&(i + k, j + k)) {
                            Some(entry) if entry == c => {true},
                            _ => {false}
                        }
                    });
                    if valid {
                        res += 1;
                    }
                },
                _ => {}
            }
        }
    }
    res
}

fn diag_left(data: &Data) -> i32 {
    let mut res = 0;
    for i in 0..data.rows {
        for j in 0..data.cols {
            match data.grid.get(&(i,j)) {
                Some('X') => {
                    let remaining = ['M', 'A', 'S'];
                    let valid = (1..4).zip(remaining.iter()).all(|(k, c)| {
                        match data.grid.get(&(i + k, j - k)) {
                            Some(entry) if entry == c => {true},
                            _ => {false}
                        }
                    });
                    if valid {
                        res += 1;
                    }
                },
                Some('S') =>  {
                    let remaining = ['A', 'M', 'X'];
                    let valid = (1..4).zip(remaining.iter()).all(|(k, c)| {
                        match data.grid.get(&(i + k, j - k)) {
                            Some(entry) if entry == c => {true},
                            _ => {false}
                        }
                    });
                    if valid {
                        res += 1;
                    }
                },
                _ => {}
            }
        }
    }
    res
}


pub fn part_two() -> anyhow::Result<i32> {
   // lost this part and dont feel like redoing lol
   Ok(0)
}