mod arguments;
mod expression;
mod filter;
mod parser;
mod tokenizer;

use anyhow::{anyhow, Context, Error};

use crate::arguments::Arguments;
use crate::filter::Filter;
use crate::filter::Settings;
use crate::tokenizer::Tokenizer;

fn main() -> Result<(), Error> {
    let mut args = Arguments::parse().context("Invalid command arguments")?;
    let tokenizer = Tokenizer::new(args.separators).context("Initializing tokenizer failed")?;
    let filter = Filter::new(&tokenizer, &args.expression, &args.settings).context("Initializing filter failed")?;

    enable_ansi_support(&args.settings).context("Enabling ANSI support failed")?;

    filter
        .filter(&mut args.input, &mut args.output)
        .context("Filtering failed")?;

    Ok(())
}

fn enable_ansi_support(settings: &Settings) -> Result<(), Error> {
    if cfg!(windows) {
        if settings.mode.is_highlight() {
            match ansi_term::enable_ansi_support() {
                Ok(()) => return Ok(()),
                Err(error) => return Err(anyhow!("enabling failed with error code {}", error)),
            }
        }
    }

    Ok(())
}
