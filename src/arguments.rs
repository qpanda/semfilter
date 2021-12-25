use crate::expression::Validator;
use crate::filter::{Formats, Mode, Settings};
use crate::filter::{DATE_FORMAT, DATE_TIME_FORMAT, LOCAL_DATE_TIME_FORMAT, TIME_FORMAT};
use crate::filter::{FILTER, FILTER_HIGHLIGHT, HIGHLIGHT};
use crate::tokenizer::Separators;
use crate::tokenizer::{SEPARATORS, WHITESPACES};
use anyhow::{Context, Error};
use chrono::format::{strftime::StrftimeItems, Item};
use clap::{App, Arg};
use std::collections::HashSet;
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::str::FromStr;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

const EXPRESSION_HELP: &str = r#"Filter expression applied to tokens found on each input line.

SYNTAX
The supported syntax of filter expressions is shown in BNF below:

<expression> ::= <conditions>
<conditions> ::= <condition> | "(" <conditions> ")" |
                 <conditions> <operator> <conditions>
<operator>   ::= " and " | " or "
<condition>  ::= <type> <comperator> <value>
<type>       ::= "$integer" | "$float" | "$id" | "$date" | "$time" |
                 "$dateTime" | "$localDateTime" | "$ipv4Address" |
                 "$ipv6Address" | "$ipv4SocketAddress" |
                 "$ipv6SocketAddress" | "$semanticVersion"
<comperator> ::= " == " | " != " | " > " | " >= " | " < " | " <= "

Note that the expected format of <value> in a <condition> depends on the
<type> being used.

VALUES
The values for $integer, $float, and $id must conform to the following Rust
patterns.

Type                 Syntax
-----------------------------------------------------------------------------
$integer             ['+'|'-']? ['0'..='9']+
$float               ['+'|'-']? ['0'..='9']* ['.']? ['0'..='9']*
$id                  ['a'..='z'|'A'..='Z'|'0'..='9'|'.'|':'|'_'|'-']+

The values for $date, $time, $dateTime, and $localDateTime must conform to the
default format strings or the format strings configured on the command line
and must be valid dates / times.

The values for $ipv4Address, $ipv6Address, $ipv4SocketAddress, and
$ipv6SocketAddress must be valid IP and socket addresses.

The values for $semanticVersion must be a valid semantic version string.

EXAMPLES
The following are examples of filter expressions with explanation:

"$date > 2021-01-01"
    Matches all lines that contain a date (in date format yyyy-mm-dd) after
    '2021-01-01'

"$semanticVersion >= 0.1.0 and $id == david"
    Matches all lines that contain a semantic version larger than or equal to
    '0.1.0' and the id 'david'

"$semanticVersion >= 0.1.0 and ($id == david or $id == sara)"
    Matches all lines that contain a semantic version larger than or equal to
    '0.1.0' and the id 'david' or the id 'sara'
"#;

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
        let add_separator_argument = "add-separator";
        let remove_separator_argument = "remove-separator";
        let mode_argument = "mode";
        let count_argument = "count";
        let date_format_argument = "date-format";
        let time_format_argument = "time-format";
        let date_time_format_argument = "date-time-format";
        let local_date_time_format_argument = "local-date-time-format";
        let expression_argument = "expression";

        let semfilter_command = App::new(NAME)
            .version(VERSION)
            .about("Filters semi-structured and unstructured text by matching tokens found on each input line against a specified expressions")
            .arg(
                Arg::with_name(input_argument)
                    .short("i")
                    .long("input-file")
                    .value_name("input-file")
                    .help("Input file to read (stdin if not specified)")
                    .next_line_help(true),
            )
            .arg(
                Arg::with_name(output_argument)
                    .short("o")
                    .long("output-file")
                    .value_name("output-file")
                    .help("Output file to write (stdout if not specified)")
                    .next_line_help(true),
            )
            .arg(
                Arg::with_name(add_separator_argument)
                    .short("a")
                    .long("add-separator")
                    .multiple(true)
                    .number_of_values(1)
                    .possible_values(&[&[WHITESPACES], SEPARATORS].concat())
                    .help("separator to add to default separators")
                    .next_line_help(true),
            )
            .arg(
                Arg::with_name(remove_separator_argument)
                    .short("r")
                    .long("remove-separator")
                    .multiple(true)
                    .number_of_values(1)
                    .possible_values(&[&[WHITESPACES], SEPARATORS].concat())
                    .help("separator to remove from default separators")
                    .next_line_help(true),
            )
            .arg(
                Arg::with_name(mode_argument)
                    .short("m")
                    .long("mode")
                    .value_name("mode")
                    .default_value(FILTER_HIGHLIGHT)
                    .possible_values(&[FILTER, HIGHLIGHT, FILTER_HIGHLIGHT])
                    .help("Filter mode")
                    .next_line_help(true),
            )
            .arg(
                Arg::with_name(count_argument)
                    .short("c")
                    .long("count")
                    .takes_value(false)
                    .help("Print processed and matched line count")
                    .next_line_help(true),
            )
            .arg(
                Arg::with_name(date_format_argument)
                    .long("date-format")
                    .value_name("date-format")
                    .default_value(DATE_FORMAT)
                    .validator(Arguments::validate_strftime)
                    .help("Date format using chrono::format::strftime specifiers (must not include separators)")
                    .next_line_help(true),
            )
            .arg(
                Arg::with_name(time_format_argument)
                    .long("time-format")
                    .value_name("time-format")
                    .default_value(TIME_FORMAT)
                    .validator(Arguments::validate_strftime)
                    .help("Time format using chrono::format::strftime specifiers (must not include separators)")
                    .next_line_help(true),
            )
            .arg(
                Arg::with_name(date_time_format_argument)
                    .long("date-time-format")
                    .value_name("date-time-format")
                    .default_value(DATE_TIME_FORMAT)
                    .validator(Arguments::validate_strftime)
                    .help("DateTime format using chrono::format::strftime specifiers (must not include separators)")
                    .next_line_help(true),
            )
            .arg(
                Arg::with_name(local_date_time_format_argument)
                    .long("local-date-time-format")
                    .value_name("local-date-time-format")
                    .default_value(LOCAL_DATE_TIME_FORMAT)
                    .validator(Arguments::validate_strftime)
                    .help("LocalDateTime format using chrono::format::strftime specifiers (must not include separators)")
                    .next_line_help(true),
            )
            .arg(
                Arg::with_name(expression_argument)
                    .help("Filter expression applied to tokens found on each input line")
                    .required(true)
                    .index(1)
                    .next_line_help(true)
                    .long_help(EXPRESSION_HELP),
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
        let count = argument_matches.is_present(count_argument);
        let expression = String::from(argument_matches.value_of(expression_argument).unwrap());
        let formats = Formats {
            date: String::from(argument_matches.value_of(date_format_argument).unwrap()),
            time: String::from(argument_matches.value_of(time_format_argument).unwrap()),
            date_time: String::from(argument_matches.value_of(date_time_format_argument).unwrap()),
            local_date_time: String::from(argument_matches.value_of(local_date_time_format_argument).unwrap()),
        };
        let add_separators = match argument_matches.values_of(add_separator_argument) {
            None => vec![],
            Some(add_separators) => add_separators.collect(),
        };
        let remove_separators = match argument_matches.values_of(remove_separator_argument) {
            None => vec![],
            Some(remove_separators) => remove_separators.collect(),
        };

        let separators = Separators::new(Arguments::separators(add_separators, remove_separators))?;
        Validator::validate_formats(&formats).context("Invalid chrono format strings")?;
        Validator::validate_separators(&expression, &separators, &formats).context("Invalid separators")?;

        Ok(Arguments {
            input: input,
            output: output,
            expression: expression,
            separators: separators,
            settings: Settings {
                formats: formats,
                mode: mode,
                count: count,
            },
        })
    }

    fn separators<'a>(add_separators: Vec<&'a str>, remove_separators: Vec<&'a str>) -> Vec<&'a str> {
        let mut default_separators = HashSet::from([
            " ", ",", ";", "|", "!", "\"", "#", "%", "&", "(", ")", "*", "<", "=", ">", "?", "@", "\\", "^", "{", "}",
        ]);
        default_separators.extend(add_separators);
        default_separators.retain(|separator| !remove_separators.contains(separator));
        return default_separators.into_iter().collect();
    }

    fn validate_strftime(format: String) -> Result<(), String> {
        match StrftimeItems::new(&format).position(|i| i == Item::Error) {
            None => Ok(()),
            Some(_) => Err(format!("Format string '{}' invalid", format)),
        }
    }
}
