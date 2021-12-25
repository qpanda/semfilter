use anyhow::{anyhow, Error};
use std::collections::HashSet;

pub type Position = usize;

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub position: Position,
    pub separator: bool,
    pub word: &'a str,
}

pub const WHITESPACES: &str = "[:space:]";
pub const SEPARATORS: &'static [&'static str] = &[
    " ", ",", ";", "|", "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", "-", ".", "/", ":", "<", "=", ">", "?",
    "@", "[", "\\", "]", "^", "_", "`", "{", "}", "~",
];

pub struct Separators {
    whitespaces: bool,
    characters: HashSet<char>,
}

impl Separators {
    pub fn new(separators: Vec<&str>) -> Result<Self, Error> {
        let mut whitespaces = false;
        let mut characters = HashSet::new();

        for separator in separators {
            if WHITESPACES == separator {
                whitespaces = true;
            } else if SEPARATORS.contains(&separator) {
                characters.insert(separator.chars().next().unwrap());
            } else {
                return Err(anyhow!("invalid separator '{}'", separator));
            }
        }

        Ok(Separators {
            whitespaces: whitespaces,
            characters: characters,
        })
    }

    pub fn comprise_any(&self, characters: &str) -> String {
        let mut separators = String::new();

        for character in characters.chars() {
            if self.comprise(character) {
                separators.push(character);
            }
        }

        return separators;
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
        assert!(Separators::new(vec![WHITESPACES]).is_ok());
        assert!(Separators::new(vec![" "]).is_ok());
        assert!(Separators::new(vec![","]).is_ok());
        assert!(Separators::new(vec![";"]).is_ok());
        assert!(Separators::new(vec!["|"]).is_ok());
    }

    #[test]
    fn valid_separators() {
        assert!(Separators::new(vec![" ", ","]).is_ok());
    }

    #[test]
    fn invalid_separator() {
        assert!(Separators::new(vec!["0"]).is_err());
        assert!(Separators::new(vec!["A"]).is_err());
    }

    #[test]
    fn valid_and_invalid_separators() {
        assert!(Separators::new(vec![" ", "A"]).is_err());
        assert!(Separators::new(vec!["A", " "]).is_err());
    }

    #[test]
    fn comprise() {
        // setup
        let separators = Separators::new(vec![" "]).unwrap();

        // exercise & verify
        assert!(separators.comprise(' '));
        assert!(!separators.comprise('a'));
    }

    #[test]
    fn comprise_any() {
        // setup
        let separators = Separators::new(vec![" ", ","]).unwrap();

        // exercise & verify
        assert_eq!(separators.comprise_any("abc def"), " ");
        assert_eq!(separators.comprise_any("abc,def"), ",");
        assert_eq!(separators.comprise_any("abcdef"), "");
    }
}

#[cfg(test)]
mod tokenizer_tests {
    use super::*;

    #[test]
    fn value_only() {
        // setup
        let separators = Separators::new(vec![" "]).unwrap();
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
        let separators = Separators::new(vec![" "]).unwrap();
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
        let separators = Separators::new(vec![" "]).unwrap();
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
        let separators = Separators::new(vec![" "]).unwrap();
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
        let separators = Separators::new(vec![" "]).unwrap();
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
        let separators = Separators::new(vec![" "]).unwrap();
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
        let separators = Separators::new(vec![" "]).unwrap();
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
        let separators = Separators::new(vec![" "]).unwrap();
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
        let separators = Separators::new(vec![","]).unwrap();
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
        let separators = Separators::new(vec![WHITESPACES]).unwrap();
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
        let separators = Separators::new(vec![",", " "]).unwrap();
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
