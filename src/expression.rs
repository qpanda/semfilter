extern crate peg;

use std::collections::HashSet;

use crate::filter::Formats;
use crate::parser::Class;
use crate::parser::Term;
use crate::parser::Value;
use crate::tokenizer::Position;
use crate::tokenizer::Token;

peg::parser!(pub grammar expression() for str {
    pub rule evaluate(tokens: &Vec<Token>, formats: &Formats) -> HashSet<Position>
        = condition(tokens, formats)

    rule condition(tokens: &Vec<Token>, formats: &Formats) -> HashSet<Position>
        = integer_condition(tokens)
        / float_condition(tokens)
        / id_condition(tokens)
        / date_condition(tokens, formats)
        / time_condition(tokens, formats)
        / date_time_condition(tokens, formats)
        / local_date_time_condition(tokens, formats)

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

    rule date_condition(tokens: &Vec<Token>, formats: &Formats) -> HashSet<Position>
    = dates:dates(tokens, formats) " == " date:date(formats) { matches(&dates, |term| term.value == date) }
    / dates:dates(tokens, formats) " != " date:date(formats) { matches(&dates, |term| term.value != date) }
    / dates:dates(tokens, formats) " > " date:date(formats) { matches(&dates, |term| term.value > date) }
    / dates:dates(tokens, formats) " >= " date:date(formats) { matches(&dates, |term| term.value >= date) }
    / dates:dates(tokens, formats) " < " date:date(formats) { matches(&dates, |term| term.value < date) }
    / dates:dates(tokens, formats) " <= " date:date(formats) { matches(&dates, |term| term.value <= date) }

    rule time_condition(tokens: &Vec<Token>, formats: &Formats) -> HashSet<Position>
    = times:times(tokens, formats) " == " time:time(formats) { matches(&times, |term| term.value == time) }
    / times:times(tokens, formats) " != " time:time(formats) { matches(&times, |term| term.value != time) }
    / times:times(tokens, formats) " > " time:time(formats) { matches(&times, |term| term.value > time) }
    / times:times(tokens, formats) " >= " time:time(formats) { matches(&times, |term| term.value >= time) }
    / times:times(tokens, formats) " < " time:time(formats) { matches(&times, |term| term.value < time) }
    / times:times(tokens, formats) " <= " time:time(formats) { matches(&times, |term| term.value <= time) }

    rule date_time_condition(tokens: &Vec<Token>, formats: &Formats) -> HashSet<Position>
    = date_times:date_times(tokens, formats) " == " date_time:date_time(formats) { matches(&date_times, |term| term.value == date_time) }
    / date_times:date_times(tokens, formats) " != " date_time:date_time(formats) { matches(&date_times, |term| term.value != date_time) }
    / date_times:date_times(tokens, formats) " > " date_time:date_time(formats) { matches(&date_times, |term| term.value > date_time) }
    / date_times:date_times(tokens, formats) " >= " date_time:date_time(formats) { matches(&date_times, |term| term.value >= date_time) }
    / date_times:date_times(tokens, formats) " < " date_time:date_time(formats) { matches(&date_times, |term| term.value < date_time) }
    / date_times:date_times(tokens, formats) " <= " date_time:date_time(formats) { matches(&date_times, |term| term.value <= date_time) }

    rule local_date_time_condition(tokens: &Vec<Token>, formats: &Formats) -> HashSet<Position>
    = local_date_times:local_date_times(tokens, formats) " == " local_date_time:local_date_time(formats) { matches(&local_date_times, |term| term.value == local_date_time) }
    / local_date_times:local_date_times(tokens, formats) " != " local_date_time:local_date_time(formats) { matches(&local_date_times, |term| term.value != local_date_time) }
    / local_date_times:local_date_times(tokens, formats) " > " local_date_time:local_date_time(formats) { matches(&local_date_times, |term| term.value > local_date_time) }
    / local_date_times:local_date_times(tokens, formats) " >= " local_date_time:local_date_time(formats) { matches(&local_date_times, |term| term.value >= local_date_time) }
    / local_date_times:local_date_times(tokens, formats) " < " local_date_time:local_date_time(formats) { matches(&local_date_times, |term| term.value < local_date_time) }
    / local_date_times:local_date_times(tokens, formats) " <= " local_date_time:local_date_time(formats) { matches(&local_date_times, |term| term.value <= local_date_time) }

    //
    // terms
    //
    rule integers(tokens: &Vec<Token>) -> Vec<Term>
        = "$integer" { Term::from(tokens, &Class::Integer) }

    rule floats(tokens: &Vec<Token>) -> Vec<Term>
        = "$float" { Term::from(tokens, &Class::Float) }

    rule ids(tokens: &Vec<Token>) -> Vec<Term>
        = "$id" { Term::from(tokens, &Class::Id) }

    rule dates(tokens: &Vec<Token>, formats: &Formats) -> Vec<Term>
        = "$date" { Term::from(tokens, &Class::Date(formats.date.to_string())) }

    rule times(tokens: &Vec<Token>, formats: &Formats) -> Vec<Term>
        = "$time" { Term::from(tokens, &Class::Time(formats.time.to_string())) }

    rule date_times(tokens: &Vec<Token>, formats: &Formats) -> Vec<Term>
        = "$dateTime" { Term::from(tokens, &Class::DateTime(formats.date_time.to_string())) }

    rule local_date_times(tokens: &Vec<Token>, formats: &Formats) -> Vec<Term>
        = "$localDateTime" { Term::from(tokens, &Class::LocalDateTime(formats.local_date_time.to_string())) }

    //
    // values
    //
    rule integer() -> Value
        = n:$(['+'|'-']? ['0'..='9']+) {?
            Value::from(n, &Class::Integer).map_err(|_| "failed to parse integer")
        }

    rule float() -> Value
        = n:$(['+'|'-']? ['0'..='9']* ['.']? ['0'..='9']*) {?
            Value::from(n, &Class::Float).map_err(|_| "failed to parse float")
        }

    rule id() -> Value
        = n:$(['a'..='z'|'A'..='Z'|'0'..='9'|'.'|':'|'_'|'-']+) {?
            Value::from(n, &Class::Id).map_err(|_| "failed to parse id")
        }

    rule date(formats: &Formats) -> Value
        = n:$([_]+) {?
            Value::from(n, &Class::Date(formats.date.to_string())).map_err(|_| "failed to parse date")
        }

    rule time(formats: &Formats) -> Value
        = n:$([_]+) {?
            Value::from(n, &Class::Time(formats.time.to_string())).map_err(|_| "failed to parse time")
        }

    rule date_time(formats: &Formats) -> Value
        = n:$([_]+) {?
            Value::from(n, &Class::DateTime(formats.date_time.to_string())).map_err(|_| "failed to parse dateTime")
        }

    rule local_date_time(formats: &Formats) -> Value
        = n:$([_]+) {?
            Value::from(n, &Class::LocalDateTime(formats.local_date_time.to_string())).map_err(|_| "failed to parse localDateTime")
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
    use crate::filter::test_utils;

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
        assert!(expression::evaluate("$integer + 9", &vec![], &test_utils::default_formats()).is_err());
        assert!(expression::evaluate("wrong == 9", &vec![], &test_utils::default_formats()).is_err());
        assert!(expression::evaluate("$integer == a", &vec![], &test_utils::default_formats()).is_err());
    }

    #[test]
    fn valid_integer_conditions() {
        assert!(expression::evaluate("$integer == 9", &vec![], &test_utils::default_formats()).is_ok());
        assert!(expression::evaluate("$integer != 9", &vec![], &test_utils::default_formats()).is_ok());
        assert!(expression::evaluate("$integer > 9", &vec![], &test_utils::default_formats()).is_ok());
        assert!(expression::evaluate("$integer >= 9", &vec![], &test_utils::default_formats()).is_ok());
        assert!(expression::evaluate("$integer < 9", &vec![], &test_utils::default_formats()).is_ok());
        assert!(expression::evaluate("$integer <= 9", &vec![], &test_utils::default_formats()).is_ok());
    }

    #[test]
    fn evaluate_expression_no_tokens() {
        assert_eq!(
            expression::evaluate("$integer == 9", &vec![], &test_utils::default_formats()),
            Ok(HashSet::new())
        );
        assert_eq!(
            expression::evaluate("$integer != 9", &vec![], &test_utils::default_formats()),
            Ok(HashSet::new())
        );
        assert_eq!(
            expression::evaluate("$float > 1.0", &vec![], &test_utils::default_formats()),
            Ok(HashSet::new())
        );
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
            Token {
                position: 3,
                separator: false,
                word: "2021-01-01",
            },
            Token {
                position: 4,
                separator: false,
                word: "15:15:15",
            },
            Token {
                position: 5,
                separator: false,
                word: "2001-07-08T00:34:60.026490+09:30",
            },
            Token {
                position: 6,
                separator: false,
                word: "2001-07-08T00:34:60.026490",
            },
        ];
        let formats = test_utils::default_formats();

        // exercise & verify
        assert_eq!(
            expression::evaluate("$integer == 9", &tokens, &formats),
            Ok(HashSet::from([1]))
        );
        assert_eq!(
            expression::evaluate("$integer != 9", &tokens, &formats),
            Ok(HashSet::from([]))
        );
        assert_eq!(
            expression::evaluate("$float == 5.5", &tokens, &formats),
            Ok(HashSet::from([2]))
        );
        assert_eq!(
            expression::evaluate("$float != 5.5", &tokens, &formats),
            Ok(HashSet::from([1]))
        );
        assert_eq!(
            expression::evaluate("$float > 0.0", &tokens, &formats),
            Ok(HashSet::from([1, 2]))
        );
        assert_eq!(
            expression::evaluate("$float < 0.0", &tokens, &formats),
            Ok(HashSet::from([]))
        );
        assert_eq!(
            expression::evaluate("$id == a1", &tokens, &formats),
            Ok(HashSet::from([0]))
        );
        assert_eq!(
            expression::evaluate("$id != a1", &tokens, &formats),
            Ok(HashSet::from([1, 2, 3, 4, 5, 6]))
        );
        assert_eq!(
            expression::evaluate("$id == b1", &tokens, &formats),
            Ok(HashSet::from([]))
        );
        assert_eq!(
            expression::evaluate("$date == 2021-01-01", &tokens, &formats),
            Ok(HashSet::from([3]))
        );
        assert_eq!(
            expression::evaluate("$date != 2021-01-01", &tokens, &formats),
            Ok(HashSet::from([]))
        );
        assert_eq!(
            expression::evaluate("$date > 2000-01-01", &tokens, &formats),
            Ok(HashSet::from([3]))
        );
        assert_eq!(
            expression::evaluate("$time == 15:15:15", &tokens, &formats),
            Ok(HashSet::from([4]))
        );
        assert_eq!(
            expression::evaluate("$time != 15:15:15", &tokens, &formats),
            Ok(HashSet::from([]))
        );
        assert_eq!(
            expression::evaluate("$time > 13:00:00", &tokens, &formats),
            Ok(HashSet::from([4]))
        );
        assert_eq!(
            expression::evaluate("$dateTime == 2001-07-08T00:34:60.026490+09:30", &tokens, &formats),
            Ok(HashSet::from([5]))
        );
        assert_eq!(
            expression::evaluate("$dateTime != 2001-07-08T00:34:60.026490+09:30", &tokens, &formats),
            Ok(HashSet::from([]))
        );
        assert_eq!(
            expression::evaluate("$dateTime > 2001-07-08T00:00:00.000000+09:30", &tokens, &formats),
            Ok(HashSet::from([5]))
        );
        assert_eq!(
            expression::evaluate("$localDateTime == 2001-07-08T00:34:60.026490", &tokens, &formats),
            Ok(HashSet::from([6]))
        );
        assert_eq!(
            expression::evaluate("$localDateTime != 2001-07-08T00:34:60.026490", &tokens, &formats),
            Ok(HashSet::from([]))
        );
        assert_eq!(
            expression::evaluate("$localDateTime > 2001-07-08T00:00:00.000000", &tokens, &formats),
            Ok(HashSet::from([6]))
        );
    }
}
