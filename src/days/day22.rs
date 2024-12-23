

use std::{collections::{HashMap, VecDeque}, fs, io::{BufRead, BufReader}};
use anyhow;

struct Data {
    secrets: Vec<i64>
}

impl Data {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day22.txt")?;
        let mut reader = BufReader::new(file);
        let mut secrets = Vec::new();

        for line in reader.lines() {
            let line = line?;
            secrets.push(line.parse::<i64>()?);
        }

        Ok(Self{secrets})
    }
}

pub fn part_one() -> anyhow::Result<i64> {
    let mut data = Data::parse_input()?;
    let mut res = 0;

    for mut secret in data.secrets {
        for _ in 0..2000 {
            secret = sequence(secret);
        }
        res += secret;
    }
    Ok(res)
}

pub fn part_two() -> anyhow::Result<i64> {
    let mut data = Data::parse_input()?;
    let mut all_secrets = HashMap::new();
    
    for mut secret in data.secrets {
        let mut this_secret = HashMap::new();
        let mut prev_seq = VecDeque::with_capacity(4);
        let mut prev = last_digit(secret);
        for _ in 0..2000 {
            secret = sequence(secret);
            let last = last_digit(secret);
            let diff = last - prev;
            if prev_seq.len() < 3 {
                prev_seq.push_back(diff);
            } else if prev_seq.len() == 3 {
                prev_seq.push_back(diff);
                if !this_secret.contains_key(&prev_seq) {
                    this_secret.insert(prev_seq.clone(), last);
                }
            }
            else {
                prev_seq.pop_front();
                prev_seq.push_back(diff);
                if !this_secret.contains_key(&prev_seq) {
                    this_secret.insert(prev_seq.clone(), last);
                }
            }
            prev = last;
        }

        for (key, value) in this_secret {
            *all_secrets.entry(key).or_insert(0) += value;
        }
    }

    let mut res = all_secrets
        .into_iter()
        .map(|(_, value)| value)
        .collect::<Vec<i64>>();
    
    res.sort_by(|a,b|b.cmp(a));
    Ok(res[0])
}

fn last_digit(val: i64) -> i64 {
    let mut s = val.to_string();
    if let Some(last) = s.pop() {
        return last.to_digit(10).unwrap() as i64
    }
    0
}

fn sequence(secret: i64) -> i64 {
    step3(step2(step1(secret)))
}

fn step1(secret: i64) -> i64 {
    let temp = mix(secret * 64, secret);
    prune(temp)
    
}
fn step2(secret: i64) -> i64 {
    let temp = mix(secret / 32, secret);
    prune(temp)
}

fn step3(secret: i64) -> i64 {
    let temp = mix(secret * 2048, secret);
    prune(temp)
}

fn prune(val: i64) -> i64 {
    val % 16777216
}

fn mix(given: i64, secret: i64) -> i64 {
    given ^ secret
}