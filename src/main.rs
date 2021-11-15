mod expression;
mod filter;
mod parser;
mod tokenizer;

use std::fs::File;
use std::io::stdout;

use crate::filter::Filter;
use crate::tokenizer::Tokenizer;

// TODO parameters --separator with values "[:space:]", ",", ...

fn main() {
    // let mut input = io::stdin();
    let mut input = File::open("test.txt").unwrap(); // TODO error handling
    let mut output = stdout();

    let tokenizer = Tokenizer::new();
    let expression = "integer == 9";
    let filter = Filter::new(&tokenizer, expression).unwrap(); // TODO error handling
    filter.filter(&mut input, &mut output).unwrap(); // TODO error handling
}
