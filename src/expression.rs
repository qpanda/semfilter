extern crate peg;

use crate::parser::Class;
use crate::parser::Term;
use crate::parser::Value;
use crate::tokenizer::Position;
use crate::tokenizer::Token;

peg::parser!(grammar expression() for str {
    // TODO return matching positions as set
    // TODO probably don't need test, can make evaluate pub
    pub rule evaluate(tokens: &Vec<Token>) -> Vec<Position>
        = condition(tokens)

    // TODO return matching positions as set
    rule condition(tokens: &Vec<Token>) -> Vec<Position>
        = integer_condition(tokens)
        / float_condition(tokens)
        / text_condition(tokens)

    //
    // conditions
    //
    rule integer_condition(tokens: &Vec<Token>) -> Vec<Position>
    = integers:integers(tokens) " == " integer:integer() { matching_positions(&integers, |term| term.value == integer) }
    / integers:integers(tokens) " != " integer:integer() { matching_positions(&integers, |term| term.value != integer) }
    / integers:integers(tokens) " > " integer:integer() { matching_positions(&integers, |term| term.value > integer) }
    / integers:integers(tokens) " >= " integer:integer() { matching_positions(&integers, |term| term.value >= integer) }
    / integers:integers(tokens) " < " integer:integer() { matching_positions(&integers, |term| term.value < integer) }
    / integers:integers(tokens) " <= " integer:integer() { matching_positions(&integers, |term| term.value <= integer) }

    rule float_condition(tokens: &Vec<Token>) -> Vec<Position>
    = floats:floats(tokens) " == " float:float() { matching_positions(&floats, |term| term.value == float) }
    / floats:floats(tokens) " != " float:float() { matching_positions(&floats, |term| term.value != float) }
    / floats:floats(tokens) " > " float:float() { matching_positions(&floats, |term| term.value > float) }
    / floats:floats(tokens) " >= " float:float() { matching_positions(&floats, |term| term.value >= float) }
    / floats:floats(tokens) " < " float:float() { matching_positions(&floats, |term| term.value < float) }
    / floats:floats(tokens) " <= " float:float() { matching_positions(&floats, |term| term.value <= float) }

    rule text_condition(tokens: &Vec<Token>) -> Vec<Position>
    = texts:texts(tokens) " == " text:text() { matching_positions(&texts, |term| term.value == text) }
    / texts:texts(tokens) " != " text:text() { matching_positions(&texts, |term| term.value != text) }
    / texts:texts(tokens) " > " text:text() { matching_positions(&texts, |term| term.value > text) }
    / texts:texts(tokens) " >= " text:text() { matching_positions(&texts, |term| term.value >= text) }
    / texts:texts(tokens) " < " text:text() { matching_positions(&texts, |term| term.value < text) }
    / texts:texts(tokens) " <= " text:text() { matching_positions(&texts, |term| term.value <= text) }

    //
    // terms
    //
    rule integers(tokens: &Vec<Token>) -> Vec<Term>
        = "integer" { Term::from(tokens, &Class::Integer) }

    rule floats(tokens: &Vec<Token>) -> Vec<Term>
        = "float" { Term::from(tokens, &Class::Float) }

    rule texts(tokens: &Vec<Token>) -> Vec<Term>
        = "text" { Term::from(tokens, &Class::Text) }

    //
    // values
    //
    // TODO check internet for correct pattern for integers
    rule integer() -> Value
        = n:$(['0'..='9']+) {?
            Value::new(&String::from(n), &Class::Integer).map_err(|error| "failed to parse integer")
        }

    // TODO check internet for correct pattern for floats
    rule float() -> Value
        = n:$(['0'..='9']+"."['0'..='9']+) {?
            Value::new(&String::from(n), &Class::Float).map_err(|error| "failed to parse float")
        }

    // TODO is identifier, do we want general text?
    rule text() -> Value
        = n:$(['a'..='z'|'A'..='Z']['a'..='z'|'A'..='Z'|'0'..='9']+) {?
            Value::new(&String::from(n), &Class::Text).map_err(|error| "failed to parse text")
        }
});

fn matching_positions<P>(terms: &Vec<Term>, predicate: P) -> Vec<Position>
where
    P: FnMut(&&Term) -> bool,
{
    terms
        .into_iter()
        .filter(predicate)
        .map(|term| term.position)
        .collect::<Vec<Position>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matching_integer_positions() {
        // setup
        let integers = vec![
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
        let integers_eq_integer_0 = matching_positions(&integers, |term| term.value == Value::Integer(0));
        let integers_eq_integer_2 = matching_positions(&integers, |term| term.value == Value::Integer(2));
        let integeres_ne_integer_0 = matching_positions(&integers, |term| term.value != Value::Integer(0));
        let integers_ne_integer_2 = matching_positions(&integers, |term| term.value != Value::Integer(2));
        let integers_gt_integer_0 = matching_positions(&integers, |term| term.value > Value::Integer(0));
        let integers_lt_integer_0 = matching_positions(&integers, |term| term.value < Value::Integer(0));
        let integers_eq_float_2 = matching_positions(&integers, |term| term.value == Value::Float(2.0));
        let integers_gt_float_0 = matching_positions(&integers, |term| term.value > Value::Float(0.0));
        let integers_eq_text_2 = matching_positions(&integers, |term| term.value > Value::Text(String::from("2")));

        // verify
        assert_eq!(0, integers_eq_integer_0.len());
        assert_eq!(vec![4], integers_eq_integer_2);
        assert_eq!(vec![2, 4, 6], integeres_ne_integer_0);
        assert_eq!(vec![2, 6], integers_ne_integer_2);
        assert_eq!(vec![2, 4, 6], integers_gt_integer_0);
        assert_eq!(0, integers_lt_integer_0.len());
        assert_eq!(0, integers_eq_float_2.len());
        assert_eq!(0, integers_gt_float_0.len());
        assert_eq!(0, integers_eq_text_2.len());
    }

    #[test]
    fn invalid_integer_conditions() {
        assert!(expression::evaluate("integer + 9", &vec![]).is_err());
        assert!(expression::evaluate("wrong == 9", &vec![]).is_err());
        assert!(expression::evaluate("integer == a", &vec![]).is_err());
    }

    #[test]
    fn valid_integer_conditions() {
        assert!(expression::evaluate("integer == 9", &vec![]).is_ok());
        assert!(expression::evaluate("integer != 9", &vec![]).is_ok());
        assert!(expression::evaluate("integer > 9", &vec![]).is_ok());
        assert!(expression::evaluate("integer >= 9", &vec![]).is_ok());
        assert!(expression::evaluate("integer < 9", &vec![]).is_ok());
        assert!(expression::evaluate("integer <= 9", &vec![]).is_ok());
    }

    #[test]
    fn evaluate_expression_no_tokens() {
        assert_eq!(expression::evaluate("integer == 9", &vec![]), Ok(vec![]));
        assert_eq!(expression::evaluate("integer != 9", &vec![]), Ok(vec![]));
        assert_eq!(expression::evaluate("float > 1.0", &vec![]), Ok(vec![]));
    }

    #[test]
    fn evaluate_expression_with_tokens() {
        // setup
        let tokens = vec![
            Token {
                position: 0,
                separator: false,
                text: String::from("a1"),
            },
            Token {
                position: 1,
                separator: false,
                text: String::from("9"),
            },
            Token {
                position: 2,
                separator: false,
                text: String::from("5.5"),
            },
        ];

        assert_eq!(expression::evaluate("integer == 9", &tokens), Ok(vec![1]));
        assert_eq!(expression::evaluate("integer != 9", &tokens), Ok(vec![]));
        assert_eq!(expression::evaluate("float == 5.5", &tokens), Ok(vec![2]));
        assert_eq!(expression::evaluate("float != 5.5", &tokens), Ok(vec![1]));
        assert_eq!(expression::evaluate("float > 0.0", &tokens), Ok(vec![1, 2]));
        assert_eq!(expression::evaluate("float < 0.0", &tokens), Ok(vec![]));
        assert_eq!(expression::evaluate("text == a1", &tokens), Ok(vec![0]));
        assert_eq!(expression::evaluate("text != a1", &tokens), Ok(vec![1, 2]));
        assert_eq!(expression::evaluate("text == b1", &tokens), Ok(vec![]));
    }
}
