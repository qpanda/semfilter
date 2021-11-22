mod expression;
mod filter;
mod parser;
mod tokenizer;

use anyhow::{Context, Error};
use clap::{App, Arg};
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::str::FromStr;

use crate::filter::{Filter, Mode};
use crate::filter::{FILTER, FILTER_HIGHLIGHT, HIGHLIGHT};
use crate::tokenizer::Tokenizer;

fn main() -> Result<(), Error> {
    let (mut input, mut output, mode, summary, expression) = parse_arguments().context("Invalid command arguments")?;
    let tokenizer = Tokenizer::new();
    let filter = Filter::new(&tokenizer, &expression, mode).context("Initializing filter failed")?;
    let lines = filter.filter(&mut input, &mut output).context("Filtering failed")?;
    if summary {
        println!();
        println!(
            "{} line(s) processed, {} line(s) matched",
            lines.processed, lines.matched
        );
    }
    Ok(())
}

fn parse_arguments() -> Result<(Box<dyn Read>, Box<dyn Write>, Mode, bool, String), Error> {
    // TODO parameters --separator with values "[:space:]", ",", ...
    let input_argument = "input";
    let output_argument = "output";
    let mode_argument = "mode";
    let summary_argument = "summary";
    let expression_argument = "expression";
    let semfilter_command = App::new("semfilter")
        .version("0.1")
        .about("semantic filter") // TODO description
        .arg(
            Arg::with_name(input_argument)
                .short("i")
                .long("input-file")
                .value_name("input-file")
                .help("Input file to read (stdin if not specified)"),
        )
        .arg(
            Arg::with_name(output_argument)
                .short("o")
                .long("output-file")
                .value_name("output-file")
                .help("Output file to write (stdout if not specified)"),
        )
        .arg(
            Arg::with_name(mode_argument)
                .short("m")
                .long("mode")
                .value_name("mode")
                .default_value(FILTER_HIGHLIGHT)
                .possible_values(&[FILTER, HIGHLIGHT, FILTER_HIGHLIGHT])
                .help("Filter mode"),
        )
        .arg(
            Arg::with_name(summary_argument)
                .short("s")
                .long("summary")
                .takes_value(false)
                .help("Print summary"),
        )
        .arg(
            Arg::with_name(expression_argument)
                .help("Filter expression")
                .required(true)
                .index(1),
        );

    let argument_matches = semfilter_command.get_matches();
    let input: Box<dyn Read> = match argument_matches.value_of(input_argument) {
        None => Box::new(stdin()),
        Some(input_file) => {
            Box::new(File::open(input_file).context(format!("Failed to open input-file '{}'", input_file))?)
        }
    };
    let output: Box<dyn Write> = match argument_matches.value_of(output_argument) {
        None => Box::new(stdout()),
        Some(output_file) => {
            Box::new(File::open(output_file).context(format!("Failed to open output-file '{}'", output_file))?)
        }
    };
    let mode = Mode::from_str(argument_matches.value_of(mode_argument).unwrap())?;
    let summary = argument_matches.is_present(summary_argument);
    let expression = String::from(argument_matches.value_of(expression_argument).unwrap());

    return Ok((input, output, mode, summary, expression));
}
