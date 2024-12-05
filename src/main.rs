use anyhow::Ok;
use clap::{Parser, Subcommand};

mod day1;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
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



fn main() -> anyhow::Result<()> {
    
    let args = Args::parse();
    
    // ./run.sh day1 --one --two
    if let Some(command) = args.command {
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
                println!("Ran day 4");
            },
            Commands::day5 { one, two } => {
                println!("Ran day 5");
            },
        }
    }
    Ok(())
}
