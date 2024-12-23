use std::{collections::{HashMap, HashSet, VecDeque}, fs::{self}, io::{BufRead, BufReader}};

use anyhow::{self, Ok};

struct Data {
    adj: HashMap<String, HashSet<String>>,
    t_pcs: Vec<String>
}

impl Data {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day23.txt")?;
        let reader = BufReader::new(file);
        let mut adj = HashMap::new();
        let mut t_pcs = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if let Some((left, right)) = line.split_once('-') {
                adj.entry(left.to_string()).or_insert(HashSet::new()).insert(right.to_string());
                adj.entry(right.to_string()).or_insert(HashSet::new()).insert(left.to_string());

                if left.starts_with('t') {
                    t_pcs.push(left.to_string());
                }
                if right.starts_with('t') {
                    t_pcs.push(right.to_string());
                }
            }
        }
        Ok(Self{adj, t_pcs})
    }
}

pub fn part_one() -> anyhow::Result<i64> {
    let data = Data::parse_input()?;
    let mut lans= HashSet::new();

    for pc in &data.t_pcs {
        let mut q = VecDeque::new();
        q.push_back((pc.clone(), 0, vec![pc.clone()]));

        while let Some((node, count, mut path)) = q.pop_front() {
            if &node == pc && count == 3 {
                path.pop();
                path.sort();
                lans.insert(path);
                continue;
            }
            if count == 3 {
                continue;
            }
            if let Some(neighbors) = data.adj.get(&node) {
                for neighbor in neighbors {
                    if neighbor != path.last().unwrap() {
                        let mut new_path = path.clone();
                        new_path.push(neighbor.clone());
                        q.push_back((neighbor.clone(), count + 1, new_path));
                    }
                }
            }
        }
    }
    Ok(lans.len() as i64)
}


pub fn part_two() -> anyhow::Result<i64> {
    todo!()
}