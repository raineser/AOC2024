use std::{collections::{BinaryHeap, VecDeque}, fs::{self, read}, hash::Hash, io::{BufRead, BufReader}};
use std::cmp;
use std::collections::{HashMap, HashSet};
use anyhow;


struct Data {
    grid: Vec<Vec<char>>,
    start: (i64,i64)
}

impl Data {
    fn parse_input() -> anyhow::Result<Self> {

        let file = fs::File::open("./inputs/day16.txt")?;
        let mut reader = BufReader::new(file);
        let mut grid:Vec<Vec<char>> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            grid.push(line.chars().collect());

        }

        let mut start = (0,0);
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 'S' {
                    start = (i as i64, j as i64);
                    grid[i][j] = '.';
                    break
                }
            }
        }
        
        Ok(Self{grid, start})
    }
}


pub fn part_one() -> anyhow::Result<i64> {
    let mut data = Data::parse_input()?;
    let mut curr = data.start;
    let mut dir = [(0,1), (1,0), (0,-1), (-1,0)];

    let mut res = i64::MAX;
    dfs(&data.grid, curr, &dir, 0, &mut HashMap::new(), 0, &mut res );
    
    Ok(res)
}



fn dfs(grid: &Vec<Vec<char>>, curr: (i64,i64), dir: &[(i64,i64); 4], dir_index: usize, cache: &mut HashMap<(i64,i64,usize),i64>, score: i64, min_score: &mut i64 ) {
    if grid[curr.0 as usize][curr.1 as usize] == '#' {
        return
    }

    if grid[curr.0 as usize][curr.1 as usize] == 'E' {
        *min_score = cmp::min(*min_score, score);
        return
    }

    if let Some(val) = cache.get(&(curr.0, curr.1, dir_index)) {
        if val <= &score {
            return
        }
    }

    cache.insert((curr.0, curr.1, dir_index), score);

    let right = (dir_index + 1) % 4;
    let left = dir_index.checked_sub(1).unwrap_or(3);
    dfs(grid, (curr.0 + dir[dir_index].0, curr.1 + dir[dir_index].1), dir, dir_index, cache, score + 1, min_score);
    dfs(grid, (curr.0, curr.1), dir, right, cache, score + 1000, min_score);
    dfs(grid, (curr.0, curr.1), dir, left, cache, score + 1000, min_score);
}




pub fn part_two() -> anyhow::Result<i64> {
    todo!()
}