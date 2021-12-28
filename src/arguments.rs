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

const EXPRESSION_HELP: &str = r#"Filter expression applied to tokens found on each input line

SYNTAX
The expression can be a single <condition> or multiple <condition>s combined
with <operator>s. In complex expressions parenthesis can be used to group
<condition>s. Each <condition> compares a <type> with a <value> using a
<comparator>.

The supported <operator>s, <comparator>s, and <type>s and how an <expression>
is constructed using <condition>s is in BNF below.

<expression>         ::=  <conditions>
<conditions>         ::=  <condition> |
                          <conditions> <operator> <conditions> |
                          ( <conditions> )
<operator>           ::=  and | or
<condition>          ::=  <type> <comperator> <value> |
                          $id <string-comperator> <id> |
                          $ipAddress <set-comperator> <ip-network> |
                          $ipv4Address <set-comperator> <ipv4-network> |
                          $ipv6Address <set-comperator> <ipv6-network>
<comperator>         ::=  == | != | > | >= | < | <=
<string-comperator>  ::=  contains | starts-with | ends-with
<set-comperator>     ::=  in | not-in
<type>               ::=  $integer | $float | $id | $date | $time |
                          $dateTime | $localDateTime | $ipAddress |
                          $ipv4Address | $ipv6Address | $ipSocketAddress |
                          $ipv4SocketAddress | $ipv6SocketAddress |
                          $ipNetwork | $ipv4Network | $ipv6Network |
                          $semanticVersion

The expected format of <value> in a <condition> depends on the <type> being
used:
 * $integer must be compared with a valid integer value
 * $float must be compared with a valid float value (scientific notation,
   infinity, negative infinity, and not-a-number are not supported)
 * $id must be compared with a valid id which is a alphanumeric string which
   may contain cotaining special characters '.:_-'
 * $date, $time, $dateTime, and $localDateTime must be compared with a value
   that conforms to the format string
 * $ipAddress, $ipv4Address, $ipv6Address, $ipSocketAddress,
   $ipv4SocketAddress, $ipv6SocketAddress, $ipNetwork, $ipv4Network,
   $ipv6Network must be compared with a valid IP or socket address
 * $semanticVersion must be compared with a string representing a valid
   semantic version

EXAMPLES
'$semanticVersion >= 0.2.0'
   Match all lines containing a semantic version value greater than or equal
   to 0.2.0

'$id == qpanda and $time > 21:00:00'
   Match all lines containing an id value equal to 'qpanda' and a time value
   greater than 21:00:00


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
                Arg::with_name(count_argument)
                    .short("c")
                    .long("count")
                    .takes_value(false)
                    .help("Print processed and matched line count"),
            )
            .arg(
                Arg::with_name(input_argument)
                    .short("i")
                    .long("input-file")
                    .value_name("input-file")
                    .help("Input file to read (stdin if not specified)")
                    .display_order(1)
                    .next_line_help(true),
            )
            .arg(
                Arg::with_name(output_argument)
                    .short("o")
                    .long("output-file")
                    .value_name("output-file")
                    .help("Output file to write (stdout if not specified)")
                    .display_order(2)
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
                    .display_order(3)
                    .next_line_help(true),
            )
            .arg(
                Arg::with_name(add_separator_argument)
                    .short("a")
                    .long("add-separator")
                    .multiple(true)
                    .number_of_values(1)
                    .possible_values(&[&[WHITESPACES], SEPARATORS].concat())
                    .help("Separator(s) to add to default separators")
                    .long_help("Separator(s) to add to default separators [:space:],;|'\"()<=>{} used to split each input line into tokens.\n")
                    .display_order(4)
                    .next_line_help(true),
            )
            .arg(
                Arg::with_name(remove_separator_argument)
                    .short("r")
                    .long("remove-separator")
                    .multiple(true)
                    .number_of_values(1)
                    .possible_values(&[&[WHITESPACES], SEPARATORS].concat())
                    .help("Separator(s) to remove from default separators")
                    .long_help("Separator(s) to remove from default separators [:space:],;|'\"()<=>{} used to split each input line into tokens.\n")
                    .display_order(5)
                    .next_line_help(true),
            )
            .arg(
                Arg::with_name(date_format_argument)
                    .long("date-format")
                    .value_name("date-format")
                    .default_value(DATE_FORMAT)
                    .validator(Arguments::validate_strftime)
                    .help("$date format using chrono::format::strftime specifiers (must not include separators)")
                    .display_order(6)
                    .next_line_help(true),
            )
            .arg(
                Arg::with_name(time_format_argument)
                    .long("time-format")
                    .value_name("time-format")
                    .default_value(TIME_FORMAT)
                    .validator(Arguments::validate_strftime)
                    .help("$time format using chrono::format::strftime specifiers (must not include separators)")
                    .display_order(7)
                    .next_line_help(true),
            )
            .arg(
                Arg::with_name(date_time_format_argument)
                    .long("date-time-format")
                    .value_name("date-time-format")
                    .default_value(DATE_TIME_FORMAT)
                    .validator(Arguments::validate_strftime)
                    .help("$dateTime format using chrono::format::strftime specifiers (must not include separators)")
                    .display_order(8)
                    .next_line_help(true),
            )
            .arg(
                Arg::with_name(local_date_time_format_argument)
                    .long("local-date-time-format")
                    .value_name("local-date-time-format")
                    .default_value(LOCAL_DATE_TIME_FORMAT)
                    .validator(Arguments::validate_strftime)
                    .help("$localDateTime format using chrono::format::strftime specifiers (must not include separators)")
                    .display_order(9)
                    .next_line_help(true),
            )
            .arg(
                Arg::with_name(expression_argument)
                    .help("Filter expression applied to tokens found on each input line")
                    .required(true)
                    .index(1)
                    .next_line_help(true)
                    .long_help(EXPRESSION_HELP)
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
        let mut separators = HashSet::from(["[:space:]", ",", ";", "|", "'", "\"", "(", ")", "<", "=", ">", "{", "}"]);
        separators.extend(add_separators);
        separators.retain(|separator| !remove_separators.contains(separator));
        return separators.into_iter().collect();
    }

    fn validate_strftime(format: String) -> Result<(), String> {
        match StrftimeItems::new(&format).position(|i| i == Item::Error) {
            None => Ok(()),
            Some(_) => Err(format!("Format string '{}' invalid", format)),
        }
    }
}
