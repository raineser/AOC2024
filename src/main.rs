use anyhow;
use clap::{Parser};
use commands::{Args,handle_command};
mod days;
mod commands;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if let Some(command) = args.command {
        handle_command(command)?;
    }
    Ok(())
}
