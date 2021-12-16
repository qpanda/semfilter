use anyhow::{anyhow, Error};
use std::collections::HashSet;
use std::str::Chars;

pub type Position = usize;

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub position: Position,
    pub separator: bool,
    pub word: &'a str,
}

pub const WHITESPACE: &str = "[:space:]";
pub const SPACE: &str = " ";
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const PIPE: &str = "|";

pub struct Separators {
    whitespaces: bool,
    characters: HashSet<char>,
}

impl Separators {
    pub fn new(separators: Vec<&str>) -> Result<Self, Error> {
        let mut whitespaces = false;
        let mut characters = HashSet::new();

        for separator in separators {
            if separator == WHITESPACE {
                whitespaces = true;
            } else if separator == SPACE {
                characters.insert(SPACE.chars().next().expect("space separator invalid"));
            } else if separator == COMMA {
                characters.insert(COMMA.chars().next().expect("comma separator invalid"));
            } else if separator == SEMICOLON {
                characters.insert(SEMICOLON.chars().next().expect("semicolon separator invalid"));
            } else if separator == PIPE {
                characters.insert(PIPE.chars().next().expect("pipe separator invalid"));
            } else {
                return Err(anyhow!("invalid separator '{}'", separator));
            }
        }

        Ok(Separators {
            whitespaces: whitespaces,
            characters: characters,
        })
    }

    pub fn comprise_any(&self, characters: Chars) -> bool {
        for character in characters {
            if self.comprise(character) {
                return true;
            }
        }

        return false;
    }

    pub fn comprise(&self, character: char) -> bool {
        if self.whitespaces && character.is_whitespace() {
            return true;
        }

        return self.characters.contains(&character);
    }
}

pub struct Tokenizer {
    separators: Separators,
}

impl Tokenizer {
    pub fn new(separators: Separators) -> Result<Self, Error> {
        Ok(Tokenizer { separators: separators })
    }

    // https://stackoverflow.com/questions/32257273/
    pub fn tokens<'a>(&self, line: &'a str) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();

        let mut last = 0;
        let mut position = 0;
        for (index, seperator) in line.match_indices(|c: char| self.separators.comprise(c)) {
            if last != index {
                tokens.push(Token {
                    position: position,
                    separator: false,
                    word: &line[last..index],
                });
                position += 1;
            }
            tokens.push(Token {
                position: position,
                separator: true,
                word: seperator,
            });
            position += 1;
            last = index + seperator.len();
        }
        if last != line.len() {
            tokens.push(Token {
                position: position,
                separator: false,
                word: &line[last..line.len()],
            });
        }

        return tokens;
    }
}

#[cfg(test)]
mod separators_tests {
    use super::*;

    #[test]
    fn valid_separator() {
        assert!(Separators::new(vec![WHITESPACE]).is_ok());
        assert!(Separators::new(vec![SPACE]).is_ok());
        assert!(Separators::new(vec![COMMA]).is_ok());
        assert!(Separators::new(vec![SEMICOLON]).is_ok());
        assert!(Separators::new(vec![PIPE]).is_ok());
    }

    #[test]
    fn valid_separators() {
        assert!(Separators::new(vec![SPACE, COMMA]).is_ok());
    }

    #[test]
    fn invalid_separator() {
        assert!(Separators::new(vec!["."]).is_err());
        assert!(Separators::new(vec!["*"]).is_err());
    }

    #[test]
    fn valid_and_invalid_separators() {
        assert!(Separators::new(vec![SPACE, "."]).is_err());
        assert!(Separators::new(vec![".", SPACE]).is_err());
    }

    #[test]
    fn comprise() {
        // setup
        let separators = Separators::new(vec![SPACE]).unwrap();

        // exercise & verify
        assert!(separators.comprise(' '));
        assert!(!separators.comprise('a'));
    }

    #[test]
    fn comprise_any() {
        // setup
        let separators = Separators::new(vec![SPACE]).unwrap();

        // exercise & verify
        assert!(separators.comprise_any("abc def".chars()));
        assert!(!separators.comprise_any("abcdef".chars()));
    }
}

#[cfg(test)]
mod tokenizer_tests {
    use super::*;

    #[test]
    fn value_only() {
        // setup
        let separators = Separators::new(vec![SPACE]).unwrap();
        let tokenizer = Tokenizer::new(separators).unwrap();

        // exercise
        let tokens = tokenizer.tokens("test");

        // verify
        assert_eq!(
            &tokens,
            &[Token {
                position: 0,
                separator: false,
                word: "test"
            }]
        );
    }

    #[test]
    fn separator_only() {
        // setup
        let separators = Separators::new(vec![SPACE]).unwrap();
        let tokenizer = Tokenizer::new(separators).unwrap();

        // exercise
        let tokens = tokenizer.tokens(" ");

        // verify
        assert_eq!(
            &tokens,
            &[Token {
                position: 0,
                separator: true,
                word: " "
            }]
        );
    }

    #[test]
    fn separators_only() {
        // setup
        let separators = Separators::new(vec![SPACE]).unwrap();
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
                    word: " "
                },
                Token {
                    position: 1,
                    separator: true,
                    word: " "
                }
            ]
        );
    }

    #[test]
    fn value_separator_value() {
        // setup
        let separators = Separators::new(vec![SPACE]).unwrap();
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
                    word: "a"
                },
                Token {
                    position: 1,
                    separator: true,
                    word: " "
                },
                Token {
                    position: 2,
                    separator: false,
                    word: "b"
                }
            ]
        );
    }

    #[test]
    fn value_separator_separator_value() {
        // setup
        let separators = Separators::new(vec![SPACE]).unwrap();
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
                    word: "a"
                },
                Token {
                    position: 1,
                    separator: true,
                    word: " "
                },
                Token {
                    position: 2,
                    separator: true,
                    word: " "
                },
                Token {
                    position: 3,
                    separator: false,
                    word: "b"
                }
            ]
        );
    }

    #[test]
    fn separator_value_separator() {
        // setup
        let separators = Separators::new(vec![SPACE]).unwrap();
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
                    word: " "
                },
                Token {
                    position: 1,
                    separator: false,
                    word: "a"
                },
                Token {
                    position: 2,
                    separator: true,
                    word: " "
                }
            ]
        );
    }

    #[test]
    fn separator_separator_value_separator_separator() {
        // setup
        let separators = Separators::new(vec![SPACE]).unwrap();
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
                    word: " "
                },
                Token {
                    position: 1,
                    separator: true,
                    word: " "
                },
                Token {
                    position: 2,
                    separator: false,
                    word: "a"
                },
                Token {
                    position: 3,
                    separator: true,
                    word: " "
                },
                Token {
                    position: 4,
                    separator: true,
                    word: " "
                }
            ]
        );
    }

    #[test]
    fn line() {
        // setup
        let separators = Separators::new(vec![SPACE]).unwrap();
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
                    word: "this"
                },
                Token {
                    position: 1,
                    separator: true,
                    word: " "
                },
                Token {
                    position: 2,
                    separator: false,
                    word: "is"
                },
                Token {
                    position: 3,
                    separator: true,
                    word: " "
                },
                Token {
                    position: 4,
                    separator: false,
                    word: "a"
                },
                Token {
                    position: 5,
                    separator: true,
                    word: " "
                },
                Token {
                    position: 6,
                    separator: false,
                    word: "simple"
                },
                Token {
                    position: 7,
                    separator: true,
                    word: " "
                },
                Token {
                    position: 8,
                    separator: false,
                    word: "line"
                }
            ]
        );
    }

    #[test]
    fn value_comma_value() {
        // setup
        let separators = Separators::new(vec![COMMA]).unwrap();
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
                    word: "a"
                },
                Token {
                    position: 1,
                    separator: true,
                    word: ","
                },
                Token {
                    position: 2,
                    separator: false,
                    word: "b"
                }
            ]
        );
    }

    #[test]
    fn value_whitespace_value() {
        // setup
        let separators = Separators::new(vec![WHITESPACE]).unwrap();
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
                    word: "a"
                },
                Token {
                    position: 1,
                    separator: true,
                    word: " "
                },
                Token {
                    position: 2,
                    separator: false,
                    word: "b"
                }
            ]
        );
    }

    #[test]
    fn value_comma_or_space_value() {
        // setup
        let separators = Separators::new(vec![COMMA, SPACE]).unwrap();
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
                    word: "a"
                },
                Token {
                    position: 1,
                    separator: true,
                    word: ","
                },
                Token {
                    position: 2,
                    separator: false,
                    word: "b"
                },
                Token {
                    position: 3,
                    separator: true,
                    word: " "
                },
                Token {
                    position: 4,
                    separator: false,
                    word: "c"
                }
            ]
        );
    }
}
