extern crate peg;

use crate::parser::Class;
use crate::parser::Term;
use crate::parser::Value;
use crate::tokenizer::Position;
use crate::tokenizer::Token;

peg::parser!(grammar expression() for str {
    // TODO return matching positions as set
    pub rule test(tokens: &Vec<Token>) -> bool
        = evaluate(tokens)

    // TODO return matching positions as set
    rule evaluate(tokens: &Vec<Token>) -> bool
        = class:class() " == " text:text() {?
            let value = Value::new(&text, &class).map_err(|error| "failed to convert")?;
            let terms = Term::from(tokens, &class);
            let positions = Evaluator::positions(&terms, |term| term.value == value);
            return Ok(positions.len() != 0);
        }
        / class:class() " != " text:text() {?
            let value = Value::new(&text, &class).map_err(|error| "failed to convert")?;
            let terms = Term::from(tokens, &class);
            let positions = Evaluator::positions(&terms, |term| term.value != value);
            return Ok(positions.len() != 0);
        }

    rule class() -> Class
        = "integer" { return Class::Integer }
        / "float" { return Class::Float }
        / "text" { return Class::Text }

    rule text() -> String
        // TODO generalize expression (everything between separators as per Tokenizer)
        = (n:$(['a'..='z' | '0'..='9']+) {
            return String::from(n);
        })
});

struct Evaluator {}

impl Evaluator {
    fn positions<P>(terms: &Vec<Term>, predicate: P) -> Vec<Position>
    where
        P: FnMut(&&Term) -> bool,
    {
        return terms
            .into_iter()
            .filter(predicate)
            .map(|term| term.position)
            .collect::<Vec<Position>>();
    }
}

#[cfg(test)]
mod tests_expression {
    use super::*;

    #[test]
    fn ttt() {
        let tokens = vec![Token {
            position: 0,
            separator: false,
            text: String::from("9"),
        }];
        assert_eq!(expression::test("integer == 9", &tokens), Ok(true));
        assert_eq!(expression::test("integer != 9", &tokens), Ok(false));
    }
}

#[cfg(test)]
mod tests_evaluator {
    use super::*;

    #[test]
    fn positions() {
        // setup
        let integer_terms = vec![
            Term {
                position: 2,
                value: Value::Integer(1),
            },
            Term {
                position: 4,
                value: Value::Integer(2),
            },
            Term {
                position: 6,
                value: Value::Integer(3),
            },
        ];

        // exercise
        let positions_integer_eq_0 = Evaluator::positions(&integer_terms, |term| term.value == Value::Integer(0));
        let positions_integer_eq_2 = Evaluator::positions(&integer_terms, |term| term.value == Value::Integer(2));
        let positions_integer_ne_0 = Evaluator::positions(&integer_terms, |term| term.value != Value::Integer(0));
        let positions_ne_integer_2 = Evaluator::positions(&integer_terms, |term| term.value != Value::Integer(2));
        let positions_integer_gt_0 = Evaluator::positions(&integer_terms, |term| term.value > Value::Integer(0));
        let positions_integer_st_0 = Evaluator::positions(&integer_terms, |term| term.value < Value::Integer(0));
        let positions_float_eq_2 = Evaluator::positions(&integer_terms, |term| term.value == Value::Float(2.0));
        let positions_float_gt_0 = Evaluator::positions(&integer_terms, |term| term.value > Value::Float(0.0));

        // verify
        assert_eq!(0, positions_integer_eq_0.len());
        assert_eq!(vec![4], positions_integer_eq_2);
        assert_eq!(vec![2, 4, 6], positions_integer_ne_0);
        assert_eq!(vec![2, 6], positions_ne_integer_2);
        assert_eq!(vec![2, 4, 6], positions_integer_gt_0);
        assert_eq!(0, positions_integer_st_0.len());
        assert_eq!(0, positions_float_eq_2.len());
        assert_eq!(0, positions_float_gt_0.len());
    }
}
