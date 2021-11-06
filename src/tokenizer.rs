#[derive(Debug, PartialEq)]
pub struct Token {
    pub position: usize,
    pub separator: bool,
    pub text: String, // TODO could make this a &str to improve performance
}

pub struct Tokenizer {}

impl Tokenizer {
    // TODO need way to customize separators
    pub fn new() -> Self {
        Tokenizer {}
    }

    // https://stackoverflow.com/questions/32257273/
    pub fn tokens(&self, line: &str) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut last = 0;
        let mut position = 0;
        // TODO allow separators other than whitespaces
        for (index, seperator) in line.match_indices(|c: char| c.is_whitespace()) {
            if last != index {
                tokens.push(Token {
                    position: position,
                    separator: false,
                    text: String::from(&line[last..index]),
                });
                position += 1;
            }
            tokens.push(Token {
                position: position,
                separator: true,
                text: String::from(seperator),
            });
            position += 1;
            last = index + seperator.len();
        }
        if last != line.len() {
            tokens.push(Token {
                position: position,
                separator: false,
                text: String::from(&line[last..line.len()]),
            });
        }

        return tokens;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_only() {
        // setup
        let tokenizer = Tokenizer::new();

        // exercise
        let tokens = tokenizer.tokens("test");

        // verify
        assert_eq!(
            &tokens,
            &[Token {
                position: 0,
                separator: false,
                text: String::from("test")
            }]
        );
    }

    #[test]
    fn separator_only() {
        // setup
        let tokenizer = Tokenizer::new();

        // exercise
        let tokens = tokenizer.tokens(" ");

        // verify
        assert_eq!(
            &tokens,
            &[Token {
                position: 0,
                separator: true,
                text: String::from(" ")
            }]
        );
    }

    #[test]
    fn separators_only() {
        // setup
        let tokenizer = Tokenizer::new();

        // exercise
        let tokens = tokenizer.tokens("  ");

        // verify
        assert_eq!(
            &tokens,
            &[
                Token {
                    position: 0,
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    position: 1,
                    separator: true,
                    text: String::from(" ")
                }
            ]
        );
    }

    #[test]
    fn value_separator_value() {
        // setup
        let tokenizer = Tokenizer::new();

        // exercise
        let tokens = tokenizer.tokens("a b");

        // verify
        assert_eq!(
            &tokens,
            &[
                Token {
                    position: 0,
                    separator: false,
                    text: String::from("a")
                },
                Token {
                    position: 1,
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    position: 2,
                    separator: false,
                    text: String::from("b")
                }
            ]
        );
    }

    #[test]
    fn value_separator_separator_value() {
        // setup
        let tokenizer = Tokenizer::new();

        // exercise
        let tokens = tokenizer.tokens("a  b");

        // verify
        assert_eq!(
            &tokens,
            &[
                Token {
                    position: 0,
                    separator: false,
                    text: String::from("a")
                },
                Token {
                    position: 1,
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    position: 2,
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    position: 3,
                    separator: false,
                    text: String::from("b")
                }
            ]
        );
    }

    #[test]
    fn separator_value_separator() {
        // setup
        let tokenizer = Tokenizer::new();

        // exercise
        let tokens = tokenizer.tokens(" a ");

        // verify
        assert_eq!(
            &tokens,
            &[
                Token {
                    position: 0,
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    position: 1,
                    separator: false,
                    text: String::from("a")
                },
                Token {
                    position: 2,
                    separator: true,
                    text: String::from(" ")
                }
            ]
        );
    }

    #[test]
    fn separator_separator_value_separator_separator() {
        // setup
        let tokenizer = Tokenizer::new();

        // exercise
        let tokens = tokenizer.tokens("  a  ");

        // verify
        assert_eq!(
            &tokens,
            &[
                Token {
                    position: 0,
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    position: 1,
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    position: 2,
                    separator: false,
                    text: String::from("a")
                },
                Token {
                    position: 3,
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    position: 4,
                    separator: true,
                    text: String::from(" ")
                }
            ]
        );
    }

    #[test]
    fn line() {
        // setup
        let tokenizer = Tokenizer::new();

        // exercise
        let tokens = tokenizer.tokens("this is a simple line");

        // verify
        assert_eq!(
            &tokens,
            &[
                Token {
                    position: 0,
                    separator: false,
                    text: String::from("this")
                },
                Token {
                    position: 1,
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    position: 2,
                    separator: false,
                    text: String::from("is")
                },
                Token {
                    position: 3,
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    position: 4,
                    separator: false,
                    text: String::from("a")
                },
                Token {
                    position: 5,
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    position: 6,
                    separator: false,
                    text: String::from("simple")
                },
                Token {
                    position: 7,
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    position: 8,
                    separator: false,
                    text: String::from("line")
                }
            ]
        );
    }
}
