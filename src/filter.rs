use ansi_term::Colour;
use anyhow::{anyhow, Context, Error};
use std::collections::HashSet;
use std::io::{BufRead, BufReader, LineWriter, Read, Write};
use std::str::FromStr;

use crate::expression::expression::evaluate;
use crate::tokenizer::Position;
use crate::tokenizer::Token;
use crate::tokenizer::Tokenizer;

pub const FILTER: &str = "filter";
pub const HIGHLIGHT: &str = "highlight";
pub const FILTER_HIGHLIGHT: &str = "filter-and-highlight";

#[derive(PartialEq)]
pub enum Mode {
    Filter,
    Highlight(Colour),
    FilterHighlight(Colour),
}

impl FromStr for Mode {
    type Err = Error;

    fn from_str(mode: &str) -> Result<Self, Error> {
        match mode {
            FILTER => Ok(Mode::Filter),
            HIGHLIGHT => Ok(Mode::Highlight(Colour::Red)),
            FILTER_HIGHLIGHT => Ok(Mode::FilterHighlight(Colour::Red)),
            _ => Err(anyhow!("invalid mode '{}'", mode)),
        }
    }
}

pub struct Filter<'a> {
    tokenizer: &'a Tokenizer,
    expression: &'a str,
    mode: Mode,
    summary: bool,
}

pub struct Lines {
    pub processed: usize,
    pub matched: usize,
}

impl<'a> Filter<'a> {
    pub fn new(tokenizer: &'a Tokenizer, expression: &'a str, mode: Mode, summary: bool) -> Result<Self, Error> {
        evaluate(expression, &vec![]).context(format!("Invalid filter expression '{}'", expression))?;

        Ok(Filter {
            tokenizer: tokenizer,
            expression: expression,
            mode: mode,
            summary: summary,
        })
    }

    pub fn filter(&self, read: &mut dyn Read, write: &mut dyn Write) -> Result<Lines, Error> {
        let mut lines = Lines {
            processed: 0,
            matched: 0,
        };

        let reader = BufReader::new(read);
        let mut writer = LineWriter::new(write);
        for input_line in reader.lines() {
            let input_line = input_line.context(format!("Unable to read line '{}' of input-file", lines.processed))?;
            let tokens = self.tokenizer.tokens(&input_line);
            let matches = evaluate(self.expression, &tokens).context(format!(
                "Evaluating expression '{}' for line '{}' of input-file failed",
                self.expression, lines.processed
            ))?;
            if let Some(output_line) = self.output_line(tokens, &matches) {
                writer
                    .write_all(output_line.as_bytes())
                    .context(format!("Unable to write to output-file"))?;
                writer
                    .write_all(b"\n")
                    .context(format!("Unable to write to output-file"))?;
            }

            if !matches.is_empty() {
                lines.matched += 1;
            }
            lines.processed += 1;
        }

        if self.summary {
            println!(
                "\n{} line(s) processed, {} line(s) matched",
                lines.processed, lines.matched
            );
        }

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
        let expression = "$integer == 9";
        let filter = Filter::new(&tokenizer, expression, Mode::Filter, false).unwrap();

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
        let expression = "$id == ipsum";
        let filter = Filter::new(&tokenizer, expression, Mode::Highlight(colour), false).unwrap();

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
        let expression = "$id == abc";
        let filter = Filter::new(&tokenizer, expression, Mode::Highlight(Colour::Red), false).unwrap();

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
        let expression = "$id == ipsum";
        let filter = Filter::new(&tokenizer, expression, Mode::Filter, false).unwrap();

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
        let expression = "$id == abc";
        let filter = Filter::new(&tokenizer, expression, Mode::Filter, false).unwrap();

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
        let expression = "$id == ipsum";
        let filter = Filter::new(&tokenizer, expression, Mode::FilterHighlight(colour), false).unwrap();

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
        let expression = "$id == abc";
        let filter = Filter::new(&tokenizer, expression, Mode::FilterHighlight(Colour::Red), false).unwrap();

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
