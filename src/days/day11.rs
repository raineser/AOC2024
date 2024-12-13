use std::{fs::{self, read}, io::{BufRead, BufReader}};
use anyhow::{self, Context};
use std::collections::HashMap;
struct Data {
    nums: HashMap<i64, i64>
}

impl Data {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day11.txt")?;
        let mut reader = BufReader::new(file);

        let mut nums = HashMap::new();
        for line in reader.lines() {
            let line = line?;
            for c in line.split(' ') {
                *nums.entry(c.parse::<i64>().context("Parse from the input")?).or_insert(0) += 1;
            }
        }
        Ok(Self{nums})
    }
}

pub fn part_one() -> anyhow::Result<i64> {
    let mut data = Data::parse_input()?;
    
    for _ in 0..25 {
        let mut new = HashMap::with_capacity(data.nums.len());
        for (num, count) in data.nums {
            if num == 0 {
                *new.entry(1).or_insert(0) += count;
            } else if num.to_string().len() % 2 == 0 {
                let s = num.to_string();
                let (one, two)  = s.split_at(s.len()/2);
                *new.entry(one.parse::<i64>().context("Parse the first split")?).or_insert(0) += count;
                let two = two.trim_start_matches('0');
                if two.len() == 0 {
                    *new.entry(0).or_insert(0) += count;
                } else {
                    *new.entry(two.parse::<i64>().context("Parse the reamining split num")?).or_insert(0) += count;
                }
            } else {
                *new.entry(num * 2024).or_insert(0) += count;
            }
        }
        data.nums = new;
    } 
    //println!("{:?}", data.nums);
    Ok(data.nums.into_values().sum::<i64>())
}

pub fn part_two() -> anyhow::Result<i64> {
    let mut data = Data::parse_input()?;

    for _ in 0..75 {
        let mut new = HashMap::new();
        for (num, count) in data.nums {
            if num == 0 {
                *new.entry(1).or_insert(0) += count;
            } else if num.to_string().len() % 2 == 0 {
                let s = num.to_string();
                let (one, two)  = s.split_at(s.len()/2);
                *new.entry(one.parse::<i64>().context("Parse the first num split")?).or_insert(0) += count;
                *new.entry(two.parse::<i64>().context("Parse the second num")?).or_insert(0) += count;
            } else {
                *new.entry(num * 2024).or_insert(0) += count;
            }
        }
        data.nums = new;
    } 
    Ok(data.nums.into_values().sum::<i64>())
}








