use std::{fs, io::{BufRead, BufReader}};
use anyhow::{self, Context};

struct Data {
    levels: Vec<Vec<i32>>
}

impl Data {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day2.txt")?;
        let mut reader = BufReader::new(file);
        let mut levels = Vec::new();
        
        for line in reader.lines() {
            let line = line?;
            let level = line.split(' ').map(|val| val.parse::<i32>().expect("valid i32")).collect();
            levels.push(level);
        }

        Ok(Self{levels})
    }
}

enum State {
    Init,
    First(i32),
    Decreasing(i32),
    Increasing(i32)
}

impl State {
    fn next(prev: State, curr: i32) -> anyhow::Result<State> {
        match prev {
            State::Init => {
                Ok(State::First((curr)))
            },
            State::First(prev) => {
                valid(prev, curr)?;
                if curr > prev {
                    Ok(Self::Increasing(curr))
                } else {
                    Ok(Self::Decreasing(curr))
                }
            },
            State::Decreasing(prev) => {
                valid(prev, curr)?;
                if curr > prev {
                    anyhow::bail!("Invalid State Change")
                }
                Ok(State::Decreasing(curr))
            },
            State::Increasing(prev) => {
                valid(prev, curr)?;
                if curr < prev {
                    anyhow::bail!("Invalid State Change")
                }
                Ok(State::Increasing(curr))
            },
        }
    }
}

fn check_level(level: &Vec<i32>) -> bool {
    let mut curr = State::Init;
    for val in level {
        match State::next(curr, *val) {
            Ok(next) => {
                curr = next;
            }, 
            Err(_) => {
                return false;
            }
        }
    }
    true
}

fn valid(prev: i32, curr: i32) -> anyhow::Result<()> {
    let diff = i32::abs(prev-curr);
    if diff > 3 || diff == 0 {
        anyhow::bail!("Incorrect rate of change")
    }
    Ok(())
}

pub fn part_one() -> anyhow::Result<i32> {
    let data = Data::parse_input()?;
    Ok(data.levels
        .iter()
        .filter(|level| check_level(level))
        .count() as i32
    )
}

pub fn part_two() -> anyhow::Result<i32> {
    let data = Data::parse_input()?;
    Ok (data.levels
        .iter()
        .filter(|level| {
            for i in 0..level.len() {
                let mut new_level = (*level).clone(); //a bit gross
                new_level.remove(i);
                if check_level(&new_level) {
                    return true
                }
            }
            false
        })
        .count() as i32   
    )  
}