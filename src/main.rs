mod funcgen;
mod parser;

use std::io;
use std::io::prelude::*;
use std::error::Error;
use clap::{Command, Arg, ArgAction, value_parser};

use pest::Parser;
use parser::{FunctionParser, Rule, eval};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("MapF(unction)")
                                .version("1.0")
                                .author("Anthony Cox")
                                .about("Applies functions to a list of numbers, transforming or filtering them")
                                .arg(
                                    Arg::new("reduce")
                                    .long("reduce")
                                    .short('r')
                                    .help("reduces all lines into a single value")
                                    .action(ArgAction::SetTrue)
                                )
                                .arg(
                                    Arg::new("initializer")
                                    .long("init")
                                    .short('i')
                                    .help("initializes accumulation value")
                                    .value_name("NUMBER")
                                    .allow_negative_numbers(true)
                                    .requires("reduce")
                                    .value_parser(value_parser!(i64))
                                    .action(ArgAction::Set)
                                )
                                .arg(
                                    Arg::new("filter-input")
                                    .long("filter-input")
                                    .short('f')
                                    .value_name("PREDICATE")
                                    .help("filter input with quoted predicate (before mapping)")
                                    .action(ArgAction::Set)
                                )
                                .arg(
                                    Arg::new("filter-output")
                                    .long("filter-output")
                                    .short('F')
                                    .value_name("PREDICATE")
                                    .help("filter output with quoted predicate (after mapping)")
                                    .action(ArgAction::Set)
                                )
                                .get_matches();

    let reduce = matches.get_one::<bool>("reduce").unwrap_or(&false);
    let mut total = matches.get_one::<u64>("initializer").unwrap_or(&0).to_owned();

    let function = FunctionParser::parse(Rule::function, "2")?;
    println!("2 * (10 - 30) = {}", parser::eval(function));

    //for line in io::stdin().lock().lines() {
    //   let line = line?;

        //1. Filter input before mapping
        //2. Map the function to each line of the input
        //3. Filter after mapping
    //}


    Ok(())
}
