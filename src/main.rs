mod filter;

use std::fs::File;
use std::io::stdout;

fn main() {
    // let mut input = io::stdin();
    let mut input = File::open("test.txt").unwrap(); // TODO error handling
    let mut output = stdout();
    let mut filter = filter::Filter::new(&mut input, &mut output);
    filter.filter().unwrap(); // TODO error handling
}
