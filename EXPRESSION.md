Expression Syntax
================
An `expression` can be a single `condition` or multiple `condition`s combined with `operator`s. In complex `expression`s parenthesis can be used to group `condition`s. Each `condition` compares a typed `variable` with a literal `value` using a `comparator`. A `function` can be applied to some typed `variable`s before comparison with the literal `value`.

## Overview
The supported `operator`s, `comparator`s, `variable`s, and `function`s, and how an `expression` is constructed using `condition`s is shown in EBNF below.

```ebnf
<expression>           ::=  <conditions>
<conditions>           ::=  <condition> |
                            <conditions> <operator> <conditions> |
                            ( <conditions> )
<operator>             ::=  and | or
<condition>            ::=  <variable> <comperator> <value> |
                            <function>(<variable>) <comperator> <value>
<comperator>           ::=  <basic-comperator> | <extended-comperator>
<basic-comperator>     ::=  == | != | > | >= | < | <=
<extended-comperator>  ::=  contains | starts-with | ends-with |
                            in | not in | matches
<function>             ::=  port | ip
<variable>             ::=  $integer | $float | $id | $date | $time |
                            $dateTime | $localDateTime | $ipAddress |
                            $ipv4Address | $ipv6Address | $ipSocketAddress |
                            $ipv4SocketAddress | $ipv6SocketAddress |
                            $ipNetwork | $ipv4Network | $ipv6Network |
                            $semanticVersion
<value>                ::=  <integer> | <float> | <id> | <date> | <time> |
                            <dateTime> | <localDateTime> | <ipAddress> |
                            <ipv4Address> | <ipv6Address> | <ipSocketAddress> |
                            <ipv4SocketAddress> | <ipv6SocketAddress> |
                            <ipNetwork> | <ipv4Network> | <ipv6Network> |
                            <semanticVersion> | <semanticVersionRequirement> |
                            <port>
```

## Examples
`'$semanticVersion >= 0.2.0'`\
Match all lines containing a semantic version value greater than or equal to `0.2.0`

`'$id == qpanda and $time > 21:00:00'`\
Match all lines containing an id value equal to `qpanda` and a time value greater than `21:00:00`

`'$id == ESTABLISHED and ip($ipv4SocketAddress) in 193.32.160.0/24'`\
Match all lines containing an id value equal to `ESTABLISHED` and a IPv4 socket address which has an IPv4 address in IPv4 network `193.32.160.0/24`

## Conditions
The expected format of the literal `value` in a `condition` depends on the `variable` type and the `comperator` being used. Which `comperator` can be used depends on the `variable` type; `basic-comperator`s are supported for all types whereas `extended-comperator`s are supported only for some types. The following table shows all supported combinations.

| Variable             | Comperators                                                                              | Value                          |
| ---                  | ---                                                                                      | ---                            |
| `$integer`           | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=`                                               | `<integer>`                    |
| `$float`             | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=`                                               | `<float>`                      |
| `$id`                | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=` \| `contains` \| `starts-with` \| `ends-with` | `<id>`                         |
| `$date`              | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=`                                               | `<date>`                       |
| `$time`              | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=`                                               | `<time>`                       |
| `$dateTime`          | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=`                                               | `<dateTime>`                   |
| `$localDateTime`     | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=`                                               | `<localDateTime>`              |
| `$ipAddress`         | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=`                                               | `<ipAddress>`                  |
| `$ipAddress`         | `in` \| `not in `                                                                        | `<ipNetwork>`                  |
| `$ipv4Address`       | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=`                                               | `<ipv4Address>`                |
| `$ipv4Address`       | `in` \| `not in `                                                                        | `<ipv4Network>`                |
| `$ipv6Address`       | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=`                                               | `<ipv6Address>`                |
| `$ipv6Address`       | `in` \| `not in `                                                                        | `<ipv6Network>`                |
| `$ipSocketAddress`   | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=`                                               | `<ipSocketAddress>`            |
| `$ipv4SocketAddress` | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=`                                               | `<ipv4SocketAddress>`          |
| `$ipv6SocketAddress` | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=`                                               | `<ipv6SocketAddress>`          |
| `$ipNetwork`         | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=`                                               | `<ipNetwork>`                  |
| `$ipv4Network`       | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=`                                               | `<ipv4Network>`                |
| `$ipv6Network`       | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=`                                               | `<ipv6Network>`                |
| `$semanticVersion`   | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=`                                               | `<semanticVersion>`            |
| `$semanticVersion`   | `matches`                                                                               | `<semanticVersionRequirement>` |

## Functions
In `condition`s `function`s can be applied only to some `variable` types. The following table shows all supported combinations.

| Function Expression        | Comperators                                | Value           |
| ---                        | ---                                        | ---             |
| `port($ipSocketAddress)`   | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=` | `<port>`        |
| `port($ipv4SocketAddress)` | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=` | `<port>`        |
| `port($ipv6SocketAddress)` | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=` | `<port>`        |
| `ip($ipSocketAddress)`     | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=` | `<ipAddress>`   |
| `ip($ipv4SocketAddress)`   | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=` | `<ipv4Address>` |
| `ip($ipv6SocketAddress)`   | `==` \| `!=` \| `>` \| `>=` \| `<` \| `<=` | `<ipv6Address>` |
| `ip($ipSocketAddress)`     | `in` \| `not in`                           | `<ipNetwork>`   |
| `ip($ipv4SocketAddress)`   | `in` \| `not in`                           | `<ipv4Network>` |
| `ip($ipv6SocketAddress)`   | `in` \| `not in`                           | `<ipv6Network>` |

## Values
The format of the literal `value`s is shown in the following table.

| Value                        | Pattern                                                                                | Description                                                                                              |
| ---                          | ---                                                                                    | ---                                                                                                      |
| `integer`                    | `['+'\|'-']? ['0'..='9']+`                                                             | valid signed integer                                                                                     |
| `float`                      | `['+'\|'-']? ['0'..='9']* ['.']? ['0'..='9']*`                                         | valid signed float[^1]                                                                                   |
| `id`                         | `['a'..='z'\|'A'..='Z']+ ['a'..='z'\|'A'..='Z'\|'0'..='9'\|'+'\|'-'\|'.'\|':'\|'_']*`  | any string conforming to the pattern                                                                     |
| `date`                       | `[^'('\|')'\|' ']+`                                                                    | valid date in configured date format[^2]                                                                 |
| `time`                       | `[^'('\|')'\|' ']+`                                                                    | valid time in configured time format[^3]                                                                 |
| `dateTime`                   | `[^'('\|')'\|' ']+`                                                                    | valid dateTime in configured dateTime format[^4]                                                         |
| `localDateTime`              | `[^'('\|')'\|' ']+`                                                                    | valid localDateTime in configured localDateTime format[^5]                                               |
| `ipAddress`                  | `['0'..='9'\|'a'..='f'\|'A'..='F'\|'.'\|':']+`                                         | valid IP address                                                                                         |
| `ipv4Address`                | `['0'..='9'\|'.']+`                                                                    | valid IPv4 address                                                                                       |
| `ipv6Address`                | `['0'..='9'\|'a'..='f'\|'A'..='F'\|':']+`                                              | valid IPv6 address                                                                                       |
| `ipSocketAddress`            | `['0'..='9'\|'a'..='f'\|'A'..='F'\|'.'\|':'\|'['\|']']+`                               | valid IP socket address                                                                                  |
| `ipv4SocketAddress`          | `['0'..='9'\|'.'\|':']+`                                                               | valid IPv4 socket address                                                                                |
| `ipv6SocketAddress`          | `['0'..='9'\|'a'..='f'\|'A'..='F'\|':'\|'['\|']']+`                                    | valid IPv6 socket address                                                                                |
| `ipNetwork`                  | `['0'..='9'\|'a'..='f'\|'A'..='F'\|'.'\|':'\|'/']+`                                    | valid IP network address                                                                                 |
| `ipv4Network`                | `['0'..='9'\|'.'\|'/']+`                                                               | valid IPv4 network address                                                                               |
| `ipv6Network`                | `['0'..='9'\|'a'..='f'\|'A'..='F'\|':'\|'/']+`                                         | valid IPv6 socket address                                                                                |
| `semanticVersion`            | `['0'..='9'\|'a'..='f'\|'A'..='F'\|'.'\|'-'\|'+']+`                                    | valid [semantic version](https://docs.rs/semver/latest/semver/struct.Version.html#syntax)                |
| `semanticVersionRequirement` | `['0'..='9'\|'a'..='f'\|'A'..='F'\|'.'\|'-'\|'+'\|'>'\|'<'\|'='\|'~'\|'^'\|'*'\|',']+` | valid [semantic version requirement](https://docs.rs/semver/latest/semver/struct.VersionReq.html#syntax) |
| `port`                       | `['0'..='9']+`                                                                         | valid unsigned integer                                                                                   |

[^1]: scientific notation, infinity, negative infinity, and not-a-number are not supported
[^2]: the date format can be specified in [`chrono::format::strftime`](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html) formatting syntax using the `date-format` command argument, the default date format is `%F`
[^3]: the time format can be specified in [`chrono::format::strftime`](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html) formatting syntax using the `time-format` command argument, the default time format is `%T`
[^4]: the dateTime format can be specified in [`chrono::format::strftime`](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html) formatting syntax using the `date-time-format` command argument, the default dateTime format is `%+`
[^5]: the localDateTime format can be specified in [`chrono::format::strftime`](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html) formatting syntax using the `local-date-time-format` command argument, the default localDateTime format is `%Y-%m-%dT%H:%M:%S%.f`
