semfilter
================
**semfilter** is a command line tool to perform ad-hoc analysis of semi-structured and unstructured text data. It works by matching tokens found on each input line against a specified expressions. It can be thought of as a data type aware version of `grep`.

## Syntax
Use the following syntax to run `semfilter` from your terminal window:

    # semfilter [FLAGS] [OPTIONS] <expression>

where `FLAGS`, `OPTIONS`, and `expression` are:
* `FLAGS`: Specifies optional flags, for example `-c` will ping processed and matched line counts
* `OPTIONS`: Specifies optional options, for example which input-file to read from
* `expression`: Specifies the filter expression which should be applied to each input line

To see a list of supported `FLAGS` and `OPTIONS` run `semfilter -h` from the terminal window. To see more detailed usage information including the syntax of `expression` run `semfilter --help` from the terminal window.

> **Note**: By default **semfilter** reads from stdin and writes to stdout.

## Examples
TODO
- [ ] document example usage (git tags | semfilter "version > x" https://github.com/rust-lang/rust
- [ ] document example usage (w | semfilter "date > x")

## Motivation
There are plenty of tools to parse and process data in well defined text based formats. There is [jq](https://stedolan.github.io/jq/) a flexible JSON processor, [yq](https://github.com/mikefarah/yq) a portable YAML processor, [xsv](https://github.com/BurntSushi/xsv) which can be used to analyze CSV files, and [XMLStarlet](https://en.wikipedia.org/wiki/XMLStarlet) which provides a set of command line utilities to process XML.

**semfilter** tries to fill a gap by provide a way to perform ad-hoc analysis of semi-structured and unstructured text output from other command line tools.

## License
**semfilter** is licensed under the MIT license.