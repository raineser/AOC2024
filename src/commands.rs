use clap::{Parser, Subcommand};
use crate::{day1, day4, day5};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    day1{
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day2{
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day3 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day4 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day5 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    }

}

pub fn handle_command(command: Commands) -> anyhow::Result<()> {
    match command {
        Commands::day1 { one, two } => {
            if one {
                println!("{}", day1::part_one()?);
            }
            if two {
                println!("{}", day1::part_two()?);
            }
        },
        Commands::day2 { one, two } => {
            println!("Ran day 2");
        },
        Commands::day3 { one, two } => {
            println!("Ran day 3");
        },
        Commands::day4 { one, two } => {
            
        },
        Commands::day5 { one, two } => {
            if one {
                println!("{}", day5::part_one()?);
            }
            if two {
                println!("{}", day5::part_two()?);
            }
        },
    }
    Ok(())
}