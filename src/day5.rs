use anyhow::{self, Context};
use std::{any, cmp::Ordering, collections::HashMap, fs, io::{BufRead, BufReader}};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Page {
    val: i32
}

impl Page {
    fn new(val: i32) -> Self {
        Self{val}
    }
}

impl PageOrdering for Page {
    fn valid_ordering(&self, other: &Self, adj: &HashMap<Page, Vec<Page>>) -> bool {
        if let Some(pages) = adj.get(self) {
            for page in pages {
                if page == other {
                    return false
                }
            }
        }
        true
    }
    fn order(&self, other: &Self, adj: &HashMap<Page, Vec<Page>>) -> Ordering {
        if let Some(pages) = adj.get(self) {
            for page in pages {
                if page == other {
                    return Ordering::Greater
                }
            }
        }
        if let Some(pages) = adj.get(other) {
            for page in pages {
                if page == self {
                    return Ordering::Less
                }
            }
        }
        Ordering::Equal
    }
}

trait PageOrdering {
    fn valid_ordering(&self, other: &Self, adj: &HashMap<Page, Vec<Page>>) -> bool;

    fn order(&self, other: &Self, adj: &HashMap<Page, Vec<Page>>) -> Ordering;
}

struct Data {
    adj: HashMap<Page, Vec<Page>>,
    updates: Vec<Vec<Page>>
}

impl Data {

    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day5.txt").context("Open input file")?;
        let mut reader = BufReader::new(file);
        let mut adj = HashMap::new();
        let mut updates = Vec::new();
        
        for line in reader.lines() {
            let line = line?;
            if let Some((left, right)) = line.split_once('|') {
                let left = Page::new(left.parse::<i32>().context("Parse the left")?);
                let right = Page::new(right.parse::<i32>().context("Parse the right")?);
                adj.entry(right).or_insert(Vec::new()).push(left);
                continue;
            } 

            let mut update = Vec::new();
            for value in line.split(',') {
                match value.parse::<i32>() {
                    Ok(val) => {
                        update.push(Page::new(val));
                    },
                    Err(_) => {break;}
                }
            }
            if update.len() > 0 {
                updates.push(update);
            }
        }
        Ok(Self{adj, updates})
    }
}


pub fn part_one() -> anyhow::Result<i32> {
    let data = Data::parse_input()?;
    
    let result = data.updates
        .iter()
        .filter(|update| update.is_sorted_by(|a,b| a.valid_ordering(b, &data.adj)))
        .map(|update| update[update.len() / 2].val)
        .sum();
    
    Ok(result)
}

pub fn part_two() -> anyhow::Result<i32> {
    let mut data = Data::parse_input()?;

    let result = data.updates
        .iter_mut()
        .filter(|update| !update.is_sorted_by(|a,b| a.valid_ordering(b, &data.adj)))
        .map(|update| {
            update.sort_by(|a,b|  a.order(b, &data.adj));
            update[update.len() / 2].val
        })
        .sum();

    Ok(result)
}