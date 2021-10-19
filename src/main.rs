mod filter;
mod parser;

use std::fs::File;
use std::io::stdout;

fn main() {
    // let mut input = io::stdin();
    let mut input = File::open("test.txt").unwrap(); // TODO error handling
    let mut output = stdout();
    let filter = filter::Filter::new();
    filter.filter(&mut input, &mut output).unwrap(); // TODO error handling
}
