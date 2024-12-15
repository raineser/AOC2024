use std::{fs, i64, io::{BufRead, BufReader}};
use anyhow;

struct Data {
    robots: Vec<Robot>
}

#[derive(Debug)]
struct Robot {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64
}

impl Data {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day14.txt")?;
        let mut reader = BufReader::new(file);
        let mut robots = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let mut x_y = (0,0);
            let mut vx_y = (0,0);
            if let Some((mut position, mut velocity)) = line.split_once(' ') {
                if let Some(equal) = position.find('=') {
                    position = &position[equal+1..];
                    if let Some((x,y)) = position.split_once(',') {
                        x_y = (x.trim().parse::<i64>()?, y.trim().parse::<i64>()?);
                    }
                }
                if let Some(equal) = velocity.find('=') {
                    velocity = &velocity[equal+1..];
                    if let Some((x,y)) = velocity.split_once(',') {
                        vx_y = (x.trim().parse::<i64>()?, y.trim().parse::<i64>()?);
                    }
                }
            }
            robots.push(Robot{x: x_y.0, y: x_y.1, vx: vx_y.0, vy: vx_y.1})
        }
        Ok(Self{robots})
    }
    
    
}


pub fn part_one() -> anyhow::Result<i64> {
    let data = Data::parse_input()?;
    Ok(sim_seconds(100, &data)?.0)
}

pub fn part_two() -> anyhow::Result<i64> {
    let data = Data::parse_input()?;
    let mut seconds = 1;
    loop {
        let (_, grid) = sim_seconds(seconds, &data)?;
        let unique = grid
            .into_iter()
            .all(|row| {
                row
                .into_iter()
                .all(|val| val <= 1)
            });
        if unique {
            break
        }
        seconds += 1;
    };
    Ok(seconds)
}

fn sim_seconds(seconds: i64, data: &Data) -> anyhow::Result<(i64, Vec<Vec<i64>>)> {
    let mut grid = vec![vec![0; 101];103];
    for robot in &data.robots {
        let final_x = (robot.x + (robot.vx * seconds)).rem_euclid(grid[0].len() as i64);
        let final_y = (robot.y + (robot.vy * seconds)).rem_euclid(grid.len() as i64);
        grid[final_y as usize][final_x as usize] += 1;
    }
    
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if x < grid[0].len()/2 && y < grid.len() / 2 {
                q1 += grid[y][x];
            } else if x < grid[0].len()/2 && y > grid.len() / 2 {
                q2 += grid[y][x];
            } else if x > grid[0].len()/2 && y < grid.len() / 2 {
                q3 += grid[y][x];
            } else if x > grid[0].len()/2 && y > grid.len() / 2 {
                q4 += grid[y][x];
            }
        }
    }
    Ok( (q1* q2 * q3* q4, grid))
}