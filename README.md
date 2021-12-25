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

For an overview of supported `FLAGS` and `OPTIONS` run `semfilter -h` from the terminal window.

For detailed information on `FLAGS` and `OPTIONS` and the syntax of `expression` run `semfilter --help` from the terminal window.

> **Note**: By default **semfilter** reads from **stdin** and writes to **stdout**.

## Examples
**Filtering Git tags**
```shell
$ git tag
0.1.0
0.2.0
0.3.0
0.4.0
$ git tag | semfilter '$semanticVersion >= 0.2.0'
0.2.0
0.3.0
0.4.0
$
```

**Filtering w output**
```shell
$ w
21:41:07 up 12 days, 10:08,  2 users,  load average: 0.28, 0.20, 0.10
USER      TTY      FROM        LOGIN@   IDLE   JCPU   PCPU  WHAT
root      pts/0    10.10.0.2   20:59    1.00s  0.02s  0.00s w
qpanda    pts/1    10.10.0.8   21:41    7.00s  0.00s  0.00s zsh
qpanda    pts/1    10.10.0.7   20:01    7.00s  0.00s  0.00s zsh
$ w | semfilter '$id == qpanda and $ipv4Address == 10.10.0.7'
qpanda    pts/1    10.10.0.7   20:01    7.00s  0.00s  0.00s zsh
$ w | semfilter --time-format '%R' '$id == qpanda and $time > 21:00'
qpanda    pts/1    10.10.0.8   21:41    7.00s  0.00s  0.00s zsh
$
```

## Motivation
There are plenty of tools to parse and process data in well defined text based formats. There is [jq](https://stedolan.github.io/jq/) a flexible JSON processor, [yq](https://github.com/mikefarah/yq) a portable YAML processor, [xsv](https://github.com/BurntSushi/xsv) which can be used to analyze CSV files, and [XMLStarlet](https://en.wikipedia.org/wiki/XMLStarlet) which provides a set of command line utilities to process XML.

**semfilter** tries to fill a gap by provide a way to perform ad-hoc analysis of semi-structured and unstructured text output from other command line tools.

## License
**semfilter** is licensed under the MIT license.