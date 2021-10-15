pub struct Parser {
    separators: Vec<String>,
}



impl Parser {
    pub fn new() -> Parser {
        Parser {
            separators: vec![String::from(","), String::from(";"), String::from(" ")],
        }
    }

    pub fn parse(&self, input: String) -> Vec<String> {
        println!("Parser.parse invoked with {}", input);
        // TODO split_whitespaces, lines, split, split_inclusive, split_terminator
        Vec::new()
    }
}
