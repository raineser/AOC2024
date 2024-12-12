use std::{fs, io::{BufRead, BufReader}};
use std::collections::HashSet;
use anyhow::{Context, self};

#[derive(Clone)]
struct Data {
    grid: Vec<Vec<char>>,
    i: i32,
    j: i32,
    dir: usize
}

impl Data {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day6.txt")?;
        let mut reader = BufReader::new(file);
        let mut grid = Vec::new();
        let mut guard = (0,0,0);

        for line in reader.lines() {
            let line = line?;
            let mut row:Vec<char> = Vec::new();
            for col in line.split(' ') {
                for c in col.chars() { // Should only be one 
                    row.push(c);
                }
            }
            grid.push(row);
        }
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                for (index, direction) in ['^', '>' , 'v' , '<'].iter().enumerate() {
                    if &grid[i][j] == direction {
                        guard = (i as i32,j as i32,index);
                        grid[i][j] = 'X';
                        break;
                    }
                }
            }
        }
        Ok(Self{grid, i: guard.0, j: guard.1, dir: guard.2})
    }
}


fn in_bounds(i: i32, j: i32, grid: &Vec<Vec<char>>) -> bool {
    if i < 0 || i == grid.len() as i32 {
        return false
    }
    if j < 0 || j == grid[0].len() as i32{
        return false
    }
    true
}


pub fn part_one() -> anyhow::Result<i32> {
    let mut data = Data::parse_input()?;
    let direction = [(-1,0), (0, 1), (1, 0), (0,-1)];
    let mut count = 0;
    let mut next_i = data.i + direction[data.dir].0;
    let mut next_j = data.j + direction[data.dir].1;
    
    while in_bounds(next_i, next_j, &data.grid) {
        if data.grid[next_i as usize][next_j as usize] == '.' || data.grid[next_i as usize][next_j as usize] == 'X' {
            data.grid[data.i as usize][data.j as usize] = 'X';
            data.i = next_i;
            data.j = next_j;
        } else {
            data.grid[data.i as usize][data.j as usize] = 'X';
            data.dir = (data.dir  + 1) % 4;
        }
        next_i = data.i + direction[data.dir].0;
        next_j = data.j + direction[data.dir].1;
    }
    for i in 0..data.grid.len() {
        for j in 0..data.grid[0].len() {
            if data.grid[i][j] == 'X' {
                count += 1;
            }
        }
    }
    Ok(count + 1)
}

pub fn part_two() -> anyhow::Result<i32> {
    let data = Data::parse_input()?;
    let start = (data.i as usize, data.j as usize);
    let direction = [(-1,0), (0, 1), (1, 0), (0,-1)];
    let mut count = 0;

    for i in 0..data.grid.len() {
        for j in 0..data.grid[0].len() {
            if (i, j) == start  || data.grid[i][j] == '#'{
                continue;
            }
            let mut curr_data = data.clone();
            curr_data.grid[i][j] = '#';
            let mut next_i = curr_data.i + direction[curr_data.dir].0;
            let mut next_j = curr_data.j + direction[curr_data.dir].1;
            let mut visit = HashSet::new();
            
            while in_bounds(next_i, next_j, &curr_data.grid) { 
                let position_and_dir = (next_i, next_j, curr_data.dir);
                if visit.contains(&position_and_dir) {
                    count += 1;
                    break
                }
                visit.insert((next_i, next_j, curr_data.dir));
                if curr_data.grid[next_i as usize][next_j as usize] == '.' || curr_data.grid[next_i as usize][next_j as usize] == 'X' {
                    curr_data.grid[curr_data.i as usize][curr_data.j as usize] = 'X';
                    curr_data.i = next_i;
                    curr_data.j = next_j;
                } 
                else {
                    curr_data.grid[curr_data.i as usize][curr_data.j as usize] = 'X';
                    curr_data.dir = (curr_data.dir  + 1) % 4;
                }
                next_i = curr_data.i + direction[curr_data.dir].0;
                next_j = curr_data.j + direction[curr_data.dir].1;
            }
        }
    }
    Ok(count)
}