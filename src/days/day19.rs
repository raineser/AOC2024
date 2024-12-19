use std::{fs, io::{BufRead, BufReader}};
use std::collections::HashMap;
use anyhow;

struct Data {
    towels: Vec<Vec<u8>>,
    designs: Vec<Vec<u8>>
}

impl Data {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day19.txt")?;
        let mut reader = BufReader::new(file);
        let mut towels = Vec::new();
        let mut designs = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if let Some(_) = line.find(',') {
                for towel in line.split(',') {
                    towels.push(towel.trim().to_string().into_bytes());
                }
            } else {
                let design = line.trim().to_string().into_bytes();
                if design.len() > 0 {
                    designs.push(design);
                }
            }
        }
        Ok(Self{towels, designs})
    }
}

pub fn part_one() -> anyhow::Result<i64> {
    let mut data = Data::parse_input()?;
    let mut res = 0;
    for design in data.designs {
        if dfs(&design, 0, &data.towels) {
            res += 1
        }
    }
    Ok(res)
}

fn dfs(design: &[u8], index: usize, towels: &Vec<Vec<u8>>) -> bool {
    if index == design.len() {
        return true
    }

    for towel in towels {
        if towel.len() <= design.len() - index {
            if towel == &design[index..index+towel.len()] {
                if dfs(design, index + towel.len(), towels) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn part_two() -> anyhow::Result<i64> {
    let mut data = Data::parse_input()?;
    let mut res = 0;
    for design in data.designs {
        res += dfs_count(&design, 0, &data.towels, &mut HashMap::with_capacity(design.len()));
    }
    Ok(res)
}

fn dfs_count(design: &[u8], index: usize, towels: &Vec<Vec<u8>>, cache: &mut HashMap<usize, i64>) -> i64 {
    if index == design.len() {
        return 1
    }

    if let Some(&count) = cache.get(&index) {
        return count
    }

    let mut res = 0;
    for towel in towels {
        if towel.len() <= design.len() - index {
            if towel == &design[index..index+towel.len()] {
                res += dfs_count(design, index + towel.len(), towels, cache);
            }
        }
    }
    cache.insert(index, res);
    res
}