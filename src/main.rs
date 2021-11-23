mod arguments;
mod expression;
mod filter;
mod parser;
mod tokenizer;

use anyhow::{Context, Error};

use crate::arguments::Arguments;
use crate::filter::{Filter, Lines};
use crate::tokenizer::Tokenizer;

fn main() -> Result<(), Error> {
    let mut arguments = Arguments::parse().context("Invalid command arguments")?;
    let tokenizer = Tokenizer::new();
    let filter =
        Filter::new(&tokenizer, &arguments.expression, arguments.mode).context("Initializing filter failed")?;
    let lines = filter
        .filter(&mut arguments.input, &mut arguments.output)
        .context("Filtering failed")?;
    print_summary(arguments.summary, lines);
    Ok(())
}

fn print_summary(summary: bool, lines: Lines) {
    if summary {
        println!();
        println!(
            "{} line(s) processed, {} line(s) matched",
            lines.processed, lines.matched
        );
    }
}
