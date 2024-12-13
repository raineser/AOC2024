use std::{collections::{HashSet, VecDeque}, fs::{self, read}, io::{BufRead, BufReader}};

use anyhow;


struct Data {
    grid: Vec<Vec<i32>>
}

impl Data {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day10.txt")?;
        let mut reader = BufReader::new(file);
        let mut grid = Vec::new();
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            let mut row = Vec::new();
            for c in line.chars() {
                if let Some(height) = c.to_digit(10) {
                    row.push(height as i32);
                } 
            }
            grid.push(row);
        }
        Ok(Self{grid})
    }
}



pub fn part_one() -> anyhow::Result<i64> {
    let data = Data::parse_input()?;
    let mut res = 0;
    for i in 0..data.grid.len() {
        for j in 0..data.grid[0].len() {
            if data.grid[i][j] == 0 {
                let mut q = VecDeque::new();
                q.push_back((i as i32, j as i32, 0));
                let mut visit = HashSet::new();
                visit.insert((i as i32, j as i32));

                while let Some((r, c, prev)) = q.pop_front() {
                    if data.grid[r as usize][c as usize] == 9 {
                        res += 1;
                        continue;
                    }
                    for dir in [(1,0), (-1,0), (0,1), (0,-1)] {
                        let row = dir.0 + r;
                        let col = dir.1 + c;
                        if row < 0 || row == data.grid.len() as i32 {
                            continue
                        }
                        if col < 0 || col == data.grid.len() as i32 {
                            continue
                        }
                        if data.grid[row as usize][col as usize] != prev + 1{
                            continue;
                        }
                        if visit.contains(&(row , col)) {
                            continue
                        }
                        visit.insert((row, col));
                        q.push_back((row, col, data.grid[row as usize][col as usize]));
                    }
                }
            }
        }
    }
    Ok(res)
}


pub fn part_two() -> anyhow::Result<i64> {
    let data = Data::parse_input()?;
    let mut res = 0;
    for i in 0..data.grid.len() {
        for j in 0..data.grid[0].len() {
            if data.grid[i][j] == 0 {
                let mut q = VecDeque::new();
                q.push_back((i as i32, j as i32, 0));
                //let mut visit = HashSet::new();
                //visit.insert((i as i32, j as i32));

                while let Some((r, c, prev)) = q.pop_front() {
                    if data.grid[r as usize][c as usize] == 9 {
                        res += 1;
                        continue;
                    }
                    for dir in [(1,0), (-1,0), (0,1), (0,-1)] {
                        let row = dir.0 + r;
                        let col = dir.1 + c;
                        if row < 0 || row == data.grid.len() as i32 {
                            continue
                        }
                        if col < 0 || col == data.grid.len() as i32 {
                            continue
                        }
                        if data.grid[row as usize][col as usize] != prev + 1{
                            continue;
                        }
                        //if visit.contains(&(row , col)) {
                            //continue
                        //}
                        //visit.insert((row, col));
                        q.push_back((row, col, data.grid[row as usize][col as usize]));
                    }
                }
            }
        }
    }
    Ok(res)
}