use std::{fs, io::{BufRead, BufReader}};
use anyhow::{Context, self};

struct Data {
    totals: Vec<i64>,
    operators: Vec<Vec<i64>>
}

impl Data {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day7.txt")?;
        let mut reader = BufReader::new(file);
        let mut totals = Vec::new();
        let mut operators = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if let Some((total, ops)) = line.split_once(':') {
                totals.push(total.parse::<i64>()?);
                let mut operator_values = Vec::new();
                for op in ops.split(' ') {
                    match op.parse::<i64>() {
                        Ok(num) => {
                            operator_values.push(num);
                        },
                        Err(_) => {
                            continue
                        }
                    }
                }
                operators.push(operator_values);
            }
        }
        Ok(Self{totals, operators})
    }
}

pub fn part_one() -> anyhow::Result<i64> {
    let data = Data::parse_input()?;

    let mut res = 0;
    for (total, operator) in data.totals.iter().zip(data.operators.iter()) {
        if dfs_1(*total, operator, 0, 0) {
            res += total;
        }
    }
    Ok(res)
}

pub fn part_two() -> anyhow::Result<i64> {
    let data = Data::parse_input()?;

    let mut res = 0;
    for (total, operator) in data.totals.iter().zip(data.operators.iter()) {
        if dfs_2(*total, operator, 0, 0) {
            res += total;
        }
    }
    Ok(res)
}

fn dfs_1(total: i64, operators: &Vec<i64>, index: usize, curr_total: i64) -> bool {
    if total == curr_total && index == operators.len() {
        return true;
    } else if index == operators.len() {
        return false
    }

    let mut result = false;
    if curr_total == 0 {
        result |= 
            dfs_1(total, operators, index + 1, operators[index]);
    } else {
        result |= 
            dfs_1(total, operators, index + 1, curr_total + operators[index]) ||
            dfs_1(total, operators, index + 1, curr_total * operators[index]);
            
    }
    result
}

fn dfs_2(total: i64, operators: &Vec<i64>, index: usize, curr_total: i64) -> bool {
    if total == curr_total && index == operators.len() {
        return true;
    } else if index == operators.len() {
        return false
    }
    
    let mut result = false;
    if curr_total == 0 {
        result |= 
            dfs_2(total, operators, index + 1, operators[index]);
    } else {
        let mut c = curr_total.to_string();
        c.push_str(&operators[index].to_string());
        let concat = c.parse::<i64>().expect("Should always be valid");
        result |= 
            dfs_2(total, operators, index + 1, curr_total + operators[index]) ||
            dfs_2(total, operators, index + 1, curr_total * operators[index]) ||
            dfs_2(total, operators, index + 1, concat);
    }
    result
}