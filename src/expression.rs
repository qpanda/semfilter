extern crate peg;

use std::collections::HashSet;

use crate::parser::Class;
use crate::parser::Term;
use crate::parser::Value;
use crate::tokenizer::Position;
use crate::tokenizer::Token;

peg::parser!(pub grammar expression() for str {
    pub rule evaluate(tokens: &Vec<Token>) -> HashSet<Position>
        = condition(tokens)

    rule condition(tokens: &Vec<Token>) -> HashSet<Position>
        = integer_condition(tokens)
        / float_condition(tokens)
        / id_condition(tokens)

    //
    // conditions
    //
    rule integer_condition(tokens: &Vec<Token>) -> HashSet<Position>
    = integers:integers(tokens) " == " integer:integer() { matches(&integers, |term| term.value == integer) }
    / integers:integers(tokens) " != " integer:integer() { matches(&integers, |term| term.value != integer) }
    / integers:integers(tokens) " > " integer:integer() { matches(&integers, |term| term.value > integer) }
    / integers:integers(tokens) " >= " integer:integer() { matches(&integers, |term| term.value >= integer) }
    / integers:integers(tokens) " < " integer:integer() { matches(&integers, |term| term.value < integer) }
    / integers:integers(tokens) " <= " integer:integer() { matches(&integers, |term| term.value <= integer) }

    rule float_condition(tokens: &Vec<Token>) -> HashSet<Position>
    = floats:floats(tokens) " == " float:float() { matches(&floats, |term| term.value == float) }
    / floats:floats(tokens) " != " float:float() { matches(&floats, |term| term.value != float) }
    / floats:floats(tokens) " > " float:float() { matches(&floats, |term| term.value > float) }
    / floats:floats(tokens) " >= " float:float() { matches(&floats, |term| term.value >= float) }
    / floats:floats(tokens) " < " float:float() { matches(&floats, |term| term.value < float) }
    / floats:floats(tokens) " <= " float:float() { matches(&floats, |term| term.value <= float) }

    rule id_condition(tokens: &Vec<Token>) -> HashSet<Position>
    = ids:ids(tokens) " == " id:id() { matches(&ids, |term| term.value == id) }
    / ids:ids(tokens) " != " id:id() { matches(&ids, |term| term.value != id) }
    / ids:ids(tokens) " > " id:id() { matches(&ids, |term| term.value > id) }
    / ids:ids(tokens) " >= " id:id() { matches(&ids, |term| term.value >= id) }
    / ids:ids(tokens) " < " id:id() { matches(&ids, |term| term.value < id) }
    / ids:ids(tokens) " <= " id:id() { matches(&ids, |term| term.value <= id) }

    //
    // terms
    //
    rule integers(tokens: &Vec<Token>) -> Vec<Term>
        = "$integer" { Term::from(tokens, &Class::Integer) }

    rule floats(tokens: &Vec<Token>) -> Vec<Term>
        = "$float" { Term::from(tokens, &Class::Float) }

    rule ids(tokens: &Vec<Token>) -> Vec<Term>
        = "$id" { Term::from(tokens, &Class::Id) }

    //
    // values
    //
    rule integer() -> Value
        = n:$(['+'|'-']? ['0'..='9']+) {?
            Value::from(&String::from(n), &Class::Integer).map_err(|_| "failed to parse integer")
        }

    rule float() -> Value
        = n:$(['+'|'-']? ['0'..='9']* ['.']? ['0'..='9']*) {?
            Value::from(&String::from(n), &Class::Float).map_err(|_| "failed to parse float")
        }

    rule id() -> Value
        = n:$(['a'..='z'|'A'..='Z'|'0'..='9'|'.'|':'|'_'|'-']+) {?
            Value::from(&String::from(n), &Class::Id).map_err(|_| "failed to parse id")
        }
});

fn matches<P>(terms: &Vec<Term>, predicate: P) -> HashSet<Position>
where
    P: FnMut(&&Term) -> bool,
{
    terms
        .into_iter()
        .filter(predicate)
        .map(|term| term.position)
        .collect::<HashSet<Position>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_matches() {
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
        let integers_eq_integer_0 = matches(&integers, |term| term.value == Value::Integer(0));
        let integers_eq_integer_2 = matches(&integers, |term| term.value == Value::Integer(2));
        let integeres_ne_integer_0 = matches(&integers, |term| term.value != Value::Integer(0));
        let integers_ne_integer_2 = matches(&integers, |term| term.value != Value::Integer(2));
        let integers_gt_integer_0 = matches(&integers, |term| term.value > Value::Integer(0));
        let integers_lt_integer_0 = matches(&integers, |term| term.value < Value::Integer(0));
        let integers_eq_float_2 = matches(&integers, |term| term.value == Value::Float(2.0));
        let integers_gt_float_0 = matches(&integers, |term| term.value > Value::Float(0.0));
        let integers_eq_id_2 = matches(&integers, |term| term.value > Value::Id(String::from("2")));

        // verify
        assert_eq!(HashSet::from([]), integers_eq_integer_0);
        assert_eq!(HashSet::from([4]), integers_eq_integer_2);
        assert_eq!(HashSet::from([2, 4, 6]), integeres_ne_integer_0);
        assert_eq!(HashSet::from([2, 6]), integers_ne_integer_2);
        assert_eq!(HashSet::from([2, 4, 6]), integers_gt_integer_0);
        assert_eq!(HashSet::from([]), integers_lt_integer_0);
        assert_eq!(HashSet::from([]), integers_eq_float_2);
        assert_eq!(HashSet::from([]), integers_gt_float_0);
        assert_eq!(HashSet::from([]), integers_eq_id_2);
    }

    #[test]
    fn invalid_integer_conditions() {
        assert!(expression::evaluate("$integer + 9", &vec![]).is_err());
        assert!(expression::evaluate("wrong == 9", &vec![]).is_err());
        assert!(expression::evaluate("$integer == a", &vec![]).is_err());
    }

    #[test]
    fn valid_integer_conditions() {
        assert!(expression::evaluate("$integer == 9", &vec![]).is_ok());
        assert!(expression::evaluate("$integer != 9", &vec![]).is_ok());
        assert!(expression::evaluate("$integer > 9", &vec![]).is_ok());
        assert!(expression::evaluate("$integer >= 9", &vec![]).is_ok());
        assert!(expression::evaluate("$integer < 9", &vec![]).is_ok());
        assert!(expression::evaluate("$integer <= 9", &vec![]).is_ok());
    }

    #[test]
    fn evaluate_expression_no_tokens() {
        assert_eq!(expression::evaluate("$integer == 9", &vec![]), Ok(HashSet::new()));
        assert_eq!(expression::evaluate("$integer != 9", &vec![]), Ok(HashSet::new()));
        assert_eq!(expression::evaluate("$float > 1.0", &vec![]), Ok(HashSet::new()));
    }

    #[test]
    fn evaluate_expression_with_tokens() {
        // setup
        let tokens = vec![
            Token {
                position: 0,
                separator: false,
                word: "a1",
            },
            Token {
                position: 1,
                separator: false,
                word: "9",
            },
            Token {
                position: 2,
                separator: false,
                word: "5.5",
            },
        ];

        assert_eq!(expression::evaluate("$integer == 9", &tokens), Ok(HashSet::from([1])));
        assert_eq!(expression::evaluate("$integer != 9", &tokens), Ok(HashSet::from([])));
        assert_eq!(expression::evaluate("$float == 5.5", &tokens), Ok(HashSet::from([2])));
        assert_eq!(expression::evaluate("$float != 5.5", &tokens), Ok(HashSet::from([1])));
        assert_eq!(expression::evaluate("$float > 0.0", &tokens), Ok(HashSet::from([1, 2])));
        assert_eq!(expression::evaluate("$float < 0.0", &tokens), Ok(HashSet::from([])));
        assert_eq!(expression::evaluate("$id == a1", &tokens), Ok(HashSet::from([0])));
        assert_eq!(expression::evaluate("$id != a1", &tokens), Ok(HashSet::from([1, 2])));
        assert_eq!(expression::evaluate("$id == b1", &tokens), Ok(HashSet::from([])));
    }
}
