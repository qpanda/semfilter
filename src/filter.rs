use ansi_term::Colour;
use std::collections::HashSet;
use std::error::Error;
use std::io::{BufRead, BufReader, LineWriter, Read, Write};

use crate::expression::expression;
use crate::tokenizer::Position;
use crate::tokenizer::Token;
use crate::tokenizer::Tokenizer;

#[derive(PartialEq)]
pub enum Mode {
    Filter,
    Highlight(Colour),
    FilterHighlight(Colour),
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
            let matches = expression::evaluate(self.expression, &tokens)?;
            if let Some(output_line) = self.output_line(tokens, &matches) {
                writer.write_all(output_line.as_bytes())?;
                writer.write_all(b"\n")?;
            }

            if !matches.is_empty() {
                lines.matched += 1;
            }
            lines.processed += 1;
        }

        writer.flush()?;

        Ok(lines)
    }

    // TODO consider returning Option<&str>
    fn output_line(&self, tokens: Vec<Token>, matches: &HashSet<Position>) -> Option<String> {
        match self.mode {
            Mode::Filter => match matches.is_empty() {
                true => None,
                false => Some(self.normal_text(tokens)),
            },
            Mode::Highlight(colour) => Some(self.highlighted_text(tokens, matches, colour)),
            Mode::FilterHighlight(colour) => match matches.is_empty() {
                true => None,
                false => Some(self.highlighted_text(tokens, matches, colour)),
            },
        }
    }

    // TODO consider returning Option<&str>
    fn normal_text(&self, tokens: Vec<Token>) -> String {
        tokens.into_iter().map(|t| t.text).collect::<String>()
    }

    // TODO consider returning Option<&str>
    fn highlighted_text(&self, tokens: Vec<Token>, matches: &HashSet<Position>, colour: Colour) -> String {
        tokens
            .into_iter()
            .map(|t| match matches.contains(&t.position) {
                true => colour.paint(t.text).to_string(),
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
        let input_file = NamedTempFile::new().unwrap();
        let mut input = input_file.reopen().unwrap();

        let output_file = NamedTempFile::new().unwrap();
        let mut output = output_file.reopen().unwrap();

        let tokenizer = Tokenizer::new();
        let expression = "integer == 9";
        let filter = Filter::new(&tokenizer, expression, Mode::Filter).unwrap();

        // exercise
        let lines = filter.filter(&mut input, &mut output).unwrap();

        // verify
        let mut input = input_file.reopen().unwrap();
        let mut output = output_file.reopen().unwrap();
        assert_eq!(0, lines.processed);
        assert_eq!(0, lines.matched);
        assert!(diff_files(&mut input, &mut output));
    }

    #[test]
    fn highlight_matched() {
        // setup
        let colour = Colour::Red;

        let input_text = "lorem ipsum dolor sit amet consectetuer";
        let mut input_file = NamedTempFile::new().unwrap();
        writeln!(input_file, "{}", input_text).unwrap();
        let mut input = input_file.reopen().unwrap();

        let output_file = NamedTempFile::new().unwrap();
        let mut output = output_file.reopen().unwrap();

        let expected_text = format!("lorem {} dolor sit amet consectetuer", colour.paint("ipsum"));
        let mut expected_file = NamedTempFile::new().unwrap();
        writeln!(expected_file, "{}", expected_text).unwrap();

        let tokenizer = Tokenizer::new();
        let expression = "text == ipsum";
        let filter = Filter::new(&tokenizer, expression, Mode::Highlight(colour)).unwrap();

        // exercise
        let lines = filter.filter(&mut input, &mut output).unwrap();

        // verify
        let mut expected = expected_file.reopen().unwrap();
        let mut output = output_file.reopen().unwrap();
        assert_eq!(1, lines.processed);
        assert_eq!(1, lines.matched);
        assert!(diff_files(&mut expected, &mut output));
    }

    #[test]
    fn highlight_unmatched() {
        // setup
        let input_text = "lorem ipsum dolor sit amet consectetuer";
        let mut input_file = NamedTempFile::new().unwrap();
        writeln!(input_file, "{}", input_text).unwrap();
        let mut input = input_file.reopen().unwrap();

        let output_file = NamedTempFile::new().unwrap();
        let mut output = output_file.reopen().unwrap();

        let tokenizer = Tokenizer::new();
        let expression = "text == abc";
        let filter = Filter::new(&tokenizer, expression, Mode::Highlight(Colour::Red)).unwrap();

        // exercise
        let lines = filter.filter(&mut input, &mut output).unwrap();

        // verify
        let mut input = input_file.reopen().unwrap();
        let mut output = output_file.reopen().unwrap();
        assert_eq!(1, lines.processed);
        assert_eq!(0, lines.matched);
        assert!(diff_files(&mut input, &mut output));
    }

    #[test]
    fn filter_matched() {
        // setup
        let input_text = "lorem ipsum dolor sit amet consectetuer";
        let mut input_file = NamedTempFile::new().unwrap();
        writeln!(input_file, "{}", input_text).unwrap();
        let mut input = input_file.reopen().unwrap();

        let output_file = NamedTempFile::new().unwrap();
        let mut output = output_file.reopen().unwrap();

        let tokenizer = Tokenizer::new();
        let expression = "text == ipsum";
        let filter = Filter::new(&tokenizer, expression, Mode::Filter).unwrap();

        // exercise
        let lines = filter.filter(&mut input, &mut output).unwrap();

        // verify
        let mut input = input_file.reopen().unwrap();
        let mut output = output_file.reopen().unwrap();
        assert_eq!(1, lines.processed);
        assert_eq!(1, lines.matched);
        assert!(diff_files(&mut input, &mut output));
    }

    #[test]
    fn filter_unmatched() {
        // setup
        let input_text = "lorem ipsum dolor sit amet consectetuer";
        let mut input_file = NamedTempFile::new().unwrap();
        writeln!(input_file, "{}", input_text).unwrap();
        let mut input = input_file.reopen().unwrap();

        let output_file = NamedTempFile::new().unwrap();
        let mut output = output_file.reopen().unwrap();

        let tokenizer = Tokenizer::new();
        let expression = "text == abc";
        let filter = Filter::new(&tokenizer, expression, Mode::Filter).unwrap();

        // exercise
        let lines = filter.filter(&mut input, &mut output).unwrap();

        // verify
        let mut input = input_file.reopen().unwrap();
        let mut output = output_file.reopen().unwrap();
        assert_eq!(1, lines.processed);
        assert_eq!(0, lines.matched);
        assert!(!diff_files(&mut input, &mut output));
    }

    #[test]
    fn filter_highlight_matched() {
        // setup
        let colour = Colour::Red;

        let input_text = "lorem ipsum dolor sit amet consectetuer";
        let mut input_file = NamedTempFile::new().unwrap();
        writeln!(input_file, "{}", input_text).unwrap();
        let mut input = input_file.reopen().unwrap();

        let output_file = NamedTempFile::new().unwrap();
        let mut output = output_file.reopen().unwrap();

        let expected_text = format!("lorem {} dolor sit amet consectetuer", colour.paint("ipsum"));
        let mut expected_file = NamedTempFile::new().unwrap();
        writeln!(expected_file, "{}", expected_text).unwrap();

        let tokenizer = Tokenizer::new();
        let expression = "text == ipsum";
        let filter = Filter::new(&tokenizer, expression, Mode::FilterHighlight(colour)).unwrap();

        // exercise
        let lines = filter.filter(&mut input, &mut output).unwrap();

        // verify
        let mut expected = expected_file.reopen().unwrap();
        let mut output = output_file.reopen().unwrap();
        assert_eq!(1, lines.processed);
        assert_eq!(1, lines.matched);
        assert!(diff_files(&mut expected, &mut output));
    }

    #[test]
    fn filter_highlight_unmatched() {
        // setup
        let input_text = "lorem ipsum dolor sit amet consectetuer";
        let mut input_file = NamedTempFile::new().unwrap();
        writeln!(input_file, "{}", input_text).unwrap();
        let mut input = input_file.reopen().unwrap();

        let output_file = NamedTempFile::new().unwrap();
        let mut output = output_file.reopen().unwrap();

        let tokenizer = Tokenizer::new();
        let expression = "text == abc";
        let filter = Filter::new(&tokenizer, expression, Mode::FilterHighlight(Colour::Red)).unwrap();

        // exercise
        let lines = filter.filter(&mut input, &mut output).unwrap();

        // verify
        let mut input = input_file.reopen().unwrap();
        let mut output = output_file.reopen().unwrap();
        assert_eq!(1, lines.processed);
        assert_eq!(0, lines.matched);
        assert!(!diff_files(&mut input, &mut output));
    }
}
