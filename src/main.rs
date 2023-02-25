use std::io;
use std::io::prelude::*;
use clap::{Command, Arg, ArgAction, value_parser};

fn main() -> io::Result<()> {
    let matches = Command::new("MapF(unction)")
                                .version("1.0")
                                .author("Anthony Cox")
                                .about("Maps functions to a list of numbers or evaluatable expressions")
                                .arg(
                                    Arg::new("accumulate")
                                    .long("accumulate")
                                    .alias("accum")
                                    .short('a')
                                    .help("accumulates all values into a single value")
                                    .action(ArgAction::SetTrue)
                                )
                                .arg(
                                    Arg::new("initializer")
                                    .long("init")
                                    .short('i')
                                    .help("initializes acculation value")
                                    .value_name("number")
                                    .allow_negative_numbers(true)
                                    .requires("accumulate")
                                    .value_parser(value_parser!(u64))
                                    .action(ArgAction::Set)
                                ).get_matches();

    let accumulate = matches.get_one::<bool>("accumulate").unwrap_or(&false);
    let mut total = matches.get_one::<u64>("initializer").unwrap_or(&0).to_owned();

    if *accumulate {
        for line in io::stdin().lock().lines() {
            let line = line?;
            total += line.parse::<u64>().unwrap(); 
        }
        println!("{total}");
    }

    Ok(())
}
