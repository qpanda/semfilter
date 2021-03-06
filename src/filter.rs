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

impl Mode {
    pub fn is_highlight(&self) -> bool {
        match self {
            Self::Filter => false,
            Self::Highlight(_) => true,
            Self::FilterHighlight(_) => true,
        }
    }
}

pub const DATE_FORMAT: &str = "%F";
pub const TIME_FORMAT: &str = "%T";
pub const DATE_TIME_FORMAT: &str = "%+";
pub const LOCAL_DATE_TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f";

pub struct Formats {
    pub date: String,
    pub time: String,
    pub date_time: String,
    pub local_date_time: String,
}

pub struct Settings {
    pub formats: Formats,
    pub mode: Mode,
    pub count: bool,
}

pub struct Filter<'a> {
    tokenizer: &'a Tokenizer,
    expression: &'a str,
    settings: &'a Settings,
}

pub struct Lines {
    pub processed: usize,
    pub matched: usize,
}

impl<'a> Filter<'a> {
    pub fn new(tokenizer: &'a Tokenizer, expression: &'a str, settings: &'a Settings) -> Result<Self, Error> {
        evaluate(expression, &vec![], &settings.formats).context(format!("Invalid expression '{}'", expression))?;

        Ok(Filter {
            tokenizer: tokenizer,
            expression: expression,
            settings: settings,
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
            let matches = evaluate(self.expression, &tokens, &self.settings.formats).context(format!(
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

        if self.settings.count {
            println!(
                "\n{} line(s) processed, {} line(s) matched",
                lines.processed, lines.matched
            );
        }

        Ok(lines)
    }

    fn output_line(&self, tokens: Vec<Token>, matches: &HashSet<Position>) -> Option<String> {
        match self.settings.mode {
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

    fn normal_text(&self, tokens: Vec<Token>) -> String {
        tokens.into_iter().map(|t| t.word.to_string()).collect()
    }

    fn highlighted_text(&self, tokens: Vec<Token>, matches: &HashSet<Position>, colour: Colour) -> String {
        tokens
            .into_iter()
            .map(|t| match matches.contains(&t.position) {
                true => colour.paint(t.word).to_string(),
                false => t.word.to_string(),
            })
            .collect()
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;

    pub fn default_formats() -> Formats {
        Formats {
            date: String::from(DATE_FORMAT),
            time: String::from(TIME_FORMAT),
            date_time: String::from(DATE_TIME_FORMAT),
            local_date_time: String::from(LOCAL_DATE_TIME_FORMAT),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::Separators;
    use file_diff::diff_files;
    use tempfile::NamedTempFile;

    #[test]
    fn empty() {
        // setup
        let input_file = NamedTempFile::new().unwrap();
        let mut input = input_file.reopen().unwrap();

        let output_file = NamedTempFile::new().unwrap();
        let mut output = output_file.reopen().unwrap();

        let separators = Separators::new(vec![" "]).unwrap();
        let tokenizer = Tokenizer::new(separators).unwrap();
        let expression = "$integer == 9";
        let settings = Settings {
            formats: test_utils::default_formats(),
            mode: Mode::Filter,
            count: false,
        };
        let filter = Filter::new(&tokenizer, expression, &settings).unwrap();

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

        let separators = Separators::new(vec![" "]).unwrap();
        let tokenizer = Tokenizer::new(separators).unwrap();
        let expression = "$id == ipsum";
        let settings = Settings {
            formats: test_utils::default_formats(),
            mode: Mode::Highlight(colour),
            count: false,
        };
        let filter = Filter::new(&tokenizer, expression, &settings).unwrap();

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

        let separators = Separators::new(vec![" "]).unwrap();
        let tokenizer = Tokenizer::new(separators).unwrap();
        let expression = "$id == abc";
        let settings = Settings {
            formats: test_utils::default_formats(),
            mode: Mode::Highlight(Colour::Red),
            count: false,
        };
        let filter = Filter::new(&tokenizer, expression, &settings).unwrap();

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

        let separators = Separators::new(vec![" "]).unwrap();
        let tokenizer = Tokenizer::new(separators).unwrap();
        let expression = "$id == ipsum";
        let settings = Settings {
            formats: test_utils::default_formats(),
            mode: Mode::Filter,
            count: false,
        };
        let filter = Filter::new(&tokenizer, expression, &settings).unwrap();

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

        let separators = Separators::new(vec![" "]).unwrap();
        let tokenizer = Tokenizer::new(separators).unwrap();
        let expression = "$id == abc";
        let settings = Settings {
            formats: test_utils::default_formats(),
            mode: Mode::Filter,
            count: false,
        };
        let filter = Filter::new(&tokenizer, expression, &settings).unwrap();

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

        let separators = Separators::new(vec![" "]).unwrap();
        let tokenizer = Tokenizer::new(separators).unwrap();
        let expression = "$id == ipsum";
        let settings = Settings {
            formats: test_utils::default_formats(),
            mode: Mode::FilterHighlight(colour),
            count: false,
        };
        let filter = Filter::new(&tokenizer, expression, &settings).unwrap();

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

        let separators = Separators::new(vec![" "]).unwrap();
        let tokenizer = Tokenizer::new(separators).unwrap();
        let expression = "$id == abc";
        let settings = Settings {
            formats: test_utils::default_formats(),
            mode: Mode::FilterHighlight(Colour::Red),
            count: false,
        };
        let filter = Filter::new(&tokenizer, expression, &settings).unwrap();

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
