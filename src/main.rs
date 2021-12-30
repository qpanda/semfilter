mod ansi;
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
    let mut args = Arguments::parse().context("Invalid command arguments")?;
    let tokenizer = Tokenizer::new(args.separators).context("Initializing tokenizer failed")?;
    let filter = Filter::new(&tokenizer, &args.expression, &args.settings).context("Initializing filter failed")?;

    ansi::enable_ansi_support(&args.settings).context("Enabling ANSI support failed")?;

    filter
        .filter(&mut args.input, &mut args.output)
        .context("Filtering failed")?;

    Ok(())
}
