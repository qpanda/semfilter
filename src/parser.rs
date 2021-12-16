use anyhow::Error;

use crate::tokenizer::Position;
use crate::tokenizer::Token;

pub enum Class {
    Integer,
    Float,
    Id,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Id(String),
}

#[derive(Debug, PartialEq)]
pub struct Term {
    pub position: Position,
    pub value: Value,
}

impl Value {
    pub fn from(word: &str, class: &Class) -> Result<Self, Error> {
        match class {
            Class::Integer => match word.parse::<i64>() {
                Ok(integer) => Ok(Value::Integer(integer)),
                Err(error) => Err(error.into()),
            },
            Class::Float => match word.parse::<f64>() {
                Ok(float) => Ok(Value::Float(float)),
                Err(error) => Err(error.into()),
            },
            Class::Id => Ok(Value::Id(String::from(word))),
        }
    }
}

impl Term {
    pub fn from(tokens: &Vec<Token>, class: &Class) -> Vec<Term> {
        let mut result = Vec::new();
        for token in tokens {
            if !token.separator {
                if let Ok(value) = Value::from(token.word, class) {
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

        // exercise
        let ok = Value::from(&integer.to_string(), &Class::Integer);
        let err_1 = Value::from("5.5", &Class::Integer);
        let err_2 = Value::from("word", &Class::Integer);

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

        // exercise
        let ok_1 = Value::from(&float.to_string(), &Class::Float);
        let ok_2 = Value::from(&integer.to_string(), &Class::Float);
        let err = Value::from("word", &Class::Float);

        // verify
        assert_eq!(Value::Float(float), ok_1.unwrap());
        assert_eq!(Value::Float(integer as f64), ok_2.unwrap());
        assert_eq!(true, err.is_err());
    }

    #[test]
    fn new_id() {
        // setup
        let float = "5.5";
        let integer = "8";
        let id = "test";

        // exercise
        let ok_1 = Value::from(float, &Class::Id);
        let ok_2 = Value::from(integer, &Class::Id);
        let ok_3 = Value::from(id, &Class::Id);

        // verify
        assert_eq!(Value::Id(String::from(float)), ok_1.unwrap());
        assert_eq!(Value::Id(String::from(integer)), ok_2.unwrap());
        assert_eq!(Value::Id(String::from(id)), ok_3.unwrap());
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
            word: " ",
        };
        let tokens = vec![separator_token];

        // exercise
        let id_terms = Term::from(&tokens, &Class::Id);
        let integer_terms = Term::from(&tokens, &Class::Integer);
        let float_terms = Term::from(&tokens, &Class::Float);

        // verify
        assert_eq!(0, id_terms.len());
        assert_eq!(0, integer_terms.len());
        assert_eq!(0, float_terms.len());
    }

    #[test]
    fn from_text() {
        // setup
        let word = "text";
        let position = 0;
        let text_token = Token {
            position: position,
            separator: false,
            word: word,
        };
        let tokens = vec![text_token];

        // exercise
        let id_terms = Term::from(&tokens, &Class::Id);
        let integer_terms = Term::from(&tokens, &Class::Integer);
        let float_terms = Term::from(&tokens, &Class::Float);

        // verify
        assert_eq!(1, id_terms.len());
        assert_eq!(
            &Term {
                position: position,
                value: Value::from(word, &Class::Id).unwrap(),
            },
            id_terms.get(0).unwrap()
        );

        assert_eq!(0, integer_terms.len());

        assert_eq!(0, float_terms.len());
    }

    #[test]
    fn from_integer() {
        // setup
        let word = "8";
        let position = 0;
        let integer_token = Token {
            position: position,
            separator: false,
            word: word,
        };
        let tokens = vec![integer_token];

        // exercise
        let id_terms = Term::from(&tokens, &Class::Id);
        let integer_terms = Term::from(&tokens, &Class::Integer);
        let float_terms = Term::from(&tokens, &Class::Float);

        // verify
        assert_eq!(1, id_terms.len());
        assert_eq!(
            &Term {
                position: position,
                value: Value::from(word, &Class::Id).unwrap(),
            },
            id_terms.get(0).unwrap()
        );

        assert_eq!(1, integer_terms.len());
        assert_eq!(
            &Term {
                position: position,
                value: Value::from(word, &Class::Integer).unwrap(),
            },
            integer_terms.get(0).unwrap()
        );

        assert_eq!(1, float_terms.len());
        assert_eq!(
            &Term {
                position: position,
                value: Value::from(word, &Class::Float).unwrap(),
            },
            float_terms.get(0).unwrap()
        );
    }

    #[test]
    fn from_float() {
        // setup
        let word = "5.5";
        let position = 0;
        let float_token = Token {
            position: position,
            separator: false,
            word: word,
        };
        let tokens = vec![float_token];

        // exercise
        let id_terms = Term::from(&tokens, &Class::Id);
        let integer_terms = Term::from(&tokens, &Class::Integer);
        let float_terms = Term::from(&tokens, &Class::Float);

        // verify
        assert_eq!(1, id_terms.len());
        assert_eq!(
            &Term {
                position: position,
                value: Value::from(word, &Class::Id).unwrap(),
            },
            id_terms.get(0).unwrap()
        );

        assert_eq!(0, integer_terms.len());

        assert_eq!(1, float_terms.len());
        assert_eq!(
            &Term {
                position: position,
                value: Value::from(word, &Class::Float).unwrap(),
            },
            float_terms.get(0).unwrap()
        );
    }

    #[test]
    fn to_id() {
        // setup
        let position0 = 0;
        let position2 = 2;
        let position4 = 4;
        let position6 = 6;
        let position8 = 8;
        let word0 = "integer";
        let word2 = "8";
        let word4 = "and";
        let word6 = "float";
        let word8 = "5.5";
        let tokens = vec![
            Token {
                position: position0,
                separator: false,
                word: word0,
            },
            Token {
                position: 1,
                separator: true,
                word: " ",
            },
            Token {
                position: position2,
                separator: false,
                word: word2,
            },
            Token {
                position: 3,
                separator: true,
                word: " ",
            },
            Token {
                position: position4,
                separator: false,
                word: word4,
            },
            Token {
                position: 5,
                separator: true,
                word: " ",
            },
            Token {
                position: position6,
                separator: false,
                word: word6,
            },
            Token {
                position: 7,
                separator: true,
                word: " ",
            },
            Token {
                position: position8,
                separator: false,
                word: word8,
            },
        ];

        // exercise
        let id_terms = Term::from(&tokens, &Class::Id);

        // verify
        assert_eq!(5, id_terms.len());
        assert_eq!(
            &Term {
                position: position0,
                value: Value::from(word0, &Class::Id).unwrap(),
            },
            id_terms.get(0).unwrap()
        );
        assert_eq!(
            &Term {
                position: position2,
                value: Value::from(word2, &Class::Id).unwrap(),
            },
            id_terms.get(1).unwrap()
        );
        assert_eq!(
            &Term {
                position: position4,
                value: Value::from(word4, &Class::Id).unwrap(),
            },
            id_terms.get(2).unwrap()
        );
        assert_eq!(
            &Term {
                position: position6,
                value: Value::from(word6, &Class::Id).unwrap(),
            },
            id_terms.get(3).unwrap()
        );
        assert_eq!(
            &Term {
                position: position8,
                value: Value::from(word8, &Class::Id).unwrap(),
            },
            id_terms.get(4).unwrap()
        );
    }

    #[test]
    fn to_integer() {
        // setup
        let position2 = 2;
        let word2 = "8";
        let tokens = vec![
            Token {
                position: 0,
                separator: false,
                word: "integer",
            },
            Token {
                position: 1,
                separator: true,
                word: " ",
            },
            Token {
                position: position2,
                separator: false,
                word: word2,
            },
            Token {
                position: 3,
                separator: true,
                word: " ",
            },
            Token {
                position: 4,
                separator: false,
                word: "and",
            },
            Token {
                position: 5,
                separator: true,
                word: " ",
            },
            Token {
                position: 6,
                separator: false,
                word: "float",
            },
            Token {
                position: 7,
                separator: true,
                word: " ",
            },
            Token {
                position: 8,
                separator: false,
                word: "5.5",
            },
        ];

        // exercise
        let integer_terms = Term::from(&tokens, &Class::Integer);

        // verify
        assert_eq!(1, integer_terms.len());
        assert_eq!(
            &Term {
                position: position2,
                value: Value::from(word2, &Class::Integer).unwrap(),
            },
            integer_terms.get(0).unwrap()
        );
    }

    #[test]
    fn to_float() {
        // setup
        let position2 = 2;
        let position8 = 8;
        let word2 = "8";
        let word8 = "5.5";
        let tokens = vec![
            Token {
                position: 0,
                separator: false,
                word: "integer",
            },
            Token {
                position: 1,
                separator: true,
                word: " ",
            },
            Token {
                position: position2,
                separator: false,
                word: word2,
            },
            Token {
                position: 3,
                separator: true,
                word: " ",
            },
            Token {
                position: 4,
                separator: false,
                word: "and",
            },
            Token {
                position: 5,
                separator: true,
                word: " ",
            },
            Token {
                position: 6,
                separator: false,
                word: "float",
            },
            Token {
                position: 7,
                separator: true,
                word: " ",
            },
            Token {
                position: position8,
                separator: false,
                word: word8,
            },
        ];

        // exercise
        let float_terms = Term::from(&tokens, &Class::Float);

        // verify
        assert_eq!(2, float_terms.len());
        assert_eq!(
            &Term {
                position: position2,
                value: Value::from(word2, &Class::Float).unwrap(),
            },
            float_terms.get(0).unwrap()
        );
        assert_eq!(
            &Term {
                position: position8,
                value: Value::from(word8, &Class::Float).unwrap(),
            },
            float_terms.get(1).unwrap()
        );
    }
}
