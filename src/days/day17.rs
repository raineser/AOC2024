use std::{fs, io::{BufRead, BufReader}};
use anyhow;

#[derive(Debug, Clone, Copy)]
enum opcode {
    adv(Combo),
    bxl(Combo),
    bst(Combo),
    jnz(Combo),
    bxc(Combo),
    out(Combo),
    bdv(Combo),
    cdv(Combo)
}
#[derive(Debug, Clone, Copy)]
enum Combo {
    Register(usize),
    Literal(i64)
}

#[derive(Debug)]
struct Cpu {
    registers:[i64;3],
    program: Vec<opcode>,
    pc: usize
}

impl Cpu {
    fn parse_input() -> anyhow::Result<Self> {
        let file = fs::File::open("./inputs/day17.txt")?;
        let mut reader = BufReader::new(file);
        let mut registers = [0;3];
        let mut numbers = Vec::new();

        for (index, line) in reader.lines().enumerate() {
            let line= line?;
            if index < 3 {
                if let Some(colon) = line.find(':') {
                    registers[index] = line[colon + 1..].trim().parse::<i64>()?;
                }
            } else {
                if let Some((text, nums)) = line.split_once(':') {
                    for num in nums.split(',') {
                        numbers.push(num.trim().parse::<i64>()?);
                    }
                }
            }           
        }
        
        let mut program = Vec::new();
        for instruction in numbers.chunks_exact(2) {
            let combo: anyhow::Result<Combo> = match instruction[1] {
                0..=3 => {
                    Ok(Combo::Literal(instruction[1]))
                },
                4..=6 => {
                    Ok(Combo::Register(instruction[1] as usize - 4))
                },
                _ => {anyhow::bail!("Invalid combo")}
            };
            
            let opcode: anyhow::Result<opcode> = match instruction[0] {
                0 => {
                    Ok(opcode::adv(combo?))
                },
                1 => {
                    Ok(opcode::bxl(combo?))
                },
                2 => {
                    Ok(opcode::bst(combo?))
                },
                3 => {
                    Ok(opcode::jnz(combo?))
                },
                4 => {
                    Ok(opcode::bxc(combo?))
                }, 
                5 => {
                    Ok(opcode::out(combo?))
                }, 
                6 => {
                    Ok(opcode::bdv(combo?))
                }, 
                7 => {
                    Ok(opcode::cdv(combo?))
                },
                _ => {anyhow::bail!("Invalid opcode")}
            };
            program.push(opcode?);
        }
        let pc = 0;
        Ok(Self{registers, program, pc})
    } 
}

pub fn part_one() -> anyhow::Result<String> {
    let mut cpu = Cpu::parse_input()?;
    let mut res= String::new();
    
    while cpu.pc < cpu.program.len() {
        match cpu.program[cpu.pc] {
            opcode::adv(combo) => {
                let denominator = match combo {
                    Combo::Register(index) => {2_i64.pow(cpu.registers[index] as u32)},
                    Combo::Literal(value) => {2_i64.pow(value as u32)},
                };
                cpu.registers[0] /= denominator;
            },
            opcode::bxl(combo) => {
                let left = match combo {
                    Combo::Register(index) => {index as i64 + 4},
                    Combo::Literal(value) => {value},
                };
                cpu.registers[1] ^= left;
            },
            opcode::bst(combo) => {
                let left = match combo {
                    Combo::Register(index) => {cpu.registers[index] % 8},
                    Combo::Literal(value) => {value % 8}
                };
                cpu.registers[1] = left;
            },
            opcode::jnz(combo) => {
                if cpu.registers[0] > 0 {
                    let jump = match combo {
                        Combo::Register(index) => {(index as i64 + 4) / 2},
                        Combo::Literal(value) => {value / 2},
                    };
                    if jump % 2 != 0 {
                        anyhow::bail!("invalid jump")
                    }
                    cpu.pc = jump as usize;
                    continue;
                }
            },
            opcode::bxc(_) => {
                let b = cpu.registers[1];
                let c = cpu.registers[2];
                cpu.registers[1] = b ^ c;
            },
            opcode::out(combo) => {
                let output = match combo {
                    Combo::Register(index) => {cpu.registers[index] % 8},
                    Combo::Literal(value) => {value % 8},
                };
                res.push_str(&output.to_string());
                res.push(',');
            },
            opcode::bdv(combo) => {
                let denominator = match combo {
                    Combo::Register(index) => {2_i64.pow(cpu.registers[index] as u32)},
                    Combo::Literal(value) => {2_i64.pow(value as u32)},
                };
                cpu.registers[1] = cpu.registers[0] / denominator;
            },
            opcode::cdv(combo) => {
                let denominator = match combo {
                    Combo::Register(index) => {2_i64.pow(cpu.registers[index] as u32)},
                    Combo::Literal(value) => {2_i64.pow(value as u32)},
                };
                cpu.registers[2] = cpu.registers[0] / denominator;
            },
        }
        cpu.pc += 1;
    }
    res.pop();
    Ok(res)
}


pub fn part_two() -> anyhow::Result<i64> {
    let program = vec![2,4,1,5,7,5,1,6,0,3,4,6,5,5,3,0];
    let mut possible = vec![0];
    
    for i in (0..program.len()).rev() {
        let mut temp = Vec::new();
        for j in 0..8 {
            for val in &possible {
                let mut curr = val << 3;
                curr |= j;
                if hard_coded(curr) == program[i] {
                    temp.push(curr);
                }
            }
        }
        possible = temp;
    }
    possible.sort_unstable();
    Ok(possible[0])
}

fn hard_coded(a: i64) -> i64 {
    let mut b = a % 8;
    b = b ^ 5;
    let mut c = a / (2_i64.pow(b as u32));
    b = b ^ 6;
    b  = b ^ c;
    b % 8
}
