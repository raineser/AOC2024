use anyhow::Ok;
use clap::{Parser};
use commands::{Args, Commands, handle_command};


mod day1;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day2;
mod day9;
mod day3;
mod day10;
mod day11;
mod day12;

mod commands;

fn main() -> anyhow::Result<()> {
    
    let args = Args::parse();

    if let Some(command) = args.command {
        handle_command(command)?;
    }
    Ok(())
}
