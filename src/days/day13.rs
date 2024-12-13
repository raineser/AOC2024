use std::{fs, io::{BufRead, BufReader}};
use std::cmp;
use std::collections::HashMap;
use anyhow;
use ndarray::*;
use ndarray_linalg::Solve;

struct Data {
    games: Vec<Vec<(i64, i64)>>
}

impl Data {
    //this was a bit gross
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day13.txt")?;
        let mut reader = BufReader::new(file);
        let mut lines = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if let Some(_) = line.find(',') {
                lines.push(line);
            }
        }

        let mut games = Vec::new();
        for chunk in lines.chunks_exact(3) {
            let mut game = Vec::new();
            for i in 0..3 {
                let mut x:i64 = 0;
                let mut y = 0;
                if let Some(x_index) = chunk[i].find('X') {
                    if let Some(comma_index) = chunk[i].find(',') {
                        x = chunk[i][x_index+2..comma_index].parse::<i64>()?;
                    }
                }
                if let Some(y_index) = chunk[i].find('Y') {
                    y = chunk[i][y_index+2..].trim().parse::<i64>()?;
                }
                game.push((x,y));
            }
            games.push(game);
        }
        Ok(Self{games})
    }
}


pub fn part_one() -> anyhow::Result<i64> {
    let data = Data::parse_input()?;
    let mut tokens = 0;
    for game in data.games {
        let mut cache = HashMap::with_capacity(200);
        let token = dfs(game[0], game[1], game[2], (0,0), &mut cache);
        if token <= 400 {
            tokens += token;
        }
    }
    Ok(tokens)
}

fn dfs(a: (i64,i64), b: (i64,i64), prize: (i64,i64), curr: (i64,i64), cache: &mut HashMap<(i64,i64), i64>) -> i64 {
    if curr == prize {
        return 0;
    }

    if curr.0 > prize.0 || curr.1 > prize.1 {
        return 1000
    }

    if let Some(res) = cache.get(&curr) {
        return *res
    }

    let mut res = 0;
    res = cmp::min( 
        3 + dfs(a, b, prize, (curr.0 + a.0, curr.1 + a.1), cache),
        1 + dfs(a, b, prize, (curr.0 + b.0, curr.1 + b.1), cache));
    
    cache.insert(curr, res);
    res
}   

pub fn part_two() -> anyhow::Result<i64> {
    let data = Data::parse_input()?;
    let mut tokens = 0;
    for mut game in data.games {
        game[2].0 += 10000000000000;
        game[2].1 += 10000000000000;
        let b = (game[2].1 * game[0].0 - game[2].0 * game[0].1) / (game[1].1 * game[0].0 - game[1].0 * game[0].1);
        let a = (game[2].0 - b * game[1].0) / game[0].0;
        if (game[0].0 * a + game[1].0 * b, game[0].1 * a + game[1].1 * b) == (game[2].0, game[2].1) {
            tokens += (3 * a) + b
        }
    }
    Ok(tokens)
}