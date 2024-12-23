use anyhow;
use std::io::{BufRead, BufReader};
use std::{fs, usize};
use std::collections::{BinaryHeap, HashMap, VecDeque, HashSet};
use std::cmp::{self, max, Reverse};


/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
*/

/*
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
 */

struct Data {
    key_adj: HashMap<char, Vec<(char, char)>>,
    dir_adj: HashMap<char, Vec<(char, char)>>,
    codes: Vec<(Vec<char>, i64)>
}

impl Data {

    fn parse_input() -> anyhow::Result<Self> {
        let mut key_adj = build_key_adj();
        let mut dir_adj = build_dir_adj();
        let mut codes = Vec::new();

        let file = fs::File::open("./inputs/day21.txt")?;
        let mut reader = BufReader::new(file);
        
        for line in reader.lines() {
            let line = line?;
            if let Some(num) = line.strip_suffix('A') {
                let value = num.parse::<i64>()?;
                let mut code = num.to_string();
                code.push('A');
                codes.push((code.chars().collect(), value));
            }
        }
        Ok(Self{key_adj, dir_adj, codes})
    }
    
}

pub fn part_one() -> anyhow::Result<i64> {
    let data = Data::parse_input()?;
    let mut res = 0;
    
    for (code, value) in data.codes {
        let mut remaining_paths = Vec::new();
        // first pass
        dfs_path(&data.key_adj, &code, 'A', 0, &mut usize::MAX, Vec::new(), &mut remaining_paths, &mut HashMap::new());

        let mut final_min = 0;
        for i in 0..2 {
            let mut curr_min = usize::MAX;
            let mut best_paths = Vec::new();
            for code in remaining_paths {
                let mut bests = Vec::new();
                dfs_path(&data.dir_adj, &code, 'A', 0, &mut curr_min, Vec::new(), &mut bests, &mut HashMap::new());
                best_paths.append(&mut bests);
                best_paths.retain(|path| path.len() == curr_min);
            }
            remaining_paths = best_paths;
            final_min = cmp::max(curr_min, final_min);
            println!("{i}");
        }

        res += final_min as i64 * value;
    }
    Ok(res)
}

/* 
fn dfs(adj: &HashMap<char, Vec<(char, char)>>, code: &Vec<char>, count: usize, max_count: usize) -> usize {
    if count == max_count {
        return 0
    }

    let mut remaining_paths = Vec::new();
    let mut min = 0;
    dfs_path(adj, code, 'A', 0, min, path, best_paths, cache);
    

    todo!()
}
*/


fn dfs_path(adj: &HashMap<char, Vec<(char, char)>>, code: &Vec<char>, curr: char, 
    index: usize, min: &mut usize, mut path: Vec<char>, best_paths: &mut Vec<Vec<char>>, cache: &mut HashMap<usize, usize> ) 
{
    if index == code.len() {
        if path.len() < *min {
            *min = path.len();
            best_paths.clear();
        }
        if path.len() == *min {
            best_paths.push(path);
        }
        return
    }

    if let Some(&best) = cache.get(&index) {
        if best < path.len() {
            return
        }
    }

    for mut p_path in shortest_num_path(adj, curr, code[index]).iter_mut() {
        let mut new_path = path.clone();
        new_path.append(&mut p_path);
        dfs_path(adj, code, code[index], index + 1, min, new_path, best_paths, cache);
    }

    cache.insert(index, *min);

}




fn shortest_num_path(adj: &HashMap<char, Vec<(char, char)>>, start: char, end: char) -> Vec<Vec<char>> {
    let mut q = BinaryHeap::new();
    q.push((Reverse(0), start, Vec::new()));
    let mut visit = HashMap::new();
    let mut min = usize::MAX;
    let mut shorest_paths= Vec::new();

    while let Some((count, c, mut path)) = q.pop() {
        if c == end {
            if path.len() < min {
                min = path.len();   
                shorest_paths = Vec::new();
            }
            if path.len() == min {
                path.push('A');
                shorest_paths.push(path)
            }
            continue;
        }
        if let Some(&best) = visit.get(&(c)) {
            if best < count.0 {
                continue;
            }
        }
        visit.insert(c, count.0);
        if let Some(nums) = adj.get(&c) {
            for (next_c, dir) in nums {
                let mut new_path = path.clone();
                new_path.push(*dir);
                q.push((Reverse(count.0 + 1), *next_c, new_path));
                continue;
            }
        }
    }
    shorest_paths
}


pub fn part_two() -> anyhow::Result<i64> {
    todo!()
}

fn build_key_adj() -> HashMap<char, Vec<(char, char)>> {
    let mut key_adj = HashMap::new();
    // A
    let mut entry = key_adj.entry('A').or_insert(Vec::new());
    entry.push(('0', '<'));
    entry.push(('3', '^'));
    // 0
    let mut entry   = key_adj.entry('0').or_insert(Vec::new());
    entry.push(('A', '>'));
    entry.push(('2', '^'));
    // 1 
    let mut entry   = key_adj.entry('1').or_insert(Vec::new());
    entry.push(('2', '>'));
    entry.push(('4', '^'));
    // 2
    let mut entry   = key_adj.entry('2').or_insert(Vec::new());
    entry.push(('1', '<'));
    entry.push(('5', '^'));
    entry.push(('3', '>'));
    entry.push(('0', 'v'));

    // 3
    let mut entry   = key_adj.entry('3').or_insert(Vec::new());
    entry.push(('A', 'v'));
    entry.push(('2', '<'));
    entry.push(('6', '^'));

    // 4
    let mut entry   = key_adj.entry('4').or_insert(Vec::new());
    entry.push(('7', '^'));
    entry.push(('5', '>'));
    entry.push(('1', 'v'));

    // 5
    let mut entry   = key_adj.entry('5').or_insert(Vec::new());
    entry.push(('8', '^'));
    entry.push(('4', '<'));
    entry.push(('6', '>'));
    entry.push(('2', 'v'));
    // 6
    let mut entry   = key_adj.entry('6').or_insert(Vec::new());
    entry.push(('9', '^'));
    entry.push(('3', 'v'));
    entry.push(('5', '<'));
    // 7
    let mut entry   = key_adj.entry('7').or_insert(Vec::new());
    entry.push(('8', '>'));
    entry.push(('4', 'v'));
    // 8
    let mut entry   = key_adj.entry('8').or_insert(Vec::new());
    entry.push(('7', '<'));
    entry.push(('9', '>'));
    entry.push(('5', 'v'));
    // 9
    let mut entry   = key_adj.entry('9').or_insert(Vec::new());
    entry.push(('8', '<'));
    entry.push(('6', 'v'));

    key_adj 
}

fn build_dir_adj() -> HashMap<char, Vec<(char, char)>>  {
    let mut dir_adj = HashMap::new();

    // A
    let mut entry = dir_adj.entry('A').or_insert(Vec::new());
    entry.push(('^', '<'));
    entry.push(('>', 'v'));

    // ^
    let mut entry = dir_adj.entry('^').or_insert(Vec::new());
    entry.push(('A', '>'));
    entry.push(('v', 'v'));


    // >
    let mut entry = dir_adj.entry('>').or_insert(Vec::new());
    entry.push(('A', '^'));
    entry.push(('v', '<'));

    // v
    let mut entry = dir_adj.entry('v').or_insert(Vec::new());
    entry.push(('>', '>'));
    entry.push(('^', '^'));
    entry.push(('<', '<'));

    // <
    let mut entry = dir_adj.entry('<').or_insert(Vec::new());
    entry.push(('v', '>'));

    dir_adj

}
