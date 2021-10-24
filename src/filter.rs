use std::error::Error;
use std::io::{BufRead, BufReader, LineWriter, Read, Write};

use crate::parser::Parser;
use crate::parser::Token;
use crate::parser::Value;

pub struct Filter<'a> {
    parser: &'a Parser,
}

impl<'a> Filter<'a> {
    pub fn new(parser: &'a Parser) -> Self {
        Filter { parser: parser }
    }

    pub fn filter(&self, read: &mut dyn Read, write: &mut dyn Write) -> Result<usize, Box<dyn Error>> {
        let reader = BufReader::new(read);
        let mut writer = LineWriter::new(write);
        let mut lines = 0;
        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(error) => return Err(error.into()), // TODO add flag to allow continue on error or fail
            };

            let results = self.parser.parse(&line);
            let t: String = results
                .into_iter()
                .map(|r: (Token, Value)| r.0.text)
                .collect::<String>();

            // let x = tokens.into_iter().collect();
            // TODO 1. parse line
            // TODO 2. check match
            // TODO 3. output matching line
            // OPTION 1
            // tokens = parse(line)
            // if match(tokens) {
            //   print(tokens)
            // }
            // OPTION 2
            // parsed_line = parse(line)
            // if parsed_line.match() {
            //   print(parsed_line)
            // }

            writer.write_all(t.as_bytes())?;
            writer.write_all(b"\n")?;
            lines += 1;
        }

        writer.flush()?;

        Ok(lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_diff::diff_files;
    use tempfile::NamedTempFile;

    #[test]
    fn empty() {
        // setup
        let input = NamedTempFile::new().unwrap();
        let output = NamedTempFile::new().unwrap();
        let mut input_file = input.reopen().unwrap();
        let mut output_file = output.reopen().unwrap();

        let parser = Parser::new(Vec::new());
        let filter = Filter::new(&parser);

        // exercise
        let lines = filter.filter(&mut input_file, &mut output_file).unwrap();

        // verify
        let mut input_file = input.reopen().unwrap();
        let mut output_file = output.reopen().unwrap();
        assert_eq!(0, lines);
        assert!(diff_files(&mut input_file, &mut output_file));
    }

    #[test]
    fn pass_through() {
        // setup
        let input = NamedTempFile::new().unwrap();
        let output = NamedTempFile::new().unwrap();
        let mut input_file = input.reopen().unwrap();
        let mut output_file = output.reopen().unwrap();
        writeln!(input_file, "lorem ipsum dolor sit amet consectetuer").unwrap();
        let mut input_file = input.reopen().unwrap();

        let parser = Parser::new(Vec::new());
        let filter = Filter::new(&parser);

        // exercise
        let lines = filter.filter(&mut input_file, &mut output_file).unwrap();

        // verify
        let mut input_file = input.reopen().unwrap();
        let mut output_file = output.reopen().unwrap();
        assert_eq!(1, lines);
        assert!(diff_files(&mut input_file, &mut output_file));
    }
}
