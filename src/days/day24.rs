use std::{collections::HashMap, fs, io::{BufRead, BufReader}, process::Output};

use anyhow;

#[derive(Debug)]
enum Gate {
    And(String, String, String),
    Or(String, String, String),
    Xor(String, String, String)
}

struct Data {
    program: Vec<Gate>,
    values: HashMap<String, bool>
}

impl Data {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day24.txt")?;
        let reader = BufReader::new(file);
        let mut values = HashMap::new();
        let mut program = Vec::new();

        for line in reader.lines() {
            let line = line?;

            if let Some((input, value)) = line.split_once(':') {
                values.insert(input.to_string(), value.trim().parse::<i64>()? != 0);
                continue;
            }
            let mut left = None;
            let mut right = None;
            let mut output = None;
            let mut gate = None;

            for item in line.split(' ') {
                match item {
                    "->" => {}, 
                    _ => {
                        if left.is_none() {
                            left = Some(item);
                        } else if gate.is_none() {
                            gate = Some(item)
                        } else if right.is_none() {
                            right = Some(item);
                        } else if output.is_none() {
                            output = Some(item);
                        } else {
                            anyhow::bail!("Something has gone wrong")
                        }
                    }
                }
            }
            if !left.is_none() && !right.is_none() && !output.is_none() && !gate.is_none() {
                let left = left.unwrap().to_string();
                let right= right.unwrap().to_string();
                let output= output.unwrap().to_string();
                
                match gate {
                    Some("AND") => {
                        program.push(Gate::And(left, right, output));
                    },
                    Some("XOR") => {
                        program.push(Gate::Xor(left, right, output));
                    },
                    Some("OR") => {
                        program.push(Gate::Or(left, right, output));
                    },
                    _ => {anyhow::bail!("Invalid gate")}
                }
            }
        }
        Ok(Self{values, program})
    }
}

pub fn part_one() -> anyhow::Result<i64> {
    let mut data = Data::parse_input()?;
    
    while data.program.len() > 0 {
        data.program.retain(|gate| {
            match gate {
                Gate::And(left,right ,output ) => {
                    if let (Some(l_value), Some(r_value)) = (data.values.get(left), data.values.get(right)) {
                        data.values.insert(output.clone(), l_value & r_value);
                        false
                    } else {
                        true
                    }
                },
                Gate::Or(left, right, output) => {
                    if let (Some(l_value), Some(r_value)) = (data.values.get(left), data.values.get(right)) {
                        data.values.insert(output.clone(), l_value | r_value);
                        false
                    } else {
                        true
                    }
                },
                Gate::Xor(left, right, output) => {
                    if let (Some(l_value), Some(r_value)) = (data.values.get(left), data.values.get(right)) {
                        data.values.insert(output.clone(), l_value ^ r_value);
                        false
                    } else {
                        true
                    }
                },
            }
        });
    }
    let mut z_values = Vec::new();
    for (key, value) in data.values {
        if key.starts_with('z') {
            z_values.push((key, value));
        }
    }
    z_values.sort_by(|a,b| b.cmp(a));
    let mut res = 0;
    for (_, val) in z_values {
        res <<= 1;
        res |= val as i64
    }
    Ok(res)
}

pub fn part_two() -> anyhow::Result<i64> {
    todo!()
}