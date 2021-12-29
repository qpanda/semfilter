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

Run `semfilter -h` from the terminal window for an overview of supported `FLAGS` and `OPTIONS`.

Run `semfilter --help` from the terminal window for detailed information on `FLAGS` and `OPTIONS` and an explanation of the `expression` syntax.

> **Note**: By default **semfilter** reads from **stdin** and writes to **stdout**.

## Examples
**Filtering Git tags**
```console
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
```console
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

**Filtering netstat output**
```console
$ netstat -nt
Active Internet connections (w/o servers)
Proto Recv-Q Send-Q Local Address           Foreign Address         State
tcp        1      0 109.74.193.253:25       193.32.160.143:41356    ESTABLISHED
tcp        0      0 109.74.193.253:22       79.131.135.223:64917    ESTABLISHED
tcp        1      0 109.74.193.253:25       193.32.160.136:37752    CLOSE_WAIT
tcp        1      0 109.74.193.253:25       193.32.160.136:49900    CLOSE_WAIT
tcp        1      0 109.74.193.253:25       193.32.160.136:37752    ESTABLISHED
tcp        1      0 109.74.193.253:25       193.32.160.136:49900    CLOSE_WAIT
tcp        0      0 109.74.193.253:80       104.18.40.175:26111     SYN_RECV
tcp        0      0 109.74.193.253:80       104.18.40.175:47427     SYN_RECV
tcp        0      0 109.74.193.253:80       104.18.40.175:4436      SYN_RECV
tcp        0      0 109.74.193.253:80       104.18.41.175:12892     SYN_RECV
$ netstat -nt | semfilter '$id == ESTABLISHED and ip($ipv4SocketAddress) in 193.32.160.0/24'
tcp        1      0 109.74.193.253:25       193.32.160.143:41356    ESTABLISHED
tcp        1      0 109.74.193.253:25       193.32.160.136:37752    ESTABLISHED
$ 
```

## Motivation
There are plenty of tools to parse and process data in well defined text based formats. There is [jq](https://stedolan.github.io/jq/) a flexible JSON processor, [yq](https://github.com/mikefarah/yq) a portable YAML processor, [xsv](https://github.com/BurntSushi/xsv) which can be used to analyze CSV files, and [XMLStarlet](https://en.wikipedia.org/wiki/XMLStarlet) which provides a set of command line utilities to process XML.

**semfilter** tries to fill a gap by provide a way to perform ad-hoc analysis of semi-structured and unstructured text output from other command line tools.

## License
**semfilter** is licensed under the MIT license.