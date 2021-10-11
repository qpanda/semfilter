use std::io::{BufRead, BufReader, Error, LineWriter, Read, Write};

pub struct Filter<'a> {
    reader: BufReader<&'a mut dyn Read>,
    writer: LineWriter<&'a mut dyn Write>,
}

impl<'a> Filter<'a> {
    pub fn new(read: &'a mut dyn Read, write: &'a mut dyn Write) -> Self {
        Filter {
            reader: BufReader::new(read),
            writer: LineWriter::new(write),
        }
    }

    pub fn filter(&mut self) -> Result<(), Error> {
        let reader = &mut self.reader; // TODO explore get_mut()
        let writer = &mut self.writer; // TODO explore get_mut()
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
