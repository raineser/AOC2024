use std::collections::{HashMap, HashSet};
use std::{fs, io::BufReader};
use std::io::BufRead;

use anyhow;

struct Data {
    grid: Vec<Vec<char>>,
    directions: Vec<(i64,i64)>
}

impl Data {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day15.txt")?;
        let mut reader = BufReader::new(file);
        let mut grid = Vec::new();
        let mut directions = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if let Some(_) = line.find('#') {
                grid.push(line.chars().collect());
            } else {
                for c in line.chars() {
                    match c {
                        '^' => {
                            directions.push((-1,0))
                        }, 
                        'v' => {
                            directions.push((1,0))
                        }, 
                        '>' => {
                            directions.push((0,1))
                        },
                        '<' => {
                            directions.push((0,-1))
                        }, 
                        _ => {anyhow::bail!("Not valid direction")}
                    }
                }
            }
        }
        Ok(Data{grid, directions})
    }

    fn parse_input_scaled() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day15.txt")?;
        let mut reader = BufReader::new(file);
        let mut grid = Vec::new();
        let mut directions = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if let Some(_) = line.find('#') {
                let mut grid_line= Vec::new();
                for c in line.chars() {
                    match c {
                        '.' => {
                            grid_line.push('.');
                            grid_line.push('.');
                        }, 
                        '#' => {
                            grid_line.push('#');
                            grid_line.push('#');
                        }, 
                        'O' => {
                            grid_line.push('[');
                            grid_line.push(']');
                        }, 
                        '@' => {
                            grid_line.push('@');
                            grid_line.push('.');
                        }, 
                        _ => {anyhow::bail!("invalid char")}
                    }
                }
                grid.push(grid_line);
            } else {
                for c in line.chars() {
                    match c {
                        '^' => {
                            directions.push((-1,0))
                        }, 
                        'v' => {
                            directions.push((1,0))
                        }, 
                        '>' => {
                            directions.push((0,1))
                        },
                        '<' => {
                            directions.push((0,-1))
                        }, 
                        _ => {anyhow::bail!("invalid direction")}
                    }
                }
            }
        }
        Ok(Data{grid, directions})
    }

}

pub fn part_one() -> anyhow::Result<i64> {
    let mut data = Data::parse_input()?;
    let mut curr = (0,0);
    for i in 0..data.grid.len() {
        for j in 0..data.grid[0].len() {
            if data.grid[i][j] == '@' {
                curr = (i as i64,j as i64);
                break
            }
        }
    }

    for dir in data.directions {
        let mut i = curr.0 + dir.0;
        let mut j = curr.1 + dir.1;

        if data.grid[i as usize][j as usize] == '#' {
            continue;
        }
        if data.grid[i as usize][j as usize] == '.' {
            data.grid[i as usize][j as usize] = '@';
            data.grid[curr.0 as usize][curr.1 as usize] = '.';
            curr = (i,j);
            continue;
        }
        if data.grid[i as usize][j as usize] == 'O' {
            while data.grid[i as usize][j as usize] == 'O' {
                i += dir.0;
                j += dir.1;
            }
            if data.grid[i as usize][j as usize] == '.' {
                data.grid[i as usize][j as usize] = 'O';
                data.grid[curr.0 as usize][curr.1 as usize] = '.';
                curr = (curr.0 + dir.0, curr.1 + dir.1);
                data.grid[curr.0 as usize][curr.1 as usize] = '@';

            }
        }
    }

    let mut res = 0;
    for i in 0..data.grid.len() {
        for j in 0..data.grid[0].len() {
            if data.grid[i][j] == 'O' {
                res += 100 * i as i64 + j as i64;
            }
        }
    }
    Ok(res)
}

pub fn part_two() -> anyhow::Result<i64> {
    let mut data = Data::parse_input_scaled()?; 
    let mut curr = (0,0);
    for i in 0..data.grid.len() {
        for j in 0..data.grid[0].len() {
            if data.grid[i][j] == '@' {
                curr = (i as i64,j as i64);
                break
            }
        }
    }

    for dir in data.directions {
        let mut i = curr.0 + dir.0;
        let mut j = curr.1 + dir.1;

        if data.grid[i as usize][j as usize] == '#' {
            continue;
        }
        if data.grid[i as usize][j as usize] == '.' {
            data.grid[i as usize][j as usize] = '@';
            data.grid[curr.0 as usize][curr.1 as usize] = '.';
            curr = (i,j);
            continue;
        }
        if data.grid[i as usize][j as usize] == '[' ||  data.grid[i as usize][j as usize] == ']'  {
            match dir {
                (0,-1) | (0,1) => {
                    while data.grid[i as usize][j as usize] == '[' || data.grid[i as usize][j as usize] == ']' {
                        i += dir.0;
                        j += dir.1;
                    }
                    if data.grid[i as usize][j as usize] == '.' {
                        while j != curr.1 {
                            data.grid[i as usize].swap(j as usize, (j - dir.1) as usize);
                            j -= dir.1;
                        }
                        curr = (curr.0 + dir.0, curr.1 + dir.1);
                    }
                },
                (-1,0) | (1,0) => {
                    if let Ok(_) = valid_push(dir.0, &data.grid, (i,j)) {
                        push_vert(dir.0, &mut data.grid, (i,j), &mut HashSet::new())?;
                        data.grid[i as usize][j as usize] = '@';
                        data.grid[curr.0 as usize][curr.1 as usize] = '.';
                        curr = (i,j);
                    }
                },
                _ => {anyhow::bail!("invalid direction")}
            }
        }
    }

    for line in &data.grid {
        println!("{:?}", line);
    }

    let mut res = 0;
    for i in 0..data.grid.len() {
        for j in 0..data.grid[0].len() {
            if data.grid[i][j] == '[' {
                res += 100 * i as i64 + j as i64;
            }
        }
    }
    Ok(res)
}

fn valid_push(dir: i64, grid: &Vec<Vec<char>>, curr: (i64,i64)) -> anyhow::Result<()> {
    if grid[(curr.0) as usize][curr.1 as usize] == '#' {
        anyhow::bail!("Hit a wall");
    } 
    
    if grid[(curr.0) as usize][curr.1 as usize] == '.' {
        return Ok(())
    }

    match grid[curr.0 as usize][curr.1 as usize] {
        '[' => {
            match (valid_push(dir, grid, (curr.0 + dir, curr.1)), valid_push(dir, grid, (curr.0 + dir, curr.1 + 1))) {
                (Ok(_), Ok(_)) => {
                    Ok(())
                }, 
                _ => {
                    anyhow::bail!("Hit a Wall")
                }
            }  
        },
        ']' => {
            match (valid_push(dir, grid, (curr.0 + dir, curr.1)), valid_push(dir, grid, (curr.0 + dir, curr.1 - 1))) {
                (Ok(_), Ok(_)) => {
                    Ok(())
                }, 
                _ => {
                    anyhow::bail!("Hit a Wall")
                }
            }  
        },
        _ => {anyhow::bail!("Not a box")}
    }
}


fn push_vert(dir: i64, grid: &mut Vec<Vec<char>>, curr: (i64,i64), cache: &mut HashSet<(i64,i64)>) -> anyhow::Result<()> {
    if cache.contains(&curr) {
        return Ok(())
    }
    
    if grid[(curr.0) as usize][curr.1 as usize] == '#' {
        anyhow::bail!("Hit a wall");
    } 
    
    if grid[(curr.0) as usize][curr.1 as usize] == '.' {
        return Ok(())
    }

    match grid[curr.0 as usize][curr.1 as usize] {
        '[' => {
            match (push_vert(dir, grid, (curr.0 + dir, curr.1), cache), push_vert(dir, grid, (curr.0 + dir, curr.1 + 1), cache)) {
                (Ok(_), Ok(_)) => {
                    grid[(curr.0 + dir) as usize][curr.1 as usize] = '[';
                    grid[(curr.0) as usize][curr.1 as usize] = '.';

                    grid[(curr.0 + dir) as usize][curr.1 as usize + 1] = ']';
                    grid[(curr.0) as usize][curr.1 as usize + 1] = '.';
                }, 
                _ => {
                    anyhow::bail!("Hit a Wall")
                }
            }  
        },
        ']' => {
            match (push_vert(dir, grid, (curr.0 + dir, curr.1), cache), push_vert(dir, grid, (curr.0 + dir, curr.1 - 1), cache)) {
                (Ok(_), Ok(_)) => {
                    grid[(curr.0 + dir) as usize][curr.1 as usize] = ']';
                    grid[(curr.0) as usize][curr.1 as usize] = '.';

                    grid[(curr.0 + dir) as usize][curr.1 as usize - 1] = '[';
                    grid[(curr.0) as usize][curr.1 as usize - 1] = '.';
                }, 
                _ => {
                    anyhow::bail!("Hit a Wall")
                }
            }  
        },
        _ => {anyhow::bail!("Not a box")}
    }
    cache.insert(curr);
    Ok(())
}