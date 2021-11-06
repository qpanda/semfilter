use std::error::Error;

use crate::tokenizer::Position;
use crate::tokenizer::Token;

pub enum Class {
    Integer,
    Float,
    Text,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(u64),
    Float(f64),
    Text(String),
}

#[derive(Debug, PartialEq)]
pub struct Term {
    position: Position,
    value: Value,
}

impl Value {
    pub fn new(text: &String, class: &Class) -> Result<Self, Box<dyn Error>> {
        match class {
            Class::Integer => match text.parse::<u64>() {
                Ok(integer) => Ok(Value::Integer(integer)),
                Err(error) => Err(error.into()),
            },
            Class::Float => match text.parse::<f64>() {
                Ok(float) => Ok(Value::Float(float)),
                Err(error) => Err(error.into()),
            },
            Class::Text => Ok(Value::Text(String::from(text))),
        }
    }
}

impl Term {
    pub fn from(tokens: &Vec<Token>, class: &Class) -> Vec<Term> {
        let mut result = Vec::new();
        for token in tokens {
            if !token.separator {
                if let Ok(value) = Value::new(&token.text, class) {
                    result.push(Term {
                        position: token.position,
                        value: value,
                    });
                }
            }
        }
        return result;
    }
}

#[cfg(test)]
mod value_tests {
    use super::*;

    #[test]
    fn new_integer() {
        // setup
        let integer = 8;
        let integer_text = integer.to_string();
        let float_text = "5.5";
        let text = "text";

        // exercise
        let ok = Value::new(&String::from(integer_text), &Class::Integer);
        let err_1 = Value::new(&String::from(float_text), &Class::Integer);
        let err_2 = Value::new(&String::from(text), &Class::Integer);

        // verify
        assert_eq!(Value::Integer(integer), ok.unwrap());
        assert_eq!(true, err_1.is_err());
        assert_eq!(true, err_2.is_err());
    }

    #[test]
    fn new_float() {
        // setup
        let float = 5.5;
        let integer = 8;
        let float_text = float.to_string();
        let integer_text = integer.to_string();
        let text = "text";

        // exercise
        let ok_1 = Value::new(&String::from(float_text), &Class::Float);
        let ok_2 = Value::new(&String::from(integer_text), &Class::Float);
        let err = Value::new(&String::from(text), &Class::Float);

        // verify
        assert_eq!(Value::Float(float), ok_1.unwrap());
        assert_eq!(Value::Float(integer as f64), ok_2.unwrap());
        assert_eq!(true, err.is_err());
    }

    #[test]
    fn new_text() {
        // setup
        let float = 5.5;
        let integer = 8;
        let float_text = float.to_string();
        let integer_text = integer.to_string();
        let text = String::from("text");

        // exercise
        let ok_1 = Value::new(&String::from(&float_text), &Class::Text);
        let ok_2 = Value::new(&String::from(&integer_text), &Class::Text);
        let ok_3 = Value::new(&String::from(&text), &Class::Text);

        // verify
        assert_eq!(Value::Text(float_text), ok_1.unwrap());
        assert_eq!(Value::Text(integer_text), ok_2.unwrap());
        assert_eq!(Value::Text(text), ok_3.unwrap());
    }
}

#[cfg(test)]
mod term_tests {
    use super::*;

    #[test]
    fn from_separator() {
        // setup
        let separator_token = Token {
            position: 0,
            separator: true,
            text: String::from(" "),
        };
        let tokens = vec![separator_token];

        // exercise
        let text_terms = Term::from(&tokens, &Class::Text);
        let integer_terms = Term::from(&tokens, &Class::Integer);
        let float_terms = Term::from(&tokens, &Class::Float);

        // verify
        assert_eq!(0, text_terms.len());
        assert_eq!(0, integer_terms.len());
        assert_eq!(0, float_terms.len());
    }

    #[test]
    fn from_text() {
        // setup
        let text = "text";
        let position = 0;
        let text_token = Token {
            position: position,
            separator: false,
            text: String::from(text),
        };
        let tokens = vec![text_token];

        // exercise
        let text_terms = Term::from(&tokens, &Class::Text);
        let integer_terms = Term::from(&tokens, &Class::Integer);
        let float_terms = Term::from(&tokens, &Class::Float);

        // verify
        assert_eq!(1, text_terms.len());
        assert_eq!(
            &Term {
                position: position,
                value: Value::new(&String::from(text), &Class::Text).unwrap(),
            },
            text_terms.get(0).unwrap()
        );

        assert_eq!(0, integer_terms.len());

        assert_eq!(0, float_terms.len());
    }

    #[test]
    fn from_integer() {
        // setup
        let text = "8";
        let position = 0;
        let integer_token = Token {
            position: position,
            separator: false,
            text: String::from(text),
        };
        let tokens = vec![integer_token];

        // exercise
        let text_terms = Term::from(&tokens, &Class::Text);
        let integer_terms = Term::from(&tokens, &Class::Integer);
        let float_terms = Term::from(&tokens, &Class::Float);

        // verify
        assert_eq!(1, text_terms.len());
        assert_eq!(
            &Term {
                position: position,
                value: Value::new(&String::from(text), &Class::Text).unwrap(),
            },
            text_terms.get(0).unwrap()
        );

        assert_eq!(1, integer_terms.len());
        assert_eq!(
            &Term {
                position: position,
                value: Value::new(&String::from(text), &Class::Integer).unwrap(),
            },
            integer_terms.get(0).unwrap()
        );

        assert_eq!(1, float_terms.len());
        assert_eq!(
            &Term {
                position: position,
                value: Value::new(&String::from(text), &Class::Float).unwrap(),
            },
            float_terms.get(0).unwrap()
        );
    }

    #[test]
    fn from_float() {
        // setup
        let text = "5.5";
        let position = 0;
        let float_token = Token {
            position: position,
            separator: false,
            text: String::from(text),
        };
        let tokens = vec![float_token];

        // exercise
        let text_terms = Term::from(&tokens, &Class::Text);
        let integer_terms = Term::from(&tokens, &Class::Integer);
        let float_terms = Term::from(&tokens, &Class::Float);

        // verify
        assert_eq!(1, text_terms.len());
        assert_eq!(
            &Term {
                position: position,
                value: Value::new(&String::from(text), &Class::Text).unwrap(),
            },
            text_terms.get(0).unwrap()
        );

        assert_eq!(0, integer_terms.len());

        assert_eq!(1, float_terms.len());
        assert_eq!(
            &Term {
                position: position,
                value: Value::new(&String::from(text), &Class::Float).unwrap(),
            },
            float_terms.get(0).unwrap()
        );
    }

    #[test]
    fn to_text() {
        // setup
        let position0 = 0;
        let position2 = 2;
        let position4 = 4;
        let position6 = 6;
        let position8 = 8;
        let text0 = "integer";
        let text2 = "8";
        let text4 = "and";
        let text6 = "float";
        let text8 = "5.5";
        let tokens = vec![
            Token {
                position: position0,
                separator: false,
                text: String::from(text0),
            },
            Token {
                position: 1,
                separator: true,
                text: String::from(" "),
            },
            Token {
                position: position2,
                separator: false,
                text: String::from(text2),
            },
            Token {
                position: 3,
                separator: true,
                text: String::from(" "),
            },
            Token {
                position: position4,
                separator: false,
                text: String::from(text4),
            },
            Token {
                position: 5,
                separator: true,
                text: String::from(" "),
            },
            Token {
                position: position6,
                separator: false,
                text: String::from(text6),
            },
            Token {
                position: 7,
                separator: true,
                text: String::from(" "),
            },
            Token {
                position: position8,
                separator: false,
                text: String::from(text8),
            },
        ];

        // exercise
        let text_terms = Term::from(&tokens, &Class::Text);

        // verify
        assert_eq!(5, text_terms.len());
        assert_eq!(
            &Term {
                position: position0,
                value: Value::new(&String::from(text0), &Class::Text).unwrap(),
            },
            text_terms.get(0).unwrap()
        );
        assert_eq!(
            &Term {
                position: position2,
                value: Value::new(&String::from(text2), &Class::Text).unwrap(),
            },
            text_terms.get(1).unwrap()
        );
        assert_eq!(
            &Term {
                position: position4,
                value: Value::new(&String::from(text4), &Class::Text).unwrap(),
            },
            text_terms.get(2).unwrap()
        );
        assert_eq!(
            &Term {
                position: position6,
                value: Value::new(&String::from(text6), &Class::Text).unwrap(),
            },
            text_terms.get(3).unwrap()
        );
        assert_eq!(
            &Term {
                position: position8,
                value: Value::new(&String::from(text8), &Class::Text).unwrap(),
            },
            text_terms.get(4).unwrap()
        );
    }

    #[test]
    fn to_integer() {
        // setup
        let position2 = 2;
        let text2 = "8";
        let tokens = vec![
            Token {
                position: 0,
                separator: false,
                text: String::from("integer"),
            },
            Token {
                position: 1,
                separator: true,
                text: String::from(" "),
            },
            Token {
                position: position2,
                separator: false,
                text: String::from(text2),
            },
            Token {
                position: 3,
                separator: true,
                text: String::from(" "),
            },
            Token {
                position: 4,
                separator: false,
                text: String::from("and"),
            },
            Token {
                position: 5,
                separator: true,
                text: String::from(" "),
            },
            Token {
                position: 6,
                separator: false,
                text: String::from("float"),
            },
            Token {
                position: 7,
                separator: true,
                text: String::from(" "),
            },
            Token {
                position: 8,
                separator: false,
                text: String::from("5.5"),
            },
        ];

        // exercise
        let integer_terms = Term::from(&tokens, &Class::Integer);

        // verify
        assert_eq!(1, integer_terms.len());
        assert_eq!(
            &Term {
                position: position2,
                value: Value::new(&String::from(text2), &Class::Integer).unwrap(),
            },
            integer_terms.get(0).unwrap()
        );
    }

    #[test]
    fn to_float() {
        // setup
        let position2 = 2;
        let position8 = 8;
        let text2 = "8";
        let text8 = "5.5";
        let tokens = vec![
            Token {
                position: 0,
                separator: false,
                text: String::from("integer"),
            },
            Token {
                position: 1,
                separator: true,
                text: String::from(" "),
            },
            Token {
                position: position2,
                separator: false,
                text: String::from(text2),
            },
            Token {
                position: 3,
                separator: true,
                text: String::from(" "),
            },
            Token {
                position: 4,
                separator: false,
                text: String::from("and"),
            },
            Token {
                position: 5,
                separator: true,
                text: String::from(" "),
            },
            Token {
                position: 6,
                separator: false,
                text: String::from("float"),
            },
            Token {
                position: 7,
                separator: true,
                text: String::from(" "),
            },
            Token {
                position: position8,
                separator: false,
                text: String::from(text8),
            },
        ];

        // exercise
        let float_terms = Term::from(&tokens, &Class::Float);

        // verify
        assert_eq!(2, float_terms.len());
        assert_eq!(
            &Term {
                position: position2,
                value: Value::new(&String::from(text2), &Class::Float).unwrap(),
            },
            float_terms.get(0).unwrap()
        );
        assert_eq!(
            &Term {
                position: position8,
                value: Value::new(&String::from(text8), &Class::Float).unwrap(),
            },
            float_terms.get(1).unwrap()
        );
    }
}
