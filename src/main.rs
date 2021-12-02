mod arguments;
mod expression;
mod filter;
mod parser;
mod tokenizer;

use anyhow::{Context, Error};

use crate::arguments::Arguments;
use crate::filter::Filter;
use crate::tokenizer::Tokenizer;

fn main() -> Result<(), Error> {
    let mut arguments = Arguments::parse().context("Invalid command arguments")?;
    let tokenizer = Tokenizer::new();
    let filter = Filter::new(&tokenizer, &arguments.expression, arguments.mode, arguments.count)
        .context("Initializing filter failed")?;
    filter
        .filter(&mut arguments.input, &mut arguments.output)
        .context("Filtering failed")?;
    Ok(())
}
