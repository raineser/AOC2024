use std::{collections::{BinaryHeap, HashSet}, fs, i64, io::{BufRead, BufReader}};
use std::cmp::Reverse;
use anyhow;
struct Data {
    grid: Vec<Vec<char>>,
    bytes: Vec<(usize, usize)>
}

impl Data {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day18.txt")?;
        let mut reader = BufReader::new(file);
        let mut grid = vec![vec!['.';71];71];
        let mut bytes = Vec::new();

        for (index, line) in  reader.lines().enumerate() {
            let line = line?;
            if let Some((x,y)) = line.split_once(',') {
                let x = x.parse::<usize>()?;
                let y = y.parse::<usize>()?;
                if index < 1024 {
                    grid[y][x] = '#';
                }
                bytes.push((y,x));
            }
        }
        Ok(Self{grid, bytes})
    }
}

pub fn part_one() -> anyhow::Result<i64> {
    let data = Data::parse_input()?;
    let min = find_path(&data)?;
    Ok(min)
}

pub fn part_two() -> anyhow::Result<(usize, usize)> {
    let mut byte = 1024;
    let mut data = Data::parse_input()?;

    loop {
        println!("on byte {byte}");
        let (i, j) = data.bytes[byte];
        data.grid[i][j] = '#';
        if find_path(&data)? == i64::MAX {
            return Ok((i,j));
        }
        byte += 1;
    }
}

fn find_path(data: &Data) -> anyhow::Result<i64> {
    let mut q = BinaryHeap::new();
    let mut visit = HashSet::new();
    q.push((Reverse(0), 0, 0));
    visit.insert((0,0));
    let mut min = i64::MAX;

    while let Some((dist, i, j)) = q.pop() {
        if i == data.grid.len() as i64 -1 && j == data.grid[0].len() as i64 - 1 {
            min = dist.0;
            break
        }

        for dir in [(1,0), (-1,0), (0,1), (0, -1)] {
            let row = i + dir.0;
            let col = j + dir.1;
            if row < 0 || row == data.grid.len() as i64 {
                continue;
            }
            if col < 0 || col == data.grid[0].len() as i64 {
                continue;
            }
            if visit.contains(&(row, col)) {
                continue;
            }
            if data.grid[row as usize][col as usize] == '#' {
                continue;
            }
            visit.insert((row,col));
            q.push((Reverse(dist.0 + 1), row, col));
        }
    }
    Ok(min)
}