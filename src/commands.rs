use clap::{Parser, Subcommand};
use crate::days::*;

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
    },
    day13 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day14 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day15 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day16 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    }, 
    day17 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day18 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day19 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day20 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day21 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day22 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day23 {
        #[clap(long = "one")]
        one: bool,
        #[clap(long = "two")]
        two: bool
    },
    day24 {
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
        Commands::day13 { one, two } => {
            if one {
                println!("{}", day13::part_one()?);
            }
            if two {
                println!("{}", day13::part_two()?);
            }
        },
        Commands::day14 { one, two } => {
            if one {
                println!("{}", day14::part_one()?);
            }
            if two {
                println!("{}", day14::part_two()?);
            }
        },
        Commands::day15 { one, two } => {
            if one {
                println!("{}", day15::part_one()?);
            }
            if two {
                println!("{}", day15::part_two()?);
            }
        },
        Commands::day16 { one, two } => {
            if one {
                println!("{}", day16::part_one()?);
            }
            if two {
                println!("{}", day16::part_two()?);
            }
        },
        Commands::day17 { one, two } => {
            if one {
                println!("{}", day17::part_one()?);
            }
            if two {
                println!("{}", day17::part_two()?);
            }
        },
        Commands::day18 { one, two } => {
            if one {
                println!("{}", day18::part_one()?);
            }
            if two {
                println!("{:?}", day18::part_two()?);
            }
        },
        Commands::day19 { one, two } => {
            if one {
                println!("{}", day19::part_one()?);
            }
            if two {
                println!("{:?}", day19::part_two()?);
            }
        },
        Commands::day20 { one, two } => {
            if one {
                println!("{}", day20::part_one()?);
            }
            if two {
                println!("{:?}", day20::part_two()?);
            }
        },
        Commands::day21 { one, two } => {
            if one {
                println!("{}", day21::part_one()?);
            }
            if two {
                println!("{:?}", day21::part_two()?);
            }
        },
        Commands::day22 { one, two } => {
            if one {
                println!("{}", day22::part_one()?);
            }
            if two {
                println!("{}", day22::part_two()?);
            }
        },
        Commands::day23 { one, two } => {
            if one {
                println!("{}", day23::part_one()?);
            }
            if two {
                println!("{}", day23::part_two()?);
            }
        },
        Commands::day24 { one, two } => {
            if one {
                println!("{}", day24::part_one()?);
            }
            if two {
                println!("{}", day24::part_two()?);
            }
        },
        
    }
    Ok(())
}