use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub separator: bool,
    pub text: String, // TODO could make this a &str to improve performance
}

pub enum Class {
    Integer,
    Float,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Separator(String),
    Text(String),
    Integer(u64),
    Float(f64),
}

impl Value {
    // the order of Class in classes is important because it is the order in which we
    // try to convert token.text to a Value
    // Class::Integer ⊂ Class::Float
    // TODO should we sort classes here?
    pub fn new(token: &Token, classes: &Vec<Class>) -> Self {
        if token.separator {
            return Value::Separator(String::from(&token.text));
        }

        for class in classes {
            if let Ok(value) = Value::from(&token.text, &class) {
                return value;
            }
        }

        // we return Value::Text if (a) classes is empty or (b) none of the conversions to
        // a Class in classes succeeds
        return Value::Text(String::from(&token.text));
    }

    // TODO need to pass format string once we support dates / times / ...
    pub fn from(text: &String, class: &Class) -> Result<Self, Box<dyn Error>> {
        match class {
            Class::Integer => match text.parse::<u64>() {
                Ok(integer) => Ok(Value::Integer(integer)),
                Err(error) => Err(error.into()),
            },
            Class::Float => match text.parse::<f64>() {
                Ok(float) => Ok(Value::Float(float)),
                Err(error) => Err(error.into()),
            },
        }
    }
}

pub struct Parser {
    classes: Vec<Class>,
}

impl Parser {
    // TODO need way to customize separators
    pub fn new(classes: Vec<Class>) -> Self {
        // TODO should we sort classes here?
        Parser { classes: classes }
    }

    pub fn parse(&self, line: &str) -> Vec<(Token, Value)> {
        let tokens = self.tokenize(line);

        let mut result = Vec::new();
        for token in tokens {
            let value = Value::new(&token, &self.classes);
            result.push((token, value));
        }
        return result;
    }

    // https://stackoverflow.com/questions/32257273/
    pub fn tokenize(&self, line: &str) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut last = 0;
        // TODO allow separators other than whitespaces
        for (index, seperator) in line.match_indices(|c: char| c.is_whitespace()) {
            if last != index {
                tokens.push(Token {
                    separator: false,
                    text: String::from(&line[last..index]),
                });
            }
            tokens.push(Token {
                separator: true,
                text: String::from(seperator),
            });
            last = index + seperator.len();
        }
        if last != line.len() {
            tokens.push(Token {
                separator: false,
                text: String::from(&line[last..line.len()]),
            });
        }

        return tokens;
    }
}

#[cfg(test)]
mod value_tests {
    use super::*;

    #[test]
    fn from_integer() {
        // setup
        let integer = 8;
        let integer_text = integer.to_string();
        let float_text = "5.5";
        let text = "text";

        // exercise
        let ok = Value::from(&String::from(integer_text), &Class::Integer);
        let err_1 = Value::from(&String::from(float_text), &Class::Integer);
        let err_2 = Value::from(&String::from(text), &Class::Integer);

        // verify
        assert_eq!(Value::Integer(integer), ok.unwrap());
        assert_eq!(true, err_1.is_err());
        assert_eq!(true, err_2.is_err());
    }

    #[test]
    fn from_float() {
        // setup
        let float = 5.5;
        let integer = 8;
        let float_text = float.to_string();
        let integer_text = integer.to_string();
        let text = "text";

        // exercise
        let ok_1 = Value::from(&String::from(float_text), &Class::Float);
        let ok_2 = Value::from(&String::from(integer_text), &Class::Float);
        let err = Value::from(&String::from(text), &Class::Float);

        // verify
        assert_eq!(Value::Float(float), ok_1.unwrap());
        assert_eq!(Value::Float(integer as f64), ok_2.unwrap());
        assert_eq!(true, err.is_err());
    }

    #[test]
    fn new_separator() {
        // setup
        let separator_token = Token {
            separator: true,
            text: String::from(" "),
        };
        let text_token = Token {
            separator: false,
            text: String::from("text"),
        };
        let classes = Vec::new();

        // exercise
        let separator_value = Value::new(&separator_token, &classes);
        let text_value = Value::new(&text_token, &classes);

        // verify
        assert_eq!(Value::Separator(separator_token.text), separator_value);
        assert_eq!(Value::Text(text_token.text), text_value);
    }

    #[test]
    fn new_value() {
        // setup
        let text_token = Token {
            separator: false,
            text: String::from("text"),
        };
        let integer_token = Token {
            separator: false,
            text: String::from("8"),
        };
        let float_token = Token {
            separator: false,
            text: String::from("5.5"),
        };
        let classes = Vec::new();

        // exercise
        let text_value = Value::new(&text_token, &classes);
        let integer_value = Value::new(&integer_token, &classes);
        let float_value = Value::new(&float_token, &classes);

        // verify
        assert_eq!(Value::Text(text_token.text), text_value);
        assert_eq!(Value::Text(integer_token.text), integer_value);
        assert_eq!(Value::Text(float_token.text), float_value);
    }

    #[test]
    fn new_integer() {
        // setup
        let integer = 8;
        let text_token = Token {
            separator: false,
            text: String::from("text"),
        };
        let integer_token = Token {
            separator: false,
            text: integer.to_string(),
        };
        let float_token = Token {
            separator: false,
            text: String::from("5.5"),
        };
        let classes: Vec<Class> = vec![Class::Integer];

        // exercise
        let text_value = Value::new(&text_token, &classes);
        let integer_value = Value::new(&integer_token, &classes);
        let float_value = Value::new(&float_token, &classes);

        // verify
        assert_eq!(Value::Text(text_token.text), text_value);
        assert_eq!(Value::Integer(integer), integer_value);
        assert_eq!(Value::Text(float_token.text), float_value);
    }

    #[test]
    fn new_float() {
        // setup
        let integer = 8;
        let float = 5.5;
        let text_token = Token {
            separator: false,
            text: String::from("text"),
        };
        let integer_token = Token {
            separator: false,
            text: integer.to_string(),
        };
        let float_token = Token {
            separator: false,
            text: float.to_string(),
        };
        let classes: Vec<Class> = vec![Class::Float];

        // exercise
        let text_value = Value::new(&text_token, &classes);
        let float_value_1 = Value::new(&integer_token, &classes);
        let float_value_2 = Value::new(&float_token, &classes);

        // verify
        assert_eq!(Value::Text(text_token.text), text_value);
        assert_eq!(Value::Float(integer as f64), float_value_1);
        assert_eq!(Value::Float(float), float_value_2);
    }

    #[test]
    fn new_integer_or_float() {
        // setup
        let integer = 8;
        let float = 5.5;
        let text_token = Token {
            separator: false,
            text: String::from("text"),
        };
        let integer_token = Token {
            separator: false,
            text: integer.to_string(),
        };
        let float_token = Token {
            separator: false,
            text: float.to_string(),
        };
        let classes: Vec<Class> = vec![Class::Integer, Class::Float];

        // exercise
        let text_value = Value::new(&text_token, &classes);
        let integer_value = Value::new(&integer_token, &classes);
        let float_value = Value::new(&float_token, &classes);

        // verify
        assert_eq!(Value::Text(text_token.text), text_value);
        assert_eq!(Value::Integer(integer), integer_value);
        assert_eq!(Value::Float(float), float_value);
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn tokenize_value_only() {
        // setup
        let parser = Parser::new(Vec::new());

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
    fn tokenize_separator_only() {
        // setup
        let parser = Parser::new(Vec::new());

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
    fn tokenize_separators_only() {
        // setup
        let parser = Parser::new(Vec::new());

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
    fn tokenize_value_separator_value() {
        // setup
        let parser = Parser::new(Vec::new());

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
    fn tokenize_value_separator_separator_value() {
        // setup
        let parser = Parser::new(Vec::new());

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
    fn tokenize_separator_value_separator() {
        // setup
        let parser = Parser::new(Vec::new());

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
    fn tokenize_separator_separator_value_separator_separator() {
        // setup
        let parser = Parser::new(Vec::new());

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
    fn tokenize_line() {
        // setup
        let parser = Parser::new(Vec::new());

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

    #[test]
    fn parse_text() {
        // setup
        let parser = Parser::new(Vec::new());

        // exercise
        let results = parser.parse("integer 8 and float 5.5");

        // verify
        assert_eq!(9, results.len());
        assert_eq!(
            &results,
            &[
                (
                    Token {
                        separator: false,
                        text: String::from("integer"),
                    },
                    Value::Text(String::from("integer"))
                ),
                (
                    Token {
                        separator: true,
                        text: String::from(" "),
                    },
                    Value::Separator(String::from(" "))
                ),
                (
                    Token {
                        separator: false,
                        text: String::from("8"),
                    },
                    Value::Text(String::from("8"))
                ),
                (
                    Token {
                        separator: true,
                        text: String::from(" "),
                    },
                    Value::Separator(String::from(" "))
                ),
                (
                    Token {
                        separator: false,
                        text: String::from("and"),
                    },
                    Value::Text(String::from("and"))
                ),
                (
                    Token {
                        separator: true,
                        text: String::from(" "),
                    },
                    Value::Separator(String::from(" "))
                ),
                (
                    Token {
                        separator: false,
                        text: String::from("float"),
                    },
                    Value::Text(String::from("float"))
                ),
                (
                    Token {
                        separator: true,
                        text: String::from(" "),
                    },
                    Value::Separator(String::from(" "))
                ),
                (
                    Token {
                        separator: false,
                        text: String::from("5.5"),
                    },
                    Value::Text(String::from("5.5"))
                )
            ]
        );
    }

    #[test]
    fn parse_integer() {
        // setup
        let classes = vec![Class::Integer];
        let parser = Parser::new(classes);

        // exercise
        let results = parser.parse("integer 8 and float 5.5");

        // verify
        assert_eq!(9, results.len());
        assert_eq!(
            &results,
            &[
                (
                    Token {
                        separator: false,
                        text: String::from("integer"),
                    },
                    Value::Text(String::from("integer"))
                ),
                (
                    Token {
                        separator: true,
                        text: String::from(" "),
                    },
                    Value::Separator(String::from(" "))
                ),
                (
                    Token {
                        separator: false,
                        text: String::from("8"),
                    },
                    Value::Integer(8)
                ),
                (
                    Token {
                        separator: true,
                        text: String::from(" "),
                    },
                    Value::Separator(String::from(" "))
                ),
                (
                    Token {
                        separator: false,
                        text: String::from("and"),
                    },
                    Value::Text(String::from("and"))
                ),
                (
                    Token {
                        separator: true,
                        text: String::from(" "),
                    },
                    Value::Separator(String::from(" "))
                ),
                (
                    Token {
                        separator: false,
                        text: String::from("float"),
                    },
                    Value::Text(String::from("float"))
                ),
                (
                    Token {
                        separator: true,
                        text: String::from(" "),
                    },
                    Value::Separator(String::from(" "))
                ),
                (
                    Token {
                        separator: false,
                        text: String::from("5.5"),
                    },
                    Value::Text(String::from("5.5"))
                )
            ]
        );
    }

    #[test]
    fn parse_float() {
        // setup
        let classes = vec![Class::Float];
        let parser = Parser::new(classes);

        // exercise
        let results = parser.parse("integer 8 and float 5.5");

        // verify
        assert_eq!(9, results.len());
        assert_eq!(
            &results,
            &[
                (
                    Token {
                        separator: false,
                        text: String::from("integer"),
                    },
                    Value::Text(String::from("integer"))
                ),
                (
                    Token {
                        separator: true,
                        text: String::from(" "),
                    },
                    Value::Separator(String::from(" "))
                ),
                (
                    Token {
                        separator: false,
                        text: String::from("8"),
                    },
                    Value::Float(8.0)
                ),
                (
                    Token {
                        separator: true,
                        text: String::from(" "),
                    },
                    Value::Separator(String::from(" "))
                ),
                (
                    Token {
                        separator: false,
                        text: String::from("and"),
                    },
                    Value::Text(String::from("and"))
                ),
                (
                    Token {
                        separator: true,
                        text: String::from(" "),
                    },
                    Value::Separator(String::from(" "))
                ),
                (
                    Token {
                        separator: false,
                        text: String::from("float"),
                    },
                    Value::Text(String::from("float"))
                ),
                (
                    Token {
                        separator: true,
                        text: String::from(" "),
                    },
                    Value::Separator(String::from(" "))
                ),
                (
                    Token {
                        separator: false,
                        text: String::from("5.5"),
                    },
                    Value::Float(5.5)
                )
            ]
        );
    }

    #[test]
    fn parse_integer_or_float() {
        // setup
        let classes = vec![Class::Integer, Class::Float];
        let parser = Parser::new(classes);

        // exercise
        let results = parser.parse("integer 8 and float 5.5");

        // verify
        assert_eq!(9, results.len());
        assert_eq!(
            &results,
            &[
                (
                    Token {
                        separator: false,
                        text: String::from("integer"),
                    },
                    Value::Text(String::from("integer"))
                ),
                (
                    Token {
                        separator: true,
                        text: String::from(" "),
                    },
                    Value::Separator(String::from(" "))
                ),
                (
                    Token {
                        separator: false,
                        text: String::from("8"),
                    },
                    Value::Integer(8)
                ),
                (
                    Token {
                        separator: true,
                        text: String::from(" "),
                    },
                    Value::Separator(String::from(" "))
                ),
                (
                    Token {
                        separator: false,
                        text: String::from("and"),
                    },
                    Value::Text(String::from("and"))
                ),
                (
                    Token {
                        separator: true,
                        text: String::from(" "),
                    },
                    Value::Separator(String::from(" "))
                ),
                (
                    Token {
                        separator: false,
                        text: String::from("float"),
                    },
                    Value::Text(String::from("float"))
                ),
                (
                    Token {
                        separator: true,
                        text: String::from(" "),
                    },
                    Value::Separator(String::from(" "))
                ),
                (
                    Token {
                        separator: false,
                        text: String::from("5.5"),
                    },
                    Value::Float(5.5)
                )
            ]
        );
    }
}
