use anyhow::{anyhow, Error};
use std::collections::HashSet;

pub const WHITESPACE: &str = "[:space:]";
pub const SPACE: &str = " ";
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const PIPE: &str = "|";

pub type Position = usize;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub position: Position,
    pub separator: bool,
    pub text: String, // TODO could make this a &str to improve performance
}

pub struct Tokenizer {
    split_on_whitespace: bool,
    split_on_chars: HashSet<char>,
}

impl Tokenizer {
    pub fn new(separators: Vec<String>) -> Result<Self, Error> {
        let mut split_on_whitespace = false;
        let mut split_on_chars = HashSet::new();

        for separator in separators {
            if separator == WHITESPACE {
                split_on_whitespace = true;
            } else if separator == SPACE {
                split_on_chars.insert(SPACE.chars().next().expect("space separator invalid"));
            } else if separator == COMMA {
                split_on_chars.insert(COMMA.chars().next().expect("comma separator invalid"));
            } else if separator == SEMICOLON {
                split_on_chars.insert(SEMICOLON.chars().next().expect("semicolon separator invalid"));
            } else if separator == PIPE.to_string() {
                split_on_chars.insert(PIPE.chars().next().expect("pipe separator invalid"));
            } else {
                return Err(anyhow!("invalid separator '{}'", separator));
            }
        }

        Ok(Tokenizer {
            split_on_whitespace: split_on_whitespace,
            split_on_chars: split_on_chars,
        })
    }

    // https://stackoverflow.com/questions/32257273/
    pub fn tokens(&self, line: &str) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut last = 0;
        let mut position = 0;
        for (index, seperator) in line.match_indices(|c: char| self.is_separator(c)) {
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

    fn is_separator(&self, c: char) -> bool {
        if self.split_on_whitespace && c.is_whitespace() {
            return true;
        }

        return self.split_on_chars.contains(&c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_separator() {
        assert!(Tokenizer::new(vec![String::from(WHITESPACE)]).is_ok());
        assert!(Tokenizer::new(vec![String::from(SPACE)]).is_ok());
        assert!(Tokenizer::new(vec![String::from(COMMA)]).is_ok());
        assert!(Tokenizer::new(vec![String::from(SEMICOLON)]).is_ok());
        assert!(Tokenizer::new(vec![String::from(PIPE)]).is_ok());
    }

    #[test]
    fn valid_separators() {
        assert!(Tokenizer::new(vec![String::from(SPACE), String::from(COMMA)]).is_ok());
    }

    #[test]
    fn invalid_separator() {
        assert!(Tokenizer::new(vec![String::from(".")]).is_err());
        assert!(Tokenizer::new(vec![String::from("*")]).is_err());
    }

    #[test]
    fn invalid_separators() {
        assert!(Tokenizer::new(vec![String::from(SPACE), String::from(".")]).is_err());
    }

    #[test]
    fn value_only() {
        // setup
        let separators = vec![String::from(SPACE)];
        let tokenizer = Tokenizer::new(separators).unwrap();

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
        let separators = vec![String::from(SPACE)];
        let tokenizer = Tokenizer::new(separators).unwrap();

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
        let separators = vec![String::from(SPACE)];
        let tokenizer = Tokenizer::new(separators).unwrap();

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
        let separators = vec![String::from(SPACE)];
        let tokenizer = Tokenizer::new(separators).unwrap();

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
        let separators = vec![String::from(SPACE)];
        let tokenizer = Tokenizer::new(separators).unwrap();

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
        let separators = vec![String::from(SPACE)];
        let tokenizer = Tokenizer::new(separators).unwrap();

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
        let separators = vec![String::from(SPACE)];
        let tokenizer = Tokenizer::new(separators).unwrap();

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
        let separators = vec![String::from(SPACE)];
        let tokenizer = Tokenizer::new(separators).unwrap();

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

    #[test]
    fn value_comma_value() {
        // setup
        let separators = vec![String::from(COMMA)];
        let tokenizer = Tokenizer::new(separators).unwrap();

        // exercise
        let tokens = tokenizer.tokens("a,b");

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
                    text: String::from(",")
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
    fn value_whitespace_value() {
        // setup
        let separators = vec![String::from(WHITESPACE)];
        let tokenizer = Tokenizer::new(separators).unwrap();

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
    fn value_comma_or_space_value() {
        // setup
        let separators = vec![String::from(COMMA), String::from(SPACE)];
        let tokenizer = Tokenizer::new(separators).unwrap();

        // exercise
        let tokens = tokenizer.tokens("a,b c");

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
                    text: String::from(",")
                },
                Token {
                    position: 2,
                    separator: false,
                    text: String::from("b")
                },
                Token {
                    position: 3,
                    separator: true,
                    text: String::from(" ")
                },
                Token {
                    position: 4,
                    separator: false,
                    text: String::from("c")
                }
            ]
        );
    }
}
