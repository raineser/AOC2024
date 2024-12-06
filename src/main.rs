use anyhow::Ok;
use clap::{Parser};
use commands::{Args, Commands, handle_command};


mod day1;
mod day4;
mod day5;

mod commands;

fn main() -> anyhow::Result<()> {
    
    let args = Args::parse();

    if let Some(command) = args.command {
        handle_command(command)?;
    }
    Ok(())
}
