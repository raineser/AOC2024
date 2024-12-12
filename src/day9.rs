use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::{fs, io::BufReader};
use std::io::Read;
use anyhow;

#[derive(Eq, PartialEq)]
enum Slot {
    File(i64),
    Free
}

struct Data {
    disk: Vec<Slot>
}

impl Data {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day9.txt")?;
        let mut reader = BufReader::new(file);
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf);
        buf.push('0' as u8);

        let mut disk = Vec::new();
        for (id, chunk) in buf.chunks_exact(2 ).enumerate() {
            if let Some(size) = (chunk[0] as char).to_digit(10) {
                for _ in 0..size {
                    disk.push(Slot::File((id as i64)))
                }
            }
            if let Some(size) = (chunk[1] as char).to_digit(10) {
                for _ in 0..size {
                    disk.push(Slot::Free)
                }
            }
        }
        Ok(Data{disk})
    }
}



pub fn part_one() -> anyhow::Result<i64> {
    let mut data = Data::parse_input()?;
    
    let mut index = 0;
    while index < data.disk.len() {
        if data.disk[index] != Slot::Free {
            index += 1;
            continue;
        }
        if let Some(last) = data.disk.pop() {
            match last {
                Slot::File(id) => {
                    data.disk[index] = last;
                    index += 1;
                },
                Slot::Free => {
                    continue;
                },
            }
        }
    }

    let res = data.disk
        .iter()
        .enumerate()
        .map(|(index, slot)| {
            match slot {
                Slot::File(id) => {
                    (index as i64) * id
                },
                Slot::Free => {0},
            }
        })
        .sum();

    Ok(res)
}


pub fn part_two() -> anyhow::Result<i64> {
    let mut data = Data_Two::parse_input()?;
    let mut remaining_free  = BTreeMap::new();

    for &(index, size) in &data.frees {
        remaining_free.insert(index, size);
    }

    let mut disk = Vec::new();

    for &(id, index, size) in data.files.iter().rev() {

        let free = remaining_free
            .iter()
            .take_while(|&(&free_index, _)| free_index < id)
            .find(|&(_, &s)| s >= size);

        if let Some((&free_id, &free_size)) = free {
            disk.push((id, free_id, size));
            remaining_free.remove(&free_id);
            if free_size > size {
                remaining_free.insert(free_id + size, free_size - size);
            }
        
        } else {
            disk.push((id, index, size));
        }

    }

    let res = disk
        .iter()
        .map(|&(file_id, file_block_id, file_size)| {
            (file_block_id..(file_block_id + file_size))
                .map(|block_id| file_id * block_id)
                .sum::<i64>()
        })
        .sum();
    Ok(res)

}

struct Data_Two {
    files: Vec<(i64, i64, i64)>, 
    frees: Vec<(i64, i64)>
}

impl Data_Two {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day9.txt")?;
        let mut reader = BufReader::new(file);
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf);
        buf.push('0' as u8);
        
        let mut files = Vec::new();
        let mut frees = Vec::new();
        let mut index = 0;
        for (id, chunk) in buf.chunks_exact(2).enumerate() {
            if let Some(size) = (chunk[0] as char).to_digit(10) {
                if size > 0 {
                    files.push((id as i64, index as i64, size as i64));
                    index += size;
                }
            }
            if let Some(size) = (chunk[1] as char).to_digit(10) {
                if size > 0 {
                    frees.push((index as i64 ,size as i64));
                    index += size;
                }
            }
        }
        Ok(Self{files, frees})
    }
}