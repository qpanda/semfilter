use anyhow::Error;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use semver::Version;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

use crate::tokenizer::Position;
use crate::tokenizer::Token;

pub type Id = String;

pub trait FromWord<F>: Sized {
    fn from_word(word: &str, format: &F) -> Result<Self, Error>;
}

impl FromWord<()> for i64 {
    fn from_word(word: &str, _: &()) -> Result<Self, Error> {
        match word.parse::<i64>() {
            Ok(integer) => Ok(integer),
            Err(error) => Err(error.into()),
        }
    }
}

impl FromWord<()> for f64 {
    fn from_word(word: &str, _: &()) -> Result<Self, Error> {
        match word.parse::<f64>() {
            Ok(float) => Ok(float),
            Err(error) => Err(error.into()),
        }
    }
}

impl FromWord<()> for Id {
    fn from_word(word: &str, _: &()) -> Result<Self, Error> {
        Ok(Id::from(word))
    }
}

impl FromWord<String> for NaiveDate {
    fn from_word(word: &str, format: &String) -> Result<Self, Error> {
        match NaiveDate::parse_from_str(word, format) {
            Ok(date) => Ok(date),
            Err(error) => Err(error.into()),
        }
    }
}

impl FromWord<String> for NaiveTime {
    fn from_word(word: &str, format: &String) -> Result<Self, Error> {
        match NaiveTime::parse_from_str(word, format) {
            Ok(time) => Ok(time),
            Err(error) => Err(error.into()),
        }
    }
}

impl FromWord<String> for DateTime<FixedOffset> {
    fn from_word(word: &str, format: &String) -> Result<Self, Error> {
        match DateTime::parse_from_str(word, format) {
            Ok(date_time) => Ok(date_time),
            Err(error) => Err(error.into()),
        }
    }
}

impl FromWord<String> for NaiveDateTime {
    fn from_word(word: &str, format: &String) -> Result<Self, Error> {
        match NaiveDateTime::parse_from_str(word, format) {
            Ok(local_date_time) => Ok(local_date_time),
            Err(error) => Err(error.into()),
        }
    }
}

impl FromWord<()> for Ipv4Addr {
    fn from_word(word: &str, _: &()) -> Result<Self, Error> {
        match word.parse::<Ipv4Addr>() {
            Ok(address) => Ok(address),
            Err(error) => Err(error.into()),
        }
    }
}

impl FromWord<()> for Ipv6Addr {
    fn from_word(word: &str, _: &()) -> Result<Self, Error> {
        match word.parse::<Ipv6Addr>() {
            Ok(address) => Ok(address),
            Err(error) => Err(error.into()),
        }
    }
}

impl FromWord<()> for SocketAddrV4 {
    fn from_word(word: &str, _: &()) -> Result<Self, Error> {
        match word.parse::<SocketAddrV4>() {
            Ok(address) => Ok(address),
            Err(error) => Err(error.into()),
        }
    }
}

impl FromWord<()> for SocketAddrV6 {
    fn from_word(word: &str, _: &()) -> Result<Self, Error> {
        match word.parse::<SocketAddrV6>() {
            Ok(address) => Ok(address),
            Err(error) => Err(error.into()),
        }
    }
}

impl FromWord<()> for Version {
    fn from_word(word: &str, _: &()) -> Result<Self, Error> {
        match Version::parse(word) {
            Ok(version) => Ok(version),
            Err(error) => Err(error.into()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Term<T> {
    pub position: Position,
    pub value: T,
}

pub struct Parser<T, F>(T, F);

impl<T: FromWord<F>, F> Parser<T, F> {
    pub fn from_tokens(tokens: &Vec<Token>, format: &F) -> Vec<Term<T>> {
        let mut result = Vec::new();
        for token in tokens {
            if !token.separator {
                if let Ok(value) = T::from_word(token.word, format) {
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
        let ok = i64::from_word(&integer.to_string(), &());
        let err_1 = i64::from_word("5.5", &());
        let err_2 = i64::from_word("word", &());

        // verify
        assert_eq!(integer, ok.unwrap());
        assert_eq!(true, err_1.is_err());
        assert_eq!(true, err_2.is_err());
    }

    #[test]
    fn new_float() {
        // setup
        let float = 5.5;
        let integer = 8;

        // exercise
        let ok_1 = f64::from_word(&float.to_string(), &());
        let ok_2 = f64::from_word(&integer.to_string(), &());
        let err = f64::from_word("word", &());

        // verify
        assert_eq!(float, ok_1.unwrap());
        assert_eq!(integer as f64, ok_2.unwrap());
        assert_eq!(true, err.is_err());
    }

    #[test]
    fn new_id() {
        // setup
        let float = "5.5";
        let integer = "8";
        let id = "test";

        // exercise
        let ok_1 = Id::from_word(float, &());
        let ok_2 = Id::from_word(integer, &());
        let ok_3 = Id::from_word(id, &());

        // verify
        assert_eq!(Id::from(float), ok_1.unwrap());
        assert_eq!(Id::from(integer), ok_2.unwrap());
        assert_eq!(Id::from(id), ok_3.unwrap());
    }

    #[test]
    fn new_date() {
        // setup
        let format = "%F";
        let date = NaiveDate::from_ymd(2021, 01, 01);

        // exercise
        let ok_1 = NaiveDate::from_word(&date.format(format).to_string(), &String::from(format));
        let err_1 = NaiveDate::from_word("5.5", &String::from(format));
        let err_2 = NaiveDate::from_word("08/08/2021", &String::from(format));

        // verify
        assert_eq!(date, ok_1.unwrap());
        assert_eq!(true, err_1.is_err());
        assert_eq!(true, err_2.is_err());
    }

    #[test]
    fn new_time() {
        // setup
        let format = "%T";
        let time = NaiveTime::from_hms(15, 15, 15);

        // exercise
        let ok_1 = NaiveTime::from_word(&time.format(format).to_string(), &String::from(format));
        let err_1 = NaiveTime::from_word("5.5", &String::from(format));
        let err_2 = NaiveTime::from_word("15.15.15", &String::from(format));

        // verify
        assert_eq!(time, ok_1.unwrap());
        assert_eq!(true, err_1.is_err());
        assert_eq!(true, err_2.is_err());
    }

    #[test]
    fn new_date_time() {
        // setup
        let format = "%+";
        let date_time_string = "2001-07-08T00:34:60.026490+09:30";
        let date_time = DateTime::parse_from_str(date_time_string, format).unwrap();

        // exercise
        let ok_1 = DateTime::<FixedOffset>::from_word(&date_time_string, &String::from(format));
        let err_1 = DateTime::<FixedOffset>::from_word("5.5", &String::from(format));
        let err_2 = DateTime::<FixedOffset>::from_word("2001-07-08 00:34:60", &String::from(format));

        // verify
        assert_eq!(date_time, ok_1.unwrap());
        assert_eq!(true, err_1.is_err());
        assert_eq!(true, err_2.is_err());
    }

    #[test]
    fn new_local_date_time() {
        // setup
        let format = "%Y-%m-%dT%H:%M:%S%.f";
        let date_time_string = "2001-07-08T00:34:60.026490";
        let date_time = NaiveDateTime::parse_from_str(date_time_string, format).unwrap();

        // exercise
        let ok_1 = NaiveDateTime::from_word(&date_time_string, &String::from(format));
        let err_1 = NaiveDateTime::from_word("5.5", &String::from(format));
        let err_2 = NaiveDateTime::from_word("2001-07-08 00:34:60", &String::from(format));

        // verify
        assert_eq!(date_time, ok_1.unwrap());
        assert_eq!(true, err_1.is_err());
        assert_eq!(true, err_2.is_err());
    }

    #[test]
    fn new_ipv4_address() {
        // setup
        let address = "8.8.8.8".parse::<Ipv4Addr>().unwrap();

        // exercise
        let ok = Ipv4Addr::from_word(&address.to_string(), &());
        let err_1 = Ipv4Addr::from_word("5.5", &());
        let err_2 = Ipv4Addr::from_word("word", &());

        // verify
        assert_eq!(address, ok.unwrap());
        assert_eq!(true, err_1.is_err());
        assert_eq!(true, err_2.is_err());
    }

    #[test]
    fn new_ipv6_address() {
        // setup
        let address = "2001:4860:4860::8888".parse::<Ipv6Addr>().unwrap();

        // exercise
        let ok = Ipv6Addr::from_word(&address.to_string(), &());
        let err_1 = Ipv6Addr::from_word("2001:4860", &());
        let err_2 = Ipv6Addr::from_word("word", &());

        // verify
        assert_eq!(address, ok.unwrap());
        assert_eq!(true, err_1.is_err());
        assert_eq!(true, err_2.is_err());
    }

    #[test]
    fn new_ipv4_socket_address() {
        // setup
        let address = "8.8.8.8:53".parse::<SocketAddrV4>().unwrap();

        // exercise
        let ok = SocketAddrV4::from_word(&address.to_string(), &());
        let err_1 = SocketAddrV4::from_word("5.5.5.5", &());
        let err_2 = SocketAddrV4::from_word("word", &());

        // verify
        assert_eq!(address, ok.unwrap());
        assert_eq!(true, err_1.is_err());
        assert_eq!(true, err_2.is_err());
    }

    #[test]
    fn new_ipv6_socket_address() {
        // setup
        let address = "[2001:4860:4860::8888]:53".parse::<SocketAddrV6>().unwrap();

        // exercise
        let ok = SocketAddrV6::from_word(&address.to_string(), &());
        let err_1 = SocketAddrV6::from_word("2001:4860", &());
        let err_2 = SocketAddrV6::from_word("word", &());

        // verify
        assert_eq!(address, ok.unwrap());
        assert_eq!(true, err_1.is_err());
        assert_eq!(true, err_2.is_err());
    }

    #[test]
    fn new_semantic_version() {
        // setup
        let version = Version::parse("1.2.3").unwrap();

        // exercise
        let ok = Version::from_word(&version.to_string(), &());
        let err_1 = Version::from_word("1:2:3", &());
        let err_2 = Version::from_word("word", &());

        // verify
        assert_eq!(version, ok.unwrap());
        assert_eq!(true, err_1.is_err());
        assert_eq!(true, err_2.is_err());
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
        let id_terms = Parser::<Id, ()>::from_tokens(&tokens, &());
        let integer_terms = Parser::<i64, ()>::from_tokens(&tokens, &());
        let float_terms = Parser::<f64, ()>::from_tokens(&tokens, &());

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
        let id_terms = Parser::<Id, ()>::from_tokens(&tokens, &());
        let integer_terms = Parser::<i64, ()>::from_tokens(&tokens, &());
        let float_terms = Parser::<f64, ()>::from_tokens(&tokens, &());

        // verify
        assert_eq!(1, id_terms.len());
        assert_eq!(
            &Term {
                position: position,
                value: Id::from_word(word, &()).unwrap(),
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
        let id_terms = Parser::<Id, ()>::from_tokens(&tokens, &());
        let integer_terms = Parser::<i64, ()>::from_tokens(&tokens, &());
        let float_terms = Parser::<f64, ()>::from_tokens(&tokens, &());

        // verify
        assert_eq!(1, id_terms.len());
        assert_eq!(
            &Term {
                position: position,
                value: Id::from_word(word, &()).unwrap(),
            },
            id_terms.get(0).unwrap()
        );

        assert_eq!(1, integer_terms.len());
        assert_eq!(
            &Term {
                position: position,
                value: i64::from_word(word, &()).unwrap(),
            },
            integer_terms.get(0).unwrap()
        );

        assert_eq!(1, float_terms.len());
        assert_eq!(
            &Term {
                position: position,
                value: f64::from_word(word, &()).unwrap(),
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
        let id_terms = Parser::<Id, ()>::from_tokens(&tokens, &());
        let integer_terms = Parser::<i64, ()>::from_tokens(&tokens, &());
        let float_terms = Parser::<f64, ()>::from_tokens(&tokens, &());

        // verify
        assert_eq!(1, id_terms.len());
        assert_eq!(
            &Term {
                position: position,
                value: Id::from_word(word, &()).unwrap(),
            },
            id_terms.get(0).unwrap()
        );

        assert_eq!(0, integer_terms.len());

        assert_eq!(1, float_terms.len());
        assert_eq!(
            &Term {
                position: position,
                value: f64::from_word(word, &()).unwrap(),
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
        let id_terms = Parser::<Id, ()>::from_tokens(&tokens, &());

        // verify
        assert_eq!(5, id_terms.len());
        assert_eq!(
            &Term {
                position: position0,
                value: Id::from_word(word0, &()).unwrap(),
            },
            id_terms.get(0).unwrap()
        );
        assert_eq!(
            &Term {
                position: position2,
                value: Id::from_word(word2, &()).unwrap(),
            },
            id_terms.get(1).unwrap()
        );
        assert_eq!(
            &Term {
                position: position4,
                value: Id::from_word(word4, &()).unwrap(),
            },
            id_terms.get(2).unwrap()
        );
        assert_eq!(
            &Term {
                position: position6,
                value: Id::from_word(word6, &()).unwrap(),
            },
            id_terms.get(3).unwrap()
        );
        assert_eq!(
            &Term {
                position: position8,
                value: Id::from_word(word8, &()).unwrap(),
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
        let integer_terms = Parser::<i64, ()>::from_tokens(&tokens, &());

        // verify
        assert_eq!(1, integer_terms.len());
        assert_eq!(
            &Term {
                position: position2,
                value: i64::from_word(word2, &()).unwrap(),
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
        let float_terms = Parser::<f64, ()>::from_tokens(&tokens, &());

        // verify
        assert_eq!(2, float_terms.len());
        assert_eq!(
            &Term {
                position: position2,
                value: f64::from_word(word2, &()).unwrap(),
            },
            float_terms.get(0).unwrap()
        );
        assert_eq!(
            &Term {
                position: position8,
                value: f64::from_word(word8, &()).unwrap(),
            },
            float_terms.get(1).unwrap()
        );
    }
}
