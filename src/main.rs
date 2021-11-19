mod expression;
mod filter;
mod parser;
mod tokenizer;

use ansi_term::Colour;
use clap::{App, Arg};
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};

use crate::filter::Filter;
use crate::filter::Mode;
use crate::tokenizer::Tokenizer;

fn main() {
    let (mut input, mut output, mode, expression) = args();
    let tokenizer = Tokenizer::new();
    let filter = Filter::new(&tokenizer, &expression, mode).unwrap(); // TODO error handling
    let result = filter.filter(&mut input, &mut output).unwrap(); // TODO error handling, TODO handle result
}

fn args() -> (Box<dyn Read>, Box<dyn Write>, Mode, String) {
    // TODO parameters --separator with values "[:space:]", ",", ...
    let input_option = "input";
    let output_option = "output";
    let mode_option = "mode";
    let expression_argument = "expression";
    let semfilter = App::new("semfilter")
        .version("0.1")
        .about("semantic filter") // TODO description
        .arg(
            Arg::with_name(input_option)
                .short("i")
                .long("input-file")
                .value_name("input-file")
                .help("Input file to read (stdin if not specified)"),
        )
        .arg(
            Arg::with_name(output_option)
                .short("o")
                .long("output-file")
                .value_name("output-file")
                .help("Output file to wrote (stdout if not specified)"),
        )
        .arg(
            Arg::with_name(mode_option)
                .short("m")
                .long("mode")
                .value_name("mode")
                .default_value("filter-and-highlight")
                .possible_values(&["filter", "highlight", "filter-and-highlight"])
                .help("Filter mode"),
        )
        .arg(
            Arg::with_name(expression_argument)
                .help("Filter expression")
                .required(true)
                .index(1),
        );

    let args = semfilter.get_matches();
    let input: Box<dyn Read> = match args.value_of(input_option) {
        None => Box::new(stdin()),
        Some(input_file) => Box::new(File::open(input_file).unwrap()), // TODO error handling
    };
    let output: Box<dyn Write> = match args.value_of(output_option) {
        None => Box::new(stdout()),
        Some(output_file) => Box::new(File::open(output_file).unwrap()), // TODO error handling
    };
    let mode = Mode::FilterHighlight(Colour::Red);
    // TODO mode
    // let mode = match args.value_of(mode_arg) {
    //     None => Mode::FilterHighlight(Colour::Red),
    //     Some(mode) => Mode::from(mode),
    // };
    let expression = String::from(args.value_of(expression_argument).unwrap());

    return (input, output, mode, expression);
}
