use std::{fs, io::{BufRead, BufReader}};
use std::collections::{HashMap, HashSet};
use anyhow::{self, Context};
use std::cmp;

struct Data {
    letters: HashMap<char, Vec<(i32,i32)>>,
    rows: i32,
    cols: i32
}

impl Data {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day8.txt")?;
        let mut reader = BufReader::new(file);
        let mut letters = HashMap::new();
        let mut rows = 0;
        let mut cols = 0;
        for (row, line) in reader.lines().enumerate() {
            let line = line?;
            for (col, c) in line.char_indices() {
                cols = cmp::max(col, cols);
                if c == '.' {
                    continue;
                }
                letters.entry(c).or_insert(Vec::new()).push((row as i32, col as i32));
            }
            rows = cmp::max(row, rows);
        }
        Ok(Self{ letters, rows: rows as i32 , cols: cols as i32 })
    }
}

pub fn part_one() -> anyhow::Result<i32> {
    let data = Data::parse_input()?;
    let mut valid = HashSet::new();
    for (key, value) in &data.letters{
        for i in 0..value.len() {
            for j in i+1..value.len() {
                
                let (top_row, top_col) = value[i];
                let (bottom_row, bottom_col) = value[j];
            
                if top_row == bottom_row{ // -
                    let col_dif = bottom_col - top_col;
                    let mut i_col = col_dif;

                    while top_col - i_col >= 0 {
                        valid.insert((top_row, top_col - i_col));
                        i_col += col_dif;
                    }

                    i_col = col_dif;

                    while  i_col + bottom_col <= data.cols {
                        valid.insert((top_row, i_col + bottom_col));
                        i_col += col_dif;
                    
                    }
                } else if top_col == bottom_col { // |
                    let row_diff = bottom_row - top_row;

                    let mut i_row = row_diff;
                    
                    while top_row - i_row >= 0 {
                        valid.insert((top_row - i_row, top_col));
                        i_row += row_diff;

                    }
                    i_row = row_diff;
                    while i_row + bottom_row <= data.rows {
                        valid.insert((i_row + bottom_row, top_col));
                        i_row += row_diff;

                    }

                } else if top_col > bottom_col { // /
                    let col_dif = top_col - bottom_col;
                    let row_diff = bottom_row - top_row;

                    let mut i_row = row_diff;
                    let mut i_col =  col_dif;

                    while  top_row - i_row >= 0 && top_col + i_col <= data.cols  {
                        valid.insert((top_row - i_row, top_col + i_col));
                        i_row += row_diff;
                        i_col += col_dif;
                    }

                    i_row = row_diff;
                    i_col = col_dif;

                    while bottom_row + i_row <= data.rows && bottom_col - i_col >= 0 {
                        valid.insert((bottom_row + i_row, bottom_col - i_col));
                        i_row += row_diff;
                        i_col += col_dif;
                    }
                } else  { // \   top_col < bottom_col
                    let mut col_dif = bottom_col - top_col;
                    let mut row_diff = bottom_row - top_row;

                    let mut i_row = row_diff;
                    let mut i_col = col_dif;

                    while top_row - i_row >= 0 && top_col - i_col >= 0 {
                        valid.insert((top_row - i_row, top_col - i_col));
                        i_row += row_diff;
                        i_col += col_dif;
                    }

                    i_row = row_diff;
                    i_col = col_dif;

                    while bottom_row + i_row <= data.rows && bottom_col + i_col <= data.cols {
                        valid.insert((bottom_row + i_row, bottom_col + i_col));
                        i_row += row_diff;
                        i_col += col_dif;
                    }
                }
            }
        }
    }
    for (key, value) in data.letters {
        if value.len() > 1 {
            for (i,j)in value {
                valid.insert((i, j));
            }
        }
    }
    Ok(valid.len() as i32)
}

pub fn part_two() -> anyhow::Result<i32> {
    todo!()
}