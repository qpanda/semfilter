use crate::filter::DATE_FORMAT;
use crate::filter::{Formats, Mode, Settings};
use crate::filter::{FILTER, FILTER_HIGHLIGHT, HIGHLIGHT};
use crate::tokenizer::Separators;
use crate::tokenizer::{COMMA, PIPE, SEMICOLON, SPACE, WHITESPACE};
use anyhow::{anyhow, Context, Error};
use chrono::format::{strftime::StrftimeItems, Item};
use clap::{App, Arg};
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::str::FromStr;

pub struct Arguments {
    pub input: Box<dyn Read>,
    pub output: Box<dyn Write>,
    pub expression: String,
    pub separators: Separators,
    pub settings: Settings,
}

impl Arguments {
    pub fn parse() -> Result<Self, Error> {
        let input_argument = "input";
        let output_argument = "output";
        let separator_argument = "separator";
        let mode_argument = "mode";
        let count_argument = "count";
        let date_format = "date-format";
        let expression_argument = "expression";
        let semfilter_command = App::new("semfilter")
            .version("0.1")
            .about("semfilter filters unstructured text by matching tokens found on each input lines against an expression")
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
                Arg::with_name(separator_argument)
                    .short("s")
                    .long("separator")
                    .multiple(true)
                    .number_of_values(1)
                    .default_value(WHITESPACE)
                    .possible_values(&[SPACE, COMMA, SEMICOLON, PIPE, WHITESPACE])
                    .help("separator(s) used to split input line into tokens"),
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
                Arg::with_name(count_argument)
                    .short("c")
                    .long("count")
                    .takes_value(false)
                    .help("Print processed and matched line count"),
            )
            .arg(
                Arg::with_name(date_format)
                    .long("date-format")
                    .value_name("date-format")
                    .default_value(DATE_FORMAT)
                    .validator(Arguments::validate_strftime)
                    .help("Date format using chrono::format::strftime specifiers (must not include separators)"),
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
        let separators = Separators::new(argument_matches.values_of(separator_argument).unwrap().collect())?;
        let mode = Mode::from_str(argument_matches.value_of(mode_argument).unwrap())?;
        let count = argument_matches.is_present(count_argument);
        let date_format = argument_matches.value_of(date_format).unwrap();
        let expression = String::from(argument_matches.value_of(expression_argument).unwrap());

        if separators.comprise_any(date_format.chars()) {
            return Err(anyhow!("Date format '{}' must not contain separators", date_format));
        }

        Ok(Arguments {
            input: input,
            output: output,
            expression: expression,
            separators: separators,
            settings: Settings {
                formats: Formats {
                    date: String::from(date_format),
                },
                mode: mode,
                count: count,
            },
        })
    }

    fn validate_strftime(v: String) -> Result<(), String> {
        match StrftimeItems::new(&v).position(|i| i == Item::Error) {
            None => Ok(()),
            Some(_) => Err(format!("Format string '{}' invalid", v)),
        }
    }
}
