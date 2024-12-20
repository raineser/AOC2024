use std::{cmp::max, collections::{BinaryHeap, HashMap, HashSet, VecDeque}, fs::{self}, io::{BufRead, BufReader}};
use std::cmp::{Reverse, self};
use anyhow;


struct Data {
    grid: Vec<Vec<char>>,
    start: (i64,i64),
    end: (i64,i64)
}

impl Data {
    fn parse_input() -> anyhow::Result<Self> {

        let file = fs::File::open("./inputs/day20.txt")?;
        let mut reader = BufReader::new(file);
        let mut grid:Vec<Vec<char>> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            grid.push(line.chars().collect());
        }

        let mut start = (0,0);
        let mut end = (0,0);
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 'S' {
                    start = (i as i64, j as i64);
                    grid[i][j] = '.';
                }
                if grid[i][j] == 'E' {
                    end = (i as i64, j as i64);
                    grid[i][j] = '.';
                }
            }
        }
        
        Ok(Self{grid, start, end})
    }
}

fn valid_cheat(grid: &Vec<Vec<char>>, i: i64, j: i64) -> bool {
    if grid[i as usize][j as usize] != '#' {
        return false;
    }
    
    let mut count = 0;
    for dir in [(1,0), (-1,0), (0,1), (0, -1)] {
        let row = i + dir.0;
        let col = j + dir.1;

        if row <= 0 || row >= grid.len() as i64 - 1 {
            continue;
        }
        if col <= 0 || col >= grid[0].len() as i64 -1 {
            continue;
        }
        if grid[row as usize][col as usize] == '.' {
            count += 1;
        }
    }
    count >= 2
}




pub fn part_one() -> anyhow::Result<i64> {
    let mut data = Data::parse_input()?;
    let mut cheats = Vec::new();
    let mut d = HashMap::new();

    for i in 1..data.grid.len() - 1 {
        for j in 1..data.grid[0].len() - 1 {
            if valid_cheat(&data.grid, i as i64, j as i64) {
                cheats.push((i,j));
            }
        }
    }
    let no_cheats = find_path(&data)?;
    let mut time_saved = 0;

    for cheat in cheats {
        data.grid[cheat.0][cheat.1] = '.';
        let time= find_path(&data)?;
        if no_cheats - time >= 100 {
            time_saved += 1;
        }
        *d.entry(no_cheats - time).or_insert(0) += 1;
        data.grid[cheat.0][cheat.1] = '#';
    }
    let mut d = d.into_iter().map(|(key, value)| (key, value)).collect::<Vec<(i64,i64)>>();
    d.sort();
    println!("{:?}", d);
    Ok(time_saved)

}

fn find_path(data: &Data) -> anyhow::Result<i64> {
    let mut q = BinaryHeap::new();
    let mut visit = HashSet::new();
    q.push((Reverse(0), data.start.0, data.start.1));
    visit.insert((data.start.0,data.start.1));
    let mut min = i64::MAX;

    while let Some((dist, i, j)) = q.pop() {
        if i == data.end.0  && j == data.end.1 {
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

fn path_distance(data: &Data) -> anyhow::Result<HashMap<(i64,i64), i64>> {
    let mut q = BinaryHeap::new();
    let mut visit = HashSet::new();
    let mut distance = HashMap::new();
    q.push((Reverse(0), data.start.0, data.start.1));
    visit.insert((data.start.0,data.start.1));
    let mut min = i64::MAX;

    while let Some((dist, i, j)) = q.pop() {
        distance.insert((i,j), dist.0);
        if i == data.end.0  && j == data.end.1 {
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
    Ok(distance)
}

fn valid_20_cheat(grid: &Vec<Vec<char>>, i: i64, j: i64, distance: &HashMap<(i64,i64), i64>) -> Vec<(i64,i64,i64)> {
    
    let mut cheats = Vec::new();

    let mut q = VecDeque::new();
    for dir in [(1,0), (-1,0), (0,1), (0, -1)] {
        let row = i + dir.0;
        let col = j + dir.1;

        if row <= 0 || row >= grid.len() as i64 - 1 {
            continue;
        }
        if col <= 0 || col >= grid[0].len() as i64 -1 {
            continue;
        }
        if grid[row as usize][col as usize] == '.' {
            continue;
        }
        q.push_back((row, col));
    }
    
    while let Some((r,c)) = q.pop_front() {
        let mut max = 0;
        for dir in [(1,0), (-1,0), (0,1), (0, -1)] {
            let row = r + dir.0;
            let col = c + dir.1;
    
            if row <= 0 || row >= grid.len() as i64 - 1 {
                continue;
            }
            if col <= 0 || col >= grid[0].len() as i64 -1 {
                continue;
            }
            if grid[row as usize][col as usize] == '#' {
                continue;
            }
            if let Some(&dist) = distance.get(&(row, col)) {
                max = cmp::max(dist, max);
            }

        }
        if max > 0 {
            if let Some(&start) = distance.get(&(i,j)) {
                if start != max {
                    cheats.push((r,c, max - (start + 2)));
                }
            }
        }
    }
    cheats
}


pub fn part_two() -> anyhow::Result<i64> {
   let mut data = Data::parse_input()?;
   let distance = path_distance(&data)?;
   let mut cheats = HashMap::new();
   let mut res = 0;

    for i in 1..data.grid.len() - 1 {
        for j in 1..data.grid[0].len() - 1 {
            if data.grid[i][j] == '.' {
                for ((row, col, saved)) in valid_20_cheats(&data.grid, i as i64, j as i64, &distance, 20) {
                    *cheats.entry(saved).or_insert(0) += 1;
                    if saved >= 100 {
                        res += 1;
                    }
                }
            }
        }
    }
    let mut d = cheats.into_iter().map(|(key, value)| (key, value)).collect::<Vec<(i64,i64)>>();
    d.sort();
    println!("{:?}", d);
    Ok(res)
}
    


fn valid_cheats(grid: &Vec<Vec<char>>, i: i64, j: i64, distance: &HashMap<(i64,i64), i64>, max_count: i64) -> Vec<(i64,i64,i64)> {
    let mut cheats = Vec::new();
    let mut visit = HashSet::new();

    let mut q = VecDeque::new();
    for dir in [(1,0), (-1,0), (0,1), (0, -1)] {
        let row = i + dir.0;
        let col = j + dir.1;

        if row <= 0 || row >= grid.len() as i64 - 1 {
            continue;
        }
        if col <= 0 || col >= grid[0].len() as i64 -1 {
            continue;
        }
        if grid[row as usize][col as usize] == '.' {
            continue;
        }
        q.push_back((row, col, 1));
    }
    
    while let Some((r,c, count)) = q.pop_front() {
        if count == max_count{
            continue;
        }
        //let mut max = 0;
        for dir in [(1,0), (-1,0), (0,1), (0, -1)] {
            let row = r + dir.0;
            let col = c + dir.1;
    
            if row <= 0 || row >= grid.len() as i64 - 1 {
                continue;
            }
            if col <= 0 || col >= grid[0].len() as i64 -1 {
                continue;
            }
            if let Some(&dist) = distance.get(&(row, col)) {
                if let Some(&start) = distance.get(&(i,j)) {
                    if start != dist && dist - (start + count + 1) > 0 {
                        cheats.push((r,c, dist - (start + count + 1)));
                    }
                }

            }
            if grid[row as usize][col as usize] == '#' {
                if !visit.contains(&(row,col)) {
                    q.push_back((row, col, count + 1));
                    visit.insert((row,col));
                }
            }

        }
    }
    cheats
}