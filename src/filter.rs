use std::error::Error;
use std::io::{BufRead, BufReader, LineWriter, Read, Write};

use crate::tokenizer::Tokenizer;

pub struct Filter<'a> {
    tokenizer: &'a Tokenizer,
}

impl<'a> Filter<'a> {
    pub fn new(tokenizer: &'a Tokenizer) -> Self {
        Filter { tokenizer: tokenizer }
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

            let tokens = self.tokenizer.tokens(&line);
            let text: String = tokens.into_iter().map(|r| r.text).collect::<String>();

            writer.write_all(text.as_bytes())?;
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

        let tokenizer = Tokenizer::new();
        let filter = Filter::new(&tokenizer);

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

        let tokenizer = Tokenizer::new();
        let filter = Filter::new(&tokenizer);

        // exercise
        let lines = filter.filter(&mut input_file, &mut output_file).unwrap();

        // verify
        let mut input_file = input.reopen().unwrap();
        let mut output_file = output.reopen().unwrap();
        assert_eq!(1, lines);
        assert!(diff_files(&mut input_file, &mut output_file));
    }
}
