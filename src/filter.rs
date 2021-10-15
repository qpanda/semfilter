use std::io::{BufRead, BufReader, Error, LineWriter, Read, Write};

pub struct Filter {}

impl Filter {
    pub fn new() -> Self {
        Filter {}
    }

    // TODO should we return number of lines processed instead of nothing?
    pub fn filter(&self, read: &mut dyn Read, write: &mut dyn Write) -> Result<(), Error> {
        let reader = BufReader::new(read);
        let mut writer = LineWriter::new(write);
        for line in reader.lines() {
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
            writer.write_all(line?.as_bytes())?;
            writer.write_all(b"\n")?;
        }

        writer.flush()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_diff::diff_files;
    use tempfile::tempfile;

    #[test]
    fn empty() {
        // setup
        let mut input = tempfile().unwrap();
        let mut output = tempfile().unwrap();
        let filter = Filter::new();

        // exercise
        filter.filter(&mut input, &mut output).unwrap();

        // verify
        assert!(diff_files(&mut input, &mut output));
    }

    #[test]
    fn pass_through() {
        // setup
        let mut input = tempfile().unwrap();
        writeln!(input, "lorem ipsum dolor sit amet consectetuer").unwrap();

        let mut output = tempfile().unwrap();

        let filter = Filter::new();

        // exercise
        filter.filter(&mut input, &mut output).unwrap();

        // verify
        assert!(diff_files(&mut input, &mut output));
    }
}
