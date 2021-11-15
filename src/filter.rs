use std::error::Error;
use std::io::{BufRead, BufReader, LineWriter, Read, Write};

use crate::expression::expression;
use crate::tokenizer::Tokenizer;

pub struct Filter<'a> {
    tokenizer: &'a Tokenizer,
    expression: &'a str,
}

pub struct Lines {
    processed: usize,
    matched: usize,
}

impl<'a> Filter<'a> {
    pub fn new(tokenizer: &'a Tokenizer, expression: &'a str) -> Result<Self, Box<dyn Error>> {
        match expression::evaluate(expression, &vec![]) {
            Ok(_) => Ok(Filter {
                tokenizer: tokenizer,
                expression: expression,
            }),
            Err(error) => Err(error.into()),
        }
    }

    // TODO add expression parameter
    pub fn filter(&self, read: &mut dyn Read, write: &mut dyn Write) -> Result<Lines, Box<dyn Error>> {
        let mut lines = Lines {
            processed: 0,
            matched: 0,
        };

        let reader = BufReader::new(read);
        let mut writer = LineWriter::new(write);
        for line in reader.lines() {
            let line = line?;
            let tokens = self.tokenizer.tokens(&line);
            let positions = expression::evaluate(self.expression, &tokens)?;
            if positions.len() != 0 {
                let text: String = tokens.into_iter().map(|r| r.text).collect::<String>();
                writer.write_all(text.as_bytes())?;
                writer.write_all(b"\n")?;
                lines.matched += 1;
            }

            lines.processed += 1;
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

        let tokenizer = Tokenizer::new();
        let expression = "integer == 9";
        let filter = Filter::new(&tokenizer, expression).unwrap();

        // exercise
        let lines = filter.filter(&mut input_file, &mut output_file).unwrap();

        // verify
        let mut input_file = input.reopen().unwrap();
        let mut output_file = output.reopen().unwrap();
        assert_eq!(0, lines.processed);
        assert_eq!(0, lines.matched);
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

        let tokenizer = Tokenizer::new();
        let expression = "text == ipsum";
        let filter = Filter::new(&tokenizer, expression).unwrap();

        // exercise
        let lines = filter.filter(&mut input_file, &mut output_file).unwrap();

        // verify
        let mut input_file = input.reopen().unwrap();
        let mut output_file = output.reopen().unwrap();
        assert_eq!(1, lines.processed);
        assert_eq!(1, lines.matched);
        assert!(diff_files(&mut input_file, &mut output_file));
    }

    #[test]
    fn filter() {
        // setup
        let input = NamedTempFile::new().unwrap();
        let output = NamedTempFile::new().unwrap();
        let mut input_file = input.reopen().unwrap();
        let mut output_file = output.reopen().unwrap();
        writeln!(input_file, "lorem ipsum dolor sit amet consectetuer").unwrap();
        let mut input_file = input.reopen().unwrap();

        let tokenizer = Tokenizer::new();
        let expression = "text == abc";
        let filter = Filter::new(&tokenizer, expression).unwrap();

        // exercise
        let lines = filter.filter(&mut input_file, &mut output_file).unwrap();

        // verify
        let mut input_file = input.reopen().unwrap();
        let mut output_file = output.reopen().unwrap();
        assert_eq!(1, lines.processed);
        assert_eq!(0, lines.matched);
        assert!(!diff_files(&mut input_file, &mut output_file));
    }
}
