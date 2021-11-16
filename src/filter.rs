use std::collections::HashSet;
use std::error::Error;
use std::io::{BufRead, BufReader, LineWriter, Read, Write};

use crate::expression::expression;
use crate::tokenizer::Position;
use crate::tokenizer::Token;
use crate::tokenizer::Tokenizer;

#[derive(PartialEq)]
pub enum Mode {
    Highlight,
    Filter,
    FilterHighlight,
}

pub struct Filter<'a> {
    tokenizer: &'a Tokenizer,
    expression: &'a str,
    mode: Mode,
}

pub struct Lines {
    processed: usize,
    matched: usize,
}

impl<'a> Filter<'a> {
    pub fn new(tokenizer: &'a Tokenizer, expression: &'a str, mode: Mode) -> Result<Self, Box<dyn Error>> {
        match expression::evaluate(expression, &vec![]) {
            Ok(_) => Ok(Filter {
                tokenizer: tokenizer,
                expression: expression,
                mode: mode,
            }),
            Err(error) => Err(error.into()),
        }
    }

    pub fn filter(&self, read: &mut dyn Read, write: &mut dyn Write) -> Result<Lines, Box<dyn Error>> {
        let mut lines = Lines {
            processed: 0,
            matched: 0,
        };

        let reader = BufReader::new(read);
        let mut writer = LineWriter::new(write);
        for input_line in reader.lines() {
            let input_line = input_line?;
            let tokens = self.tokenizer.tokens(&input_line);
            let positions = expression::evaluate(self.expression, &tokens)?;
            if let Some(output_line) = self.output_line(tokens, &positions) {
                writer.write_all(output_line.as_bytes())?;
                writer.write_all(b"\n")?;
            }

            if !positions.is_empty() {
                lines.matched += 1;
            }
            lines.processed += 1;
        }

        writer.flush()?;

        Ok(lines)
    }

    // TODO consider returning Option<&str>
    fn output_line(&self, tokens: Vec<Token>, positions: &HashSet<Position>) -> Option<String> {
        match self.mode {
            Mode::Highlight => Some(self.highlighted_text(tokens, positions)),
            Mode::Filter => match positions.is_empty() {
                true => None,
                false => Some(self.normal_text(tokens)),
            },
            Mode::FilterHighlight => match positions.is_empty() {
                true => None,
                false => Some(self.highlighted_text(tokens, positions)),
            },
        }
    }

    // TODO consider returning Option<&str>
    fn normal_text(&self, tokens: Vec<Token>) -> String {
        tokens.into_iter().map(|t| t.text).collect::<String>()
    }

    // TODO consider returning Option<&str>
    fn highlighted_text(&self, tokens: Vec<Token>, positions: &HashSet<Position>) -> String {
        tokens
            .into_iter()
            .map(|t| match positions.contains(&t.position) {
                true => t.text, // TODO add color/highlight
                false => t.text,
            })
            .collect::<String>()
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
        let filter = Filter::new(&tokenizer, expression, Mode::Filter).unwrap();

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
    fn highlight_matched() {
        // setup
        let input = NamedTempFile::new().unwrap();
        let output = NamedTempFile::new().unwrap();
        let mut input_file = input.reopen().unwrap();
        let mut output_file = output.reopen().unwrap();
        writeln!(input_file, "lorem ipsum dolor sit amet consectetuer").unwrap();
        let mut input_file = input.reopen().unwrap();

        let tokenizer = Tokenizer::new();
        let expression = "text == ipsum";
        let filter = Filter::new(&tokenizer, expression, Mode::Highlight).unwrap();

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
    fn highlight_unmatched() {
        // setup
        let input = NamedTempFile::new().unwrap();
        let output = NamedTempFile::new().unwrap();
        let mut input_file = input.reopen().unwrap();
        let mut output_file = output.reopen().unwrap();
        writeln!(input_file, "lorem ipsum dolor sit amet consectetuer").unwrap();
        let mut input_file = input.reopen().unwrap();

        let tokenizer = Tokenizer::new();
        let expression = "text == abc";
        let filter = Filter::new(&tokenizer, expression, Mode::Highlight).unwrap();

        // exercise
        let lines = filter.filter(&mut input_file, &mut output_file).unwrap();

        // verify
        let mut input_file = input.reopen().unwrap();
        let mut output_file = output.reopen().unwrap();
        assert_eq!(1, lines.processed);
        assert_eq!(0, lines.matched);
        assert!(diff_files(&mut input_file, &mut output_file));
    }

    #[test]
    fn filter_matched() {
        // setup
        let input = NamedTempFile::new().unwrap();
        let output = NamedTempFile::new().unwrap();
        let mut input_file = input.reopen().unwrap();
        let mut output_file = output.reopen().unwrap();
        writeln!(input_file, "lorem ipsum dolor sit amet consectetuer").unwrap();
        let mut input_file = input.reopen().unwrap();

        let tokenizer = Tokenizer::new();
        let expression = "text == ipsum";
        let filter = Filter::new(&tokenizer, expression, Mode::Filter).unwrap();

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
    fn filter_unmatched() {
        // setup
        let input = NamedTempFile::new().unwrap();
        let output = NamedTempFile::new().unwrap();
        let mut input_file = input.reopen().unwrap();
        let mut output_file = output.reopen().unwrap();
        writeln!(input_file, "lorem ipsum dolor sit amet consectetuer").unwrap();
        let mut input_file = input.reopen().unwrap();

        let tokenizer = Tokenizer::new();
        let expression = "text == abc";
        let filter = Filter::new(&tokenizer, expression, Mode::Filter).unwrap();

        // exercise
        let lines = filter.filter(&mut input_file, &mut output_file).unwrap();

        // verify
        let mut input_file = input.reopen().unwrap();
        let mut output_file = output.reopen().unwrap();
        assert_eq!(1, lines.processed);
        assert_eq!(0, lines.matched);
        assert!(!diff_files(&mut input_file, &mut output_file));
    }

    #[test]
    fn filter_highlight_matched() {
        // setup
        let input = NamedTempFile::new().unwrap();
        let output = NamedTempFile::new().unwrap();
        let mut input_file = input.reopen().unwrap();
        let mut output_file = output.reopen().unwrap();
        writeln!(input_file, "lorem ipsum dolor sit amet consectetuer").unwrap();
        let mut input_file = input.reopen().unwrap();

        let tokenizer = Tokenizer::new();
        let expression = "text == ipsum";
        let filter = Filter::new(&tokenizer, expression, Mode::FilterHighlight).unwrap();

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
    fn filter_highlight_unmatched() {
        // setup
        let input = NamedTempFile::new().unwrap();
        let output = NamedTempFile::new().unwrap();
        let mut input_file = input.reopen().unwrap();
        let mut output_file = output.reopen().unwrap();
        writeln!(input_file, "lorem ipsum dolor sit amet consectetuer").unwrap();
        let mut input_file = input.reopen().unwrap();

        let tokenizer = Tokenizer::new();
        let expression = "text == abc";
        let filter = Filter::new(&tokenizer, expression, Mode::FilterHighlight).unwrap();

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
