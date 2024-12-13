use std::{fs, io::BufReader};
use anyhow::{Context, self};
use std::io::Read;
use regex::Regex;

struct Data {}

impl Data {
    fn parse_input() -> anyhow::Result<String> {
        let file = fs::File::open("./inputs/day3.txt")?;
        let mut reader = BufReader::new(file);
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        Ok(buf)
    }
}

pub fn part_one() -> anyhow::Result<i32>  {
    let data = Data::parse_input()?;

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let res = re.captures_iter(&data)
        .map(|cap| {
            match (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
                (Ok(a), Ok(b)) => {
                    a * b
                },
                _ => {0}
            }
        })
        .sum();
    Ok(res)
}

pub fn part_two() -> anyhow::Result<i32> {
    let data = Data::parse_input()?;

    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();
    let mut valid = true;

    let res = re.captures_iter(&data)
        .filter_map(|cap| {
            if let (Some(a), Some(b)) = (cap.get(1), cap.get(2)) {
                if valid {
                    match (a.as_str().parse::<i32>(), b.as_str().parse::<i32>()) {
                        (Ok(a), Ok(b)) => {
                            Some(a * b)
                        },
                        _ => {None}
                    }
                } else {
                    None
                }
            } else {
                match &cap[0] {
                    "don't()" => valid = false,
                    "do()" => valid = true,
                    _ => {}
                }
                None
            }
        })
        .sum();

    Ok(res)
}