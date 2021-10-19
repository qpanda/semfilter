pub struct Parser {}

#[derive(Debug, PartialEq)]
pub struct Token {
    separator: bool,
    text: String,
}

impl Parser {
    // TODO need way to customize separators
    pub fn new() -> Parser {
        Parser {}
    }

    // https://stackoverflow.com/questions/32257273/
    fn tokenize<'a>(&self, line: &str) -> Vec<Token> {
        let mut result = Vec::new();

        let mut last = 0;
        // TODO allow separators other than whitespaces
        for (index, seperator) in line.match_indices(|c: char| c.is_whitespace()) {
            if last != index {
                result.push(Token {
                    separator: false,
                    text: String::from(&line[last..index]),
                });
            }
            result.push(Token {
                separator: true,
                text: String::from(seperator),
            });
            last = index + seperator.len();
        }
        if last != line.len() {
            result.push(Token {
                separator: false,
                text: String::from(&line[last..line.len()]),
            });
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn value_only() {
        // setup
        let parser = Parser::new();

        // exercise
        let tokens = parser.tokenize("test");

        // verify
        assert_eq!(
            &tokens,
            &[Token {
                separator: false,
                text: String::from("test")
            }]
        );
    }

    #[test]
    fn separator_only() {
        // setup
        let parser = Parser::new();

        // exercise
        let tokens = parser.tokenize(" ");

        // verify
        assert_eq!(
            &tokens,
            &[Token {
                separator: true,
                text: String::from(" ")
            }]
        );
    }

    #[test]
    fn separators_only() {
        // setup
        let parser = Parser::new();

        // exercise
        let tokens = parser.tokenize("  ");

        // verify
        assert_eq!(
            &tokens,
            &[
                Token {
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    separator: true,
                    text: String::from(" ")
                }
            ]
        );
    }

    #[test]
    fn value_separator_value() {
        // setup
        let parser = Parser::new();

        // exercise
        let tokens = parser.tokenize("a b");

        // verify
        assert_eq!(
            &tokens,
            &[
                Token {
                    separator: false,
                    text: String::from("a")
                },
                Token {
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    separator: false,
                    text: String::from("b")
                }
            ]
        );
    }

    #[test]
    fn value_separator_separator_value() {
        // setup
        let parser = Parser::new();

        // exercise
        let tokens = parser.tokenize("a  b");

        // verify
        assert_eq!(
            &tokens,
            &[
                Token {
                    separator: false,
                    text: String::from("a")
                },
                Token {
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    separator: false,
                    text: String::from("b")
                }
            ]
        );
    }

    #[test]
    fn separator_value_separator() {
        // setup
        let parser = Parser::new();

        // exercise
        let tokens = parser.tokenize(" a ");

        // verify
        assert_eq!(
            &tokens,
            &[
                Token {
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    separator: false,
                    text: String::from("a")
                },
                Token {
                    separator: true,
                    text: String::from(" ")
                }
            ]
        );
    }

    #[test]
    fn separator_separator_value_separator_separator() {
        // setup
        let parser = Parser::new();

        // exercise
        let tokens = parser.tokenize("  a  ");

        // verify
        assert_eq!(
            &tokens,
            &[
                Token {
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    separator: false,
                    text: String::from("a")
                },
                Token {
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    separator: true,
                    text: String::from(" ")
                }
            ]
        );
    }

    #[test]
    fn line() {
        // setup
        let parser = Parser::new();

        // exercise
        let tokens = parser.tokenize("this is a simple line");

        // verify
        assert_eq!(
            &tokens,
            &[
                Token {
                    separator: false,
                    text: String::from("this")
                },
                Token {
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    separator: false,
                    text: String::from("is")
                },
                Token {
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    separator: false,
                    text: String::from("a")
                },
                Token {
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    separator: false,
                    text: String::from("simple")
                },
                Token {
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    separator: false,
                    text: String::from("line")
                }
            ]
        );
    }
}
