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
    let mut data = Data::parse_input()?;
    let mut curr = data.start;
    let mut dir = [(0,1), (1,0), (0,-1), (-1,0)];
    let mut res = i64::MAX;
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), curr.0, curr.1, 0 as usize, Vec::new()));
    let mut scores = HashMap::new();
    let mut min_path = HashSet::new();
    
    while let Some((score, i, j, index, mut path)) = heap.pop() {
        path.push((i,j));
        
        if data.grid[i as usize][j as usize] == 'E' {
            if score.0 <= res {
                res = score.0;
                 min_path.extend(path);
            }
            continue
        }
        
        if data.grid[i as usize][j as usize] == '#' {
            continue
        }
        
        if let Some(val) = scores.get(&(i,j,index)) {
            if *val < score.0 {
                continue
            }
        }
        
        scores.insert((i,j,index), score.0);
        
        let right = (index + 1) % 4;
        let left = index.checked_sub(1).unwrap_or(3);
        heap.push((Reverse(score.0 + 1), i + dir[index].0, j + dir[index].1, index, path.clone()));
        heap.push((Reverse(score.0 + 1001), i + dir[right].0, j + dir[right].1 , right, path.clone()));
        heap.push((Reverse(score.0 + 1001), i + dir[left].0, j + dir[left].1 , left, path));
    }

    Ok(min_path.len() as i64)
}
