mod filter;
mod parser;

use std::fs::File;
use std::io::stdout;

use crate::filter::Filter;
use crate::parser::Parser;

// TODO parameters --separator with values "[:space:]", ",", ...

fn main() {
    // let mut input = io::stdin();
    let mut input = File::open("test.txt").unwrap(); // TODO error handling
    let mut output = stdout();

    let classes = Vec::new();
    let parser = Parser::new(classes);
    let filter = Filter::new(&parser);
    filter.filter(&mut input, &mut output).unwrap(); // TODO error handling
}
