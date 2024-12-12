use clap::{Parser, Subcommand};
use crate::{day1, day3,  day4, day5, day6, day7, day8, day2, day9, day10, day11, day12};

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
    },
    day6 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day7 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day8 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day9 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day10 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day11 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day12 {
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
            if one {
                println!("{}", day2::part_one()?);
            }
            if two {
                println!("{}", day2::part_two()?);
            }
        },
        Commands::day3 { one, two } => {
            if one {
                println!("{}", day3::part_one()?);
            }
            if two {
                println!("{}", day3::part_two()?);
            }
        },
        Commands::day4 { one, two } => {
            if one {
                println!("{}", day4::part_one()?);
            }
            if two {
                println!("{}", day4::part_two()?);
            }
        },
        Commands::day5 { one, two } => {
            if one {
                println!("{}", day5::part_one()?);
            }
            if two {
                println!("{}", day5::part_two()?);
            }
        },
        Commands::day6 { one, two } => {
            if one {
                println!("{}", day6::part_one()?);
            }
            if two {
                println!("{}", day6::part_two()?);
            }
        },
        Commands::day7 { one, two } => {
            if one {
                println!("{}", day7::part_one()?);
            }
            if two {
                println!("{}", day7::part_two()?);
            }
        },
        Commands::day8 { one, two } => {
            if one {
                println!("{}", day8::part_one()?);
            }
            if two {
                println!("{}", day8::part_two()?);
            }
        },
        Commands::day9 { one, two } => {
            if one {
                println!("{}", day9::part_one()?);
            }
            if two {
                println!("{}", day9::part_two()?);
            }
        },
        Commands::day10 { one, two } => {
            if one {
                println!("{}", day10::part_one()?);
            }
            if two {
                println!("{}", day10::part_two()?);
            }
        },
        Commands::day11 { one, two } => {
            if one {
                println!("{}", day11::part_one()?);
            }
            if two {
                println!("{}", day11::part_two()?);
            }
        },
        Commands::day12 { one, two } => {
            if one {
                println!("{}", day12::part_one()?);
            }
            if two {
                println!("{}", day12::part_two()?);
            }
        },
        
    }
    Ok(())
}