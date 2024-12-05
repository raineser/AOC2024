use std::{fs, io::{BufRead, BufReader}};
use std::collections::HashMap;
use anyhow::{self, Context};

struct Data {
    left: Vec<i32>,
    right: Vec<i32>   
}

impl Data {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day1.txt").context("Open input file")?;
        let mut reader = BufReader::new(file);

        let mut left_values = Vec::new();
        let mut right_values = Vec::new();

        for line in reader.lines(){
            let line = line?;
            if let Some((left, right)) = line.split_once("   ") {
                left_values.push(left.parse::<i32>().context("Parse the left value")?);
                right_values.push(right.parse::<i32>().context("Parse the right value")?);
            }
        }

        left_values.sort_unstable();
        right_values.sort_unstable();
        Ok(Self {left: left_values, right: right_values})
    }
}

pub fn part_one() -> anyhow::Result<i32> {
    let data = Data::parse_input()?;
    
    let result = data.left
        .iter()
        .zip(data.right.iter())
        .map(|(l, r)| i32::abs(l - r))
        .sum();
    
    Ok(result)
}

pub fn part_two() -> anyhow::Result<i32> {
    let data = Data::parse_input()?;

    let right = data.right
        .iter()
        .fold(HashMap::new(), |mut map, value|{
            *map.entry(value).or_insert(0) += 1;
            map
        });
    
    let result = data.left
        .iter()
        .map(|l| {
            if let Some(entry) = right.get(l) {
                *entry * *l // that is ugly lol
            } else {
                0
            }
        })
        .sum();
    
    Ok(result)
}


