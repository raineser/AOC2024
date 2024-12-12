use std::{collections::{HashSet, VecDeque}, fs, io::{BufRead, BufReader}};
use anyhow;

struct Data {
    grid: Vec<Vec<char>>
}

impl Data {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day12.txt")?;
        let mut reader = BufReader::new(file);
        let mut grid = Vec::new();

        for line in reader.lines() {
            let line = line?;
            grid.push(line.chars().collect());
        }   
        
        Ok(Self{grid})
    }
}

pub fn part_one() -> anyhow::Result<i64> {
    let data = Data::parse_input()?;
    let mut visit = HashSet::new();
    let mut total = 0;

    for i in 0..data.grid.len() {
        for j in 0..data.grid[0].len() {
            if !visit.contains(&(i as i32, j as i32)) {
                let mut q = VecDeque::new();
                q.push_back((i as i32, j as i32, data.grid[i][j]));
                visit.insert((i as i32, j as i32));
                let mut perimeter = 0;
                let mut area  = 1;

                while let Some((r, c, plot)) = q.pop_front() {
                    for dir in [(1,0), (-1,-0), (0,1), (0,-1)] {
                        let row = dir.0 + r;
                        let col = dir.1 + c;

                        if row < 0 || row == data.grid.len() as i32 || col < 0 || col == data.grid[0].len() as i32 {
                            perimeter += 1;
                            continue;
                        }
                        if data.grid[row as usize][col as usize] != plot {
                            perimeter += 1;
                            continue;
                        }
                        if !visit.contains(&(row, col)) {
                            area += 1;
                            visit.insert((row, col));
                            q.push_back((row, col, plot));
                        }
                    }
                }
                total += perimeter * area;
            }
        }
    }
    Ok(total)
}

pub fn part_two() -> anyhow::Result<i64> {
    let data = Data::parse_input()?;
    let mut visit = HashSet::new();
    let mut total = 0;

    for i in 0..data.grid.len() {
        for j in 0..data.grid[0].len() {
            if !visit.contains(&(i as i32, j as i32)) {
                let mut q = VecDeque::new();
                q.push_back((i as i32, j as i32, data.grid[i][j]));
                visit.insert((i as i32, j as i32));
                let mut perimeter = 0;
                let mut area  = 1;
                let mut edges = Vec::new();

                while let Some((r, c, plot)) = q.pop_front() {
                    for dir in [(1,0), (-1,-0), (0,1), (0,-1)] {
                        let row = dir.0 + r;
                        let col = dir.1 + c;

                        if row < 0 || row == data.grid.len() as i32 || col < 0 || col == data.grid[0].len() as i32 {
                            perimeter += 1;
                            edges.push((row, col, dir));
                            continue;
                        }
                        if data.grid[row as usize][col as usize] != plot {
                            perimeter += 1;
                            edges.push((row, col, dir));
                            continue;
                        }
                        if !visit.contains(&(row, col)) {
                            area += 1;
                            visit.insert((row, col));
                            q.push_back((row, col, plot));
                        }
                    }
                }

                let mut used = HashSet::new();
                for e1 in 0..edges.len() {
                    for e2 in e1+1..edges.len() {
                        if edges[e1].2 == edges[e2].2 {
                            if used.contains(&e2) {
                                continue;
                            }
                            if i32::abs(edges[e1].0 - edges[e2].0) == 1 && i32::abs(edges[e1].1 - edges[e2].1) == 0 {
                                perimeter -= 1;
                                used.insert(e2);
                            }
                            if i32::abs(edges[e1].1 - edges[e2].1) == 1 && i32::abs(edges[e1].0 - edges[e2].0) == 0 {
                                perimeter -= 1;
                                used.insert(e2);
                            }
                        }
                    }
                }
                total += perimeter * area;
            }
        }
    }
    Ok(total)
}