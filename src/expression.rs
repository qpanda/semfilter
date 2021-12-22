extern crate peg;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use semver::Version;
use std::collections::HashSet;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

use crate::filter::Formats;
use crate::parser::Class;
use crate::parser::FromWord;
use crate::parser::Id;
use crate::parser::Term;
use crate::tokenizer::Position;
use crate::tokenizer::Token;

pub const GRAMMER_DELIMITERS: &'static [&'static str] = &["(", ")", " "];

peg::parser!(pub grammar expression() for str {
    pub rule evaluate(tokens: &Vec<Token>, formats: &Formats) -> HashSet<Position>
        = or(tokens, formats)

    rule or(tokens: &Vec<Token>, formats: &Formats) -> HashSet<Position>
        = l:and(tokens, formats) " or " r:and(tokens, formats) {
            if !l.is_empty() || !r.is_empty() {
                return l.union(&r).copied().collect();
            }

            return HashSet::new();
        }
        / and(tokens, formats)

    rule and(tokens: &Vec<Token>, formats: &Formats) -> HashSet<Position>
        = l:conditions(tokens, formats) " and " r:conditions(tokens, formats)  {
            if !l.is_empty() && !r.is_empty() {
                return l.union(&r).copied().collect();
            }

            return HashSet::new();
        }
        / conditions(tokens, formats)

    rule conditions(tokens: &Vec<Token>, formats: &Formats) -> HashSet<Position>
        = condition(tokens, formats)
        / "(" v:or(tokens, formats) ")" { v }

    rule condition(tokens: &Vec<Token>, formats: &Formats) -> HashSet<Position>
        = integer_condition(tokens)
        / float_condition(tokens)
        / id_condition(tokens)
        / date_condition(tokens, formats)
        / time_condition(tokens, formats)
        / date_time_condition(tokens, formats)
        / local_date_time_condition(tokens, formats)
        / ipv4_address_condition(tokens)
        / ipv6_address_condition(tokens)
        / ipv4_socket_address_condition(tokens)
        / ipv6_socket_address_condition(tokens)
        / semantic_version_condition(tokens)

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

    rule ipv4_address_condition(tokens: &Vec<Token>) -> HashSet<Position>
    = ipv4_addresses:ipv4_addresses(tokens) " == " ipv4_address:ipv4_address() { matches(&ipv4_addresses, |term| term.value == ipv4_address) }
    / ipv4_addresses:ipv4_addresses(tokens) " != " ipv4_address:ipv4_address() { matches(&ipv4_addresses, |term| term.value != ipv4_address) }
    / ipv4_addresses:ipv4_addresses(tokens) " > " ipv4_address:ipv4_address() { matches(&ipv4_addresses, |term| term.value > ipv4_address) }
    / ipv4_addresses:ipv4_addresses(tokens) " >= " ipv4_address:ipv4_address() { matches(&ipv4_addresses, |term| term.value >= ipv4_address) }
    / ipv4_addresses:ipv4_addresses(tokens) " < " ipv4_address:ipv4_address() { matches(&ipv4_addresses, |term| term.value < ipv4_address) }
    / ipv4_addresses:ipv4_addresses(tokens) " <= " ipv4_address:ipv4_address() { matches(&ipv4_addresses, |term| term.value <= ipv4_address) }

    rule ipv6_address_condition(tokens: &Vec<Token>) -> HashSet<Position>
    = ipv6_addresses:ipv6_addresses(tokens) " == " ipv6_address:ipv6_address() { matches(&ipv6_addresses, |term| term.value == ipv6_address) }
    / ipv6_addresses:ipv6_addresses(tokens) " != " ipv6_address:ipv6_address() { matches(&ipv6_addresses, |term| term.value != ipv6_address) }
    / ipv6_addresses:ipv6_addresses(tokens) " > " ipv6_address:ipv6_address() { matches(&ipv6_addresses, |term| term.value > ipv6_address) }
    / ipv6_addresses:ipv6_addresses(tokens) " >= " ipv6_address:ipv6_address() { matches(&ipv6_addresses, |term| term.value >= ipv6_address) }
    / ipv6_addresses:ipv6_addresses(tokens) " < " ipv6_address:ipv6_address() { matches(&ipv6_addresses, |term| term.value < ipv6_address) }
    / ipv6_addresses:ipv6_addresses(tokens) " <= " ipv6_address:ipv6_address() { matches(&ipv6_addresses, |term| term.value <= ipv6_address) }

    rule ipv4_socket_address_condition(tokens: &Vec<Token>) -> HashSet<Position>
    = ipv4_socket_addresses:ipv4_socket_addresses(tokens) " == " ipv4_socket_address:ipv4_socket_address() { matches(&ipv4_socket_addresses, |term| term.value == ipv4_socket_address) }
    / ipv4_socket_addresses:ipv4_socket_addresses(tokens) " != " ipv4_socket_address:ipv4_socket_address() { matches(&ipv4_socket_addresses, |term| term.value != ipv4_socket_address) }
    / ipv4_socket_addresses:ipv4_socket_addresses(tokens) " > " ipv4_socket_address:ipv4_socket_address() { matches(&ipv4_socket_addresses, |term| term.value > ipv4_socket_address) }
    / ipv4_socket_addresses:ipv4_socket_addresses(tokens) " >= " ipv4_socket_address:ipv4_socket_address() { matches(&ipv4_socket_addresses, |term| term.value >= ipv4_socket_address) }
    / ipv4_socket_addresses:ipv4_socket_addresses(tokens) " < " ipv4_socket_address:ipv4_socket_address() { matches(&ipv4_socket_addresses, |term| term.value < ipv4_socket_address) }
    / ipv4_socket_addresses:ipv4_socket_addresses(tokens) " <= " ipv4_socket_address:ipv4_socket_address() { matches(&ipv4_socket_addresses, |term| term.value <= ipv4_socket_address) }

    rule ipv6_socket_address_condition(tokens: &Vec<Token>) -> HashSet<Position>
    = ipv6_socket_addresses:ipv6_socket_addresses(tokens) " == " ipv6_socket_address:ipv6_socket_address() { matches(&ipv6_socket_addresses, |term| term.value == ipv6_socket_address) }
    / ipv6_socket_addresses:ipv6_socket_addresses(tokens) " != " ipv6_socket_address:ipv6_socket_address() { matches(&ipv6_socket_addresses, |term| term.value != ipv6_socket_address) }
    / ipv6_socket_addresses:ipv6_socket_addresses(tokens) " > " ipv6_socket_address:ipv6_socket_address() { matches(&ipv6_socket_addresses, |term| term.value > ipv6_socket_address) }
    / ipv6_socket_addresses:ipv6_socket_addresses(tokens) " >= " ipv6_socket_address:ipv6_socket_address() { matches(&ipv6_socket_addresses, |term| term.value >= ipv6_socket_address) }
    / ipv6_socket_addresses:ipv6_socket_addresses(tokens) " < " ipv6_socket_address:ipv6_socket_address() { matches(&ipv6_socket_addresses, |term| term.value < ipv6_socket_address) }
    / ipv6_socket_addresses:ipv6_socket_addresses(tokens) " <= " ipv6_socket_address:ipv6_socket_address() { matches(&ipv6_socket_addresses, |term| term.value <= ipv6_socket_address) }

    rule semantic_version_condition(tokens: &Vec<Token>) -> HashSet<Position>
    = semantic_versions:semantic_versions(tokens) " == " semantic_version:semantic_version() { matches(&semantic_versions, |term| term.value == semantic_version) }
    / semantic_versions:semantic_versions(tokens) " != " semantic_version:semantic_version() { matches(&semantic_versions, |term| term.value != semantic_version) }
    / semantic_versions:semantic_versions(tokens) " > " semantic_version:semantic_version() { matches(&semantic_versions, |term| term.value > semantic_version) }
    / semantic_versions:semantic_versions(tokens) " >= " semantic_version:semantic_version() { matches(&semantic_versions, |term| term.value >= semantic_version) }
    / semantic_versions:semantic_versions(tokens) " < " semantic_version:semantic_version() { matches(&semantic_versions, |term| term.value < semantic_version) }
    / semantic_versions:semantic_versions(tokens) " <= " semantic_version:semantic_version() { matches(&semantic_versions, |term| term.value <= semantic_version) }

    //
    // terms
    //
    rule integers(tokens: &Vec<Token>) -> Vec<Term<i64>>
        = "$integer" { Term::<i64>::from_tokens(tokens, &Class::Integer) }

    rule floats(tokens: &Vec<Token>) -> Vec<Term<f64>>
        = "$float" { Term::<f64>::from_tokens(tokens, &Class::Float) }

    rule ids(tokens: &Vec<Token>) -> Vec<Term<Id>>
        = "$id" { Term::<Id>::from_tokens(tokens, &Class::Id) }

    rule dates(tokens: &Vec<Token>, formats: &Formats) -> Vec<Term<NaiveDate>>
        = "$date" { Term::<NaiveDate>::from_tokens(tokens, &Class::Date(formats.date.to_string())) }

    rule times(tokens: &Vec<Token>, formats: &Formats) -> Vec<Term<NaiveTime>>
        = "$time" { Term::<NaiveTime>::from_tokens(tokens, &Class::Time(formats.time.to_string())) }

    rule date_times(tokens: &Vec<Token>, formats: &Formats) -> Vec<Term<DateTime<FixedOffset>>>
        = "$dateTime" { Term::<DateTime<FixedOffset>>::from_tokens(tokens, &Class::DateTime(formats.date_time.to_string())) }

    rule local_date_times(tokens: &Vec<Token>, formats: &Formats) -> Vec<Term<NaiveDateTime>>
        = "$localDateTime" { Term::<NaiveDateTime>::from_tokens(tokens, &Class::LocalDateTime(formats.local_date_time.to_string())) }

    rule ipv4_addresses(tokens: &Vec<Token>) -> Vec<Term<Ipv4Addr>>
        = "$ipv4Address" { Term::<Ipv4Addr>::from_tokens(tokens, &Class::Ipv4Address) }

    rule ipv6_addresses(tokens: &Vec<Token>) -> Vec<Term<Ipv6Addr>>
        = "$ipv6Address" { Term::<Ipv6Addr>::from_tokens(tokens, &Class::Ipv6Address) }

    rule ipv4_socket_addresses(tokens: &Vec<Token>) -> Vec<Term<SocketAddrV4>>
        = "$ipv4SocketAddress" { Term::<SocketAddrV4>::from_tokens(tokens, &Class::Ipv4SocketAddress) }

    rule ipv6_socket_addresses(tokens: &Vec<Token>) -> Vec<Term<SocketAddrV6>>
        = "$ipv6SocketAddress" { Term::<SocketAddrV6>::from_tokens(tokens, &Class::Ipv6SocketAddress) }

    rule semantic_versions(tokens: &Vec<Token>) -> Vec<Term<Version>>
        = "$semanticVersion" { Term::<Version>::from_tokens(tokens, &Class::SemanticVersion) }

    //
    // values
    //
    rule integer() -> i64
        = n:$(['+'|'-']? ['0'..='9']+) {?
            i64::from_word(n, &Class::Integer).map_err(|_| "failed to parse integer")
        }

    rule float() -> f64
        = n:$(['+'|'-']? ['0'..='9']* ['.']? ['0'..='9']*) {?
            f64::from_word(n, &Class::Float).map_err(|_| "failed to parse float")
        }

    rule id() -> Id
        = n:$(['a'..='z'|'A'..='Z'|'0'..='9'|'.'|':'|'_'|'-']+) {?
            Id::from_word(n, &Class::Id).map_err(|_| "failed to parse id")
        }

    rule date(formats: &Formats) -> NaiveDate
        = n:$([^'('|')'|' ']+) {?
            NaiveDate::from_word(n, &Class::Date(formats.date.to_string())).map_err(|_| "failed to parse date")
        }

    rule time(formats: &Formats) -> NaiveTime
        = n:$([^'('|')'|' ']+) {?
            NaiveTime::from_word(n, &Class::Time(formats.time.to_string())).map_err(|_| "failed to parse time")
        }

    rule date_time(formats: &Formats) -> DateTime<FixedOffset>
        = n:$([^'('|')'|' ']+) {?
            DateTime::<FixedOffset>::from_word(n, &Class::DateTime(formats.date_time.to_string())).map_err(|_| "failed to parse dateTime")
        }

    rule local_date_time(formats: &Formats) -> NaiveDateTime
        = n:$([^'('|')'|' ']+) {?
            NaiveDateTime::from_word(n, &Class::LocalDateTime(formats.local_date_time.to_string())).map_err(|_| "failed to parse localDateTime")
        }

    rule ipv4_address() -> Ipv4Addr
        = n:$([^'('|')'|' ']+) {?
            Ipv4Addr::from_word(n, &Class::Ipv4Address).map_err(|_| "failed to parse IPv4 address")
        }

    rule ipv6_address() -> Ipv6Addr
        = n:$([^'('|')'|' ']+) {?
            Ipv6Addr::from_word(n, &Class::Ipv6Address).map_err(|_| "failed to parse IPv6 address")
        }

    rule ipv4_socket_address() -> SocketAddrV4
        = n:$([^'('|')'|' ']+) {?
            SocketAddrV4::from_word(n, &Class::Ipv4SocketAddress).map_err(|_| "failed to parse IPv4 socket address")
        }

    rule ipv6_socket_address() -> SocketAddrV6
        = n:$([^'('|')'|' ']+) {?
            SocketAddrV6::from_word(n, &Class::Ipv6SocketAddress).map_err(|_| "failed to parse IPv6 socket address")
        }

    rule semantic_version() -> Version
        = n:$([^'('|')'|' ']+) {?
            Version::from_word(n, &Class::SemanticVersion).map_err(|_| "failed to parse semantic version")
        }
});

fn matches<T, P>(terms: &Vec<Term<T>>, predicate: P) -> HashSet<Position>
where
    P: FnMut(&&Term<T>) -> bool,
{
    terms
        .into_iter()
        .filter(predicate)
        .map(|term| term.position)
        .collect::<HashSet<Position>>()
}

#[cfg(test)]
mod matches_tests {
    use super::*;

    #[test]
    fn integer_matches() {
        // setup
        let integers = vec![
            Term { position: 2, value: 1 },
            Term { position: 4, value: 2 },
            Term { position: 6, value: 3 },
        ];

        // exercise
        let integers_eq_integer_0 = matches(&integers, |term| term.value == 0);
        let integers_eq_integer_2 = matches(&integers, |term| term.value == 2);
        let integeres_ne_integer_0 = matches(&integers, |term| term.value != 0);
        let integers_ne_integer_2 = matches(&integers, |term| term.value != 2);
        let integers_gt_integer_0 = matches(&integers, |term| term.value > 0);
        let integers_lt_integer_0 = matches(&integers, |term| term.value < 0);

        // verify
        assert_eq!(HashSet::from([]), integers_eq_integer_0);
        assert_eq!(HashSet::from([4]), integers_eq_integer_2);
        assert_eq!(HashSet::from([2, 4, 6]), integeres_ne_integer_0);
        assert_eq!(HashSet::from([2, 6]), integers_ne_integer_2);
        assert_eq!(HashSet::from([2, 4, 6]), integers_gt_integer_0);
        assert_eq!(HashSet::from([]), integers_lt_integer_0);
    }
}

#[cfg(test)]
mod expression_tests {
    use super::*;
    use crate::filter::test_utils;

    #[test]
    fn invalid_integer_expressions() {
        assert_invalid_expression("$integer + 9");
        assert_invalid_expression("wrong == 9");
        assert_invalid_expression("$integer == a");
    }

    #[test]
    fn valid_integer_expressions() {
        assert_valid_expression("$integer == 9");
        assert_valid_expression("$integer != 9");
        assert_valid_expression("$integer > 9");
        assert_valid_expression("$integer >= 9");
        assert_valid_expression("$integer < 9");
        assert_valid_expression("$integer <= 9");
        assert_valid_expression("($integer == 9)");
    }

    #[test]
    fn valid_date_expressions() {
        assert_valid_expression("$date == 2021-01-01");
        assert_valid_expression("$date != 2021-01-01");
        assert_valid_expression("$date > 2021-01-01");
        assert_valid_expression("$date >= 2021-01-01");
        assert_valid_expression("$date < 2021-01-01");
        assert_valid_expression("$date <= 2021-01-01");
        assert_valid_expression("($date == 2021-01-01)");
    }

    #[test]
    fn valid_and_expressions() {
        assert_valid_expression("$integer > 9 and $integer > 8");
        assert_valid_expression("$date > 2021-01-01 and $integer > 8");
        assert_valid_expression("$integer > 9 and ($integer > 8 and $integer > 7)");
        assert_valid_expression("($integer > 9 and $integer > 8) and $integer > 7");
        assert_valid_expression("$integer > 9 and $float < 5.5");
        assert_valid_expression("($integer > 9) and ($integer > 8)");
        assert_valid_expression("(($integer > 9) and ($integer > 8))");
    }

    #[test]
    fn invalid_and_expressions() {
        assert_invalid_expression("()");
        assert_invalid_expression("$integer > 9 and $integer > 8 and $integer > 7");
        assert_invalid_expression("$integer > 9 && $integer > 8");
        assert_invalid_expression("$integer > 9 and < 5.5");
        assert_invalid_expression("$integer > 9 (and < 5.5)");
        assert_invalid_expression("($integer > 9)($integer > 8)");
        assert_invalid_expression("(($integer > 9)($integer > 8))");
    }

    #[test]
    fn valid_or_expressions() {
        assert_valid_expression("$integer > 9 or $integer > 8");
        assert_valid_expression("$integer > 9 or $date > 2021-01-01");
        assert_valid_expression("$integer > 9 or ($integer > 8 or $integer > 7)");
        assert_valid_expression("($integer > 9 or $integer > 8) or $integer > 7");
        assert_valid_expression("$integer > 9 or $float < 5.5");
        assert_valid_expression("($integer > 9) or ($integer > 8)");
        assert_valid_expression("(($integer > 9) or ($integer > 8))");
    }

    #[test]
    fn invalid_or_expressions() {
        assert_invalid_expression("()");
        assert_invalid_expression("$integer > 9 or $integer > 8 or $integer > 7");
        assert_invalid_expression("$integer > 9 || $integer > 8");
        assert_invalid_expression("$integer > 9 or < 5.5");
        assert_invalid_expression("$integer > 9 (or < 5.5)");
        assert_invalid_expression("($integer > 9)($integer > 8)");
        assert_invalid_expression("(($integer > 9)($integer > 8))");
    }

    #[test]
    fn valid_and_or_expressions() {
        assert_valid_expression("$integer > 9 and $integer > 8 or $float < 5.5");
        assert_valid_expression("$integer > 9 and ($integer > 8 or $float < 5.5)");
        assert_valid_expression("($integer > 9 and $integer > 8) or $float < 5.5");
        assert_valid_expression("$integer > 9 or $integer > 8 and $float < 5.5");
        assert_valid_expression("$integer > 9 or ($integer > 8 and $float < 5.5)");
        assert_valid_expression("($integer > 9 or $integer > 8) and $float < 5.5");
        assert_valid_expression("($integer > 9) and ($integer > 8) or ($float < 5.5)");
    }

    fn assert_valid_expression(expression: &str) {
        assert!(expression::evaluate(expression, &vec![], &test_utils::default_formats()).is_ok());
    }

    fn assert_invalid_expression(expression: &str) {
        assert!(expression::evaluate(expression, &vec![], &test_utils::default_formats()).is_err());
    }
}

#[cfg(test)]
mod evaluation_tests {
    use super::*;
    use crate::filter::test_utils;

    #[test]
    fn evaluate_expression_without_tokens() {
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
    fn evaluate_simple_expression() {
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
            Token {
                position: 7,
                separator: false,
                word: "8.8.8.8",
            },
            Token {
                position: 8,
                separator: false,
                word: "2001:4860:4860::8888",
            },
            Token {
                position: 9,
                separator: false,
                word: "8.8.8.8:53",
            },
            Token {
                position: 10,
                separator: false,
                word: "[2001:4860:4860::8888]:53",
            },
            Token {
                position: 11,
                separator: false,
                word: "1.2.3",
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
            Ok(HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]))
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
        assert_eq!(
            expression::evaluate("$ipv4Address == 8.8.8.8", &tokens, &formats),
            Ok(HashSet::from([7]))
        );
        assert_eq!(
            expression::evaluate("$ipv4Address != 8.8.8.8", &tokens, &formats),
            Ok(HashSet::from([]))
        );
        assert_eq!(
            expression::evaluate("$ipv4Address > 1.1.1.1", &tokens, &formats),
            Ok(HashSet::from([7]))
        );
        assert_eq!(
            expression::evaluate("$ipv6Address == 2001:4860:4860::8888", &tokens, &formats),
            Ok(HashSet::from([8]))
        );
        assert_eq!(
            expression::evaluate("$ipv6Address != 2001:4860:4860::8888", &tokens, &formats),
            Ok(HashSet::from([]))
        );
        assert_eq!(
            expression::evaluate("$ipv6Address > 2001:4860:4860::8844", &tokens, &formats),
            Ok(HashSet::from([8]))
        );
        assert_eq!(
            expression::evaluate("$ipv4SocketAddress == 8.8.8.8:53", &tokens, &formats),
            Ok(HashSet::from([9]))
        );
        assert_eq!(
            expression::evaluate("$ipv4SocketAddress != 8.8.8.8:53", &tokens, &formats),
            Ok(HashSet::from([]))
        );
        assert_eq!(
            expression::evaluate("$ipv4SocketAddress > 1.1.1.1:53", &tokens, &formats),
            Ok(HashSet::from([9]))
        );
        assert_eq!(
            expression::evaluate("$ipv6SocketAddress == [2001:4860:4860::8888]:53", &tokens, &formats),
            Ok(HashSet::from([10]))
        );
        assert_eq!(
            expression::evaluate("$ipv6SocketAddress != [2001:4860:4860::8888]:53", &tokens, &formats),
            Ok(HashSet::from([]))
        );
        assert_eq!(
            expression::evaluate("$ipv6SocketAddress > [2001:4860:4860::8844]:53", &tokens, &formats),
            Ok(HashSet::from([10]))
        );
        assert_eq!(
            expression::evaluate("$semanticVersion == 1.2.3", &tokens, &formats),
            Ok(HashSet::from([11]))
        );
        assert_eq!(
            expression::evaluate("$semanticVersion != 1.2.3", &tokens, &formats),
            Ok(HashSet::from([]))
        );
        assert_eq!(
            expression::evaluate("$semanticVersion > 1.0.0", &tokens, &formats),
            Ok(HashSet::from([11]))
        );
    }

    #[test]
    fn evaluate_complex_expression() {
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
        let formats = test_utils::default_formats();

        // exercise & verify
        assert_eq!(
            expression::evaluate("$integer == 9 and $float == 5.5", &tokens, &formats),
            Ok(HashSet::from([1, 2]))
        );
        assert_eq!(
            expression::evaluate("$integer == 9 or $float == 5.5", &tokens, &formats),
            Ok(HashSet::from([1, 2]))
        );
        assert_eq!(
            expression::evaluate("$integer == 9 or $float == 8.8", &tokens, &formats),
            Ok(HashSet::from([1]))
        );
        assert_eq!(
            expression::evaluate("$integer == 8 or $float == 5.5", &tokens, &formats),
            Ok(HashSet::from([2]))
        );
        assert_eq!(
            expression::evaluate("$integer == 8 or $float == 6.6", &tokens, &formats),
            Ok(HashSet::from([]))
        );
        assert_eq!(
            expression::evaluate("$integer == 9 and $integer == 8", &tokens, &formats),
            Ok(HashSet::from([]))
        );
        assert_eq!(
            expression::evaluate("$integer == 9 and ($float == 5.5 or $id == a1)", &tokens, &formats),
            Ok(HashSet::from([0, 1, 2]))
        );
        assert_eq!(
            expression::evaluate("$integer == 9 and ($float == 5.5 or $id == b1)", &tokens, &formats),
            Ok(HashSet::from([1, 2]))
        );
        assert_eq!(
            expression::evaluate("$integer == 9 and ($float != 5.5 or $id == a1)", &tokens, &formats),
            Ok(HashSet::from([0, 1]))
        );
        assert_eq!(
            expression::evaluate("$integer == 9 or ($float == 8.8 or $id == b1)", &tokens, &formats),
            Ok(HashSet::from([1]))
        );
        assert_eq!(
            expression::evaluate("$integer == 9 or ($float != 5.5)", &tokens, &formats),
            Ok(HashSet::from([1]))
        );
    }

    #[test]
    fn evaluate_operator_precedence() {
        // setup
        let tokens = vec![
            Token {
                position: 0,
                separator: false,
                word: "1",
            },
            Token {
                position: 1,
                separator: false,
                word: "2.2",
            },
        ];
        let formats = test_utils::default_formats();

        // exercise & verify
        assert_eq!(
            expression::evaluate("$integer == 0 and $integer == 1 or $float == 2.2", &tokens, &formats),
            Ok(HashSet::from([1]))
        );
        assert_eq!(
            expression::evaluate("($integer == 0 and $integer == 1) or $float == 2.2", &tokens, &formats),
            Ok(HashSet::from([1]))
        );
        assert_eq!(
            expression::evaluate("$integer == 0 and ($integer == 1 or $float == 2.2)", &tokens, &formats),
            Ok(HashSet::from([]))
        );
    }
}
