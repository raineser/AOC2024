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
    let mut res = 0;
    let mut disk = Vec::new();
    
    while let Some(mut file) = data.files.pop() {
        let next_free = data.frees
            .iter()
            .take_while(|&(&free_index, &free)| {
                free_index < file.disk_index
            })
            .find(|(&free_index, &free)| free.size >= file.size);

        if let Some((&free_index, &free)) = next_free {
            file.disk_index = free_index;
            disk.push(file);
            data.frees.remove(&free_index);
            if free.size > file.size {
                data.frees.insert(free_index + file.size, Free{size: free.size - file.size});
            }
        } else {
            disk.push(file);
        }
    }

    let res = disk.iter()
        .map(|file| file.sum())
        .sum();
    Ok(res)

}

#[derive(Copy, Clone)]
struct File {
    id: i64,
    disk_index: i64,
    size: i64
}

impl File {
    fn sum(&self) -> i64{
        let mut index = self.disk_index;
        let mut sum = 0;
        for _ in 0..self.size {
            sum += (index * self.id);
            index += 1;
        }
        sum
    }
}

#[derive(Copy, Clone)]
struct Free {
    size: i64
}

struct Data_Two {
    files: Vec<File>, 
    frees: BTreeMap<i64, Free>
}

impl Data_Two {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day9.txt")?;
        let mut reader = BufReader::new(file);
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf);
        buf.push('0' as u8);
        
        let mut files = Vec::new();
        let mut frees = BTreeMap::new();
        let mut disk_index = 0;
        for (id, chunk) in buf.chunks_exact(2).enumerate() {
            if let Some(size) = (chunk[0] as char).to_digit(10) {
                if size > 0 {
                    files.push(File{id: id as i64, disk_index: disk_index as i64, size: size as i64});
                    disk_index += size;
                }
            }
            if let Some(size) = (chunk[1] as char).to_digit(10) {
                if size > 0 {
                    frees.insert(disk_index as i64, Free{size: size as i64});
                    disk_index += size;
                }
            }
        }
        Ok(Self{files, frees})
    }
}