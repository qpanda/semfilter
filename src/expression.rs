extern crate peg;

use anyhow::{anyhow, Error};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use semver::Version;
use std::collections::HashSet;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use crate::filter::Formats;
use crate::parser::FromWord;
use crate::parser::Id;
use crate::parser::Parser;
use crate::parser::Term;
use crate::tokenizer::Position;
use crate::tokenizer::Separators;
use crate::tokenizer::Token;

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
        / ip_address_condition(tokens)
        / ipv4_address_condition(tokens)
        / ipv6_address_condition(tokens)
        / ip_socket_address_condition(tokens)
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
    / ids:ids(tokens) " contains " id:id() { matches(&ids, |term| term.value.contains(&id)) }
    / ids:ids(tokens) " starts-with " id:id() { matches(&ids, |term| term.value.starts_with(&id)) }
    / ids:ids(tokens) " ends-with " id:id() { matches(&ids, |term| term.value.ends_with(&id)) }

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

    rule ip_address_condition(tokens: &Vec<Token>) -> HashSet<Position>
    = ip_addresses:ip_addresses(tokens) " == " ip_address:ip_address() { matches(&ip_addresses, |term| term.value == ip_address) }
    / ip_addresses:ip_addresses(tokens) " != " ip_address:ip_address() { matches(&ip_addresses, |term| term.value != ip_address) }
    / ip_addresses:ip_addresses(tokens) " > " ip_address:ip_address() { matches(&ip_addresses, |term| term.value > ip_address) }
    / ip_addresses:ip_addresses(tokens) " >= " ip_address:ip_address() { matches(&ip_addresses, |term| term.value >= ip_address) }
    / ip_addresses:ip_addresses(tokens) " < " ip_address:ip_address() { matches(&ip_addresses, |term| term.value < ip_address) }
    / ip_addresses:ip_addresses(tokens) " <= " ip_address:ip_address() { matches(&ip_addresses, |term| term.value <= ip_address) }

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

    rule ip_socket_address_condition(tokens: &Vec<Token>) -> HashSet<Position>
    = ip_socket_addresses:ip_socket_addresses(tokens) " == " ip_socket_address:ip_socket_address() { matches(&ip_socket_addresses, |term| term.value == ip_socket_address) }
    / ip_socket_addresses:ip_socket_addresses(tokens) " != " ip_socket_address:ip_socket_address() { matches(&ip_socket_addresses, |term| term.value != ip_socket_address) }
    / ip_socket_addresses:ip_socket_addresses(tokens) " > " ip_socket_address:ip_socket_address() { matches(&ip_socket_addresses, |term| term.value > ip_socket_address) }
    / ip_socket_addresses:ip_socket_addresses(tokens) " >= " ip_socket_address:ip_socket_address() { matches(&ip_socket_addresses, |term| term.value >= ip_socket_address) }
    / ip_socket_addresses:ip_socket_addresses(tokens) " < " ip_socket_address:ip_socket_address() { matches(&ip_socket_addresses, |term| term.value < ip_socket_address) }
    / ip_socket_addresses:ip_socket_addresses(tokens) " <= " ip_socket_address:ip_socket_address() { matches(&ip_socket_addresses, |term| term.value <= ip_socket_address) }

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
        = "$integer" { Parser::<i64, ()>::from_tokens(tokens, &()) }

    rule floats(tokens: &Vec<Token>) -> Vec<Term<f64>>
        = "$float" { Parser::<f64, ()>::from_tokens(tokens, &()) }

    rule ids(tokens: &Vec<Token>) -> Vec<Term<Id>>
        = "$id" { Parser::<Id, ()>::from_tokens(tokens, &()) }

    rule dates(tokens: &Vec<Token>, formats: &Formats) -> Vec<Term<NaiveDate>>
        = "$date" { Parser::<NaiveDate, String>::from_tokens(tokens, &formats.date) }

    rule times(tokens: &Vec<Token>, formats: &Formats) -> Vec<Term<NaiveTime>>
        = "$time" { Parser::<NaiveTime, String>::from_tokens(tokens, &formats.time) }

    rule date_times(tokens: &Vec<Token>, formats: &Formats) -> Vec<Term<DateTime<FixedOffset>>>
        = "$dateTime" { Parser::<DateTime<FixedOffset>, String>::from_tokens(tokens, &formats.date_time) }

    rule local_date_times(tokens: &Vec<Token>, formats: &Formats) -> Vec<Term<NaiveDateTime>>
        = "$localDateTime" { Parser::<NaiveDateTime, String>::from_tokens(tokens, &formats.local_date_time) }

    rule ip_addresses(tokens: &Vec<Token>) -> Vec<Term<IpAddr>>
        = "$ipAddress" { Parser::<IpAddr, ()>::from_tokens(tokens, &()) }

    rule ipv4_addresses(tokens: &Vec<Token>) -> Vec<Term<Ipv4Addr>>
        = "$ipv4Address" { Parser::<Ipv4Addr, ()>::from_tokens(tokens, &()) }

    rule ipv6_addresses(tokens: &Vec<Token>) -> Vec<Term<Ipv6Addr>>
        = "$ipv6Address" { Parser::<Ipv6Addr, ()>::from_tokens(tokens, &()) }

    rule ip_socket_addresses(tokens: &Vec<Token>) -> Vec<Term<SocketAddr>>
        = "$ipSocketAddress" { Parser::<SocketAddr, ()>::from_tokens(tokens, &()) }

    rule ipv4_socket_addresses(tokens: &Vec<Token>) -> Vec<Term<SocketAddrV4>>
        = "$ipv4SocketAddress" { Parser::<SocketAddrV4, ()>::from_tokens(tokens, &()) }

    rule ipv6_socket_addresses(tokens: &Vec<Token>) -> Vec<Term<SocketAddrV6>>
        = "$ipv6SocketAddress" { Parser::<SocketAddrV6, ()>::from_tokens(tokens, &()) }

    rule semantic_versions(tokens: &Vec<Token>) -> Vec<Term<Version>>
        = "$semanticVersion" { Parser::<Version, ()>::from_tokens(tokens, &()) }

    //
    // values
    //
    rule integer() -> i64
        = n:$(['+'|'-']? ['0'..='9']+) {?
            i64::from_word(n, &()).map_err(|_| "failed to parse integer")
        }

    rule float() -> f64
        = n:$(['+'|'-']? ['0'..='9']* ['.']? ['0'..='9']*) {?
            f64::from_word(n, &()).map_err(|_| "failed to parse float")
        }

    rule id() -> Id
        = n:$(['a'..='z'|'A'..='Z'|'0'..='9'|'+'|'-'|'.'|':'|'_']+) {?
            Id::from_word(n, &()).map_err(|_| "failed to parse id")
        }

    rule date(formats: &Formats) -> NaiveDate
        = n:$([^'('|')'|' ']+) {?
            NaiveDate::from_word(n, &formats.date).map_err(|_| "failed to parse date")
        }

    rule time(formats: &Formats) -> NaiveTime
        = n:$([^'('|')'|' ']+) {?
            NaiveTime::from_word(n, &formats.time).map_err(|_| "failed to parse time")
        }

    rule date_time(formats: &Formats) -> DateTime<FixedOffset>
        = n:$([^'('|')'|' ']+) {?
            DateTime::<FixedOffset>::from_word(n, &formats.date_time).map_err(|_| "failed to parse dateTime")
        }

    rule local_date_time(formats: &Formats) -> NaiveDateTime
        = n:$([^'('|')'|' ']+) {?
            NaiveDateTime::from_word(n, &formats.local_date_time).map_err(|_| "failed to parse localDateTime")
        }

    rule ip_address() -> IpAddr
        = n:$(['0'..='9'|'a'..='f'|'A'..='F'|'.'|':']+) {?
            IpAddr::from_word(n, &()).map_err(|_| "failed to parse IP address")
        }

    rule ipv4_address() -> Ipv4Addr
        = n:$(['0'..='9'|'.']+) {?
            Ipv4Addr::from_word(n, &()).map_err(|_| "failed to parse IPv4 address")
        }

    rule ipv6_address() -> Ipv6Addr
        = n:$(['0'..='9'|'a'..='f'|'A'..='F'|':']+) {?
            Ipv6Addr::from_word(n, &()).map_err(|_| "failed to parse IPv6 address")
        }

    rule ip_socket_address() -> SocketAddr
        = n:$(['0'..='9'|'a'..='f'|'A'..='F'|'.'|':'|'['|']']+) {?
            SocketAddr::from_word(n, &()).map_err(|_| "failed to parse IP socket address")
        }

    rule ipv4_socket_address() -> SocketAddrV4
        = n:$(['0'..='9'|'.'|':']+) {?
            SocketAddrV4::from_word(n, &()).map_err(|_| "failed to parse IPv4 socket address")
        }

    rule ipv6_socket_address() -> SocketAddrV6
        = n:$(['0'..='9'|'a'..='f'|'A'..='F'|':'|'['|']']+) {?
            SocketAddrV6::from_word(n, &()).map_err(|_| "failed to parse IPv6 socket address")
        }

    rule semantic_version() -> Version
        = n:$(['0'..='9'|'.']+) {?
            Version::from_word(n, &()).map_err(|_| "failed to parse semantic version")
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

pub struct Validator {}

impl Validator {
    pub fn validate_formats(formats: &Formats) -> Result<(), Error> {
        Validator::validate_format("$date", &formats.date)?;
        Validator::validate_format("$time", &formats.time)?;
        Validator::validate_format("$dateTime", &formats.date_time)?;
        Validator::validate_format("$localDateTime", &formats.local_date_time)?;

        Ok(())
    }

    fn validate_format(class: &str, format: &str) -> Result<(), Error> {
        if format.chars().any(|c| c == ' ' || c == '(' || c == ')') {
            return Err(anyhow!(
                "'{}' format string '{}' must not contain grammar delimiters ' ' or '(' or ')'",
                class,
                format
            ));
        }

        if format.contains("%c") || format.contains("%t") || format.contains("%n") {
            return Err(anyhow!(
                "'{}' format string '{}' must not contain specifiers '%c', '%t', and '%n'",
                class,
                format
            ));
        }

        Ok(())
    }

    pub fn validate_separators(expression: &str, separators: &Separators, formats: &Formats) -> Result<(), Error> {
        Validator::validate_class_separators(expression, "$integer", separators, "+-")?;
        Validator::validate_class_separators(expression, "$float", separators, "+-.")?;
        Validator::validate_class_separators(expression, "$id", separators, "+-.:_")?;
        Validator::validate_class_separators(expression, "$date", separators, "/-.:+")?;
        Validator::validate_class_separators(expression, "$time", separators, "/-.:+")?;
        Validator::validate_class_separators(expression, "$dateTime", separators, "/-.:+")?;
        Validator::validate_class_separators(expression, "$localDateTime", separators, "/-.:+")?;
        Validator::validate_class_separators(expression, "$ipAddress", separators, ".:")?;
        Validator::validate_class_separators(expression, "$ipv4Address", separators, ".")?;
        Validator::validate_class_separators(expression, "$ipv6Address", separators, ":")?;
        Validator::validate_class_separators(expression, "$ipSocketAddress", separators, ".:[]")?;
        Validator::validate_class_separators(expression, "$ipv4SocketAddress", separators, ".:")?;
        Validator::validate_class_separators(expression, "$ipv6SocketAddress", separators, "[]:")?;
        Validator::validate_class_separators(expression, "$semanticVersion", separators, ".")?;

        Validator::validate_format_separators(expression, "$date", separators, &formats.date)?;
        Validator::validate_format_separators(expression, "$time", separators, &formats.time)?;
        Validator::validate_format_separators(expression, "$dateTime", separators, &formats.date_time)?;
        Validator::validate_format_separators(expression, "$localDateTime", separators, &formats.local_date_time)?;

        Ok(())
    }

    fn validate_class_separators(
        expression: &str,
        class: &str,
        separators: &Separators,
        characters: &str,
    ) -> Result<(), Error> {
        if expression.contains(class) {
            let separator_characters = separators.comprise_any(characters);
            if separator_characters.chars().count() != 0 {
                return Err(anyhow!(
                    "separator(s) '{}' can not be used with an expression containing '{}'",
                    separator_characters,
                    class
                ));
            }
        }

        Ok(())
    }

    fn validate_format_separators(
        expression: &str,
        class: &str,
        separators: &Separators,
        format: &str,
    ) -> Result<(), Error> {
        if expression.contains(class) {
            let raw_format = format.replace("%", "");
            let separator_characters = separators.comprise_any(&raw_format);
            if separator_characters.chars().count() != 0 {
                return Err(anyhow!(
                    "separator(s) '{}' can not be used in '{}' format string '{}' with an expression containing '{}'",
                    separator_characters,
                    class,
                    format,
                    class
                ));
            }
        }

        Ok(())
    }
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
mod validator_tests {
    use super::*;
    use crate::filter::test_utils;

    #[test]
    fn validate_formats() {
        // setup
        let valid_formats = test_utils::default_formats();
        let invalid_formats_chrono_specifier = Formats {
            date: String::from("%F"),
            time: String::from("%T"),
            date_time: String::from("%+"),
            local_date_time: String::from("%c"),
        };
        let invalid_formats_grammar_delimiter = Formats {
            date: String::from("(%F)"),
            time: String::from("%T"),
            date_time: String::from("%+"),
            local_date_time: String::from("%Y-%m-%dT%H:%M:%S%.f"),
        };

        // exercise & verify
        assert!(Validator::validate_formats(&valid_formats).is_ok());
        assert!(Validator::validate_formats(&invalid_formats_chrono_specifier).is_err());
        assert!(Validator::validate_formats(&invalid_formats_grammar_delimiter).is_err());
    }

    #[test]
    fn validate_separators() {
        // setup
        let separators = Separators::new(vec![":"]).unwrap();
        let formats = test_utils::default_formats();

        // exercise & verify
        assert!(Validator::validate_separators("$integer == 5", &separators, &formats).is_ok());
        assert!(Validator::validate_separators("$id == a", &separators, &formats).is_err());
    }
}

#[cfg(test)]
mod expression_tests {
    use super::*;
    use crate::filter::test_utils;

    #[test]
    fn invalid_expressions() {
        assert_invalid_expression("()");
        assert_invalid_expression("wrong == 9");
    }

    #[test]
    fn invalid_integer_expressions() {
        assert_invalid_expression("$integer + 9");
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
    fn invalid_date_expressions() {
        assert_invalid_expression("$date + 2021-01-01");
        assert_invalid_expression("$date == a");
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
        assert_valid_expression("$integer > 9 and $date > 2021-01-01");
        assert_valid_expression("$integer > 9 and ($integer > 8 and $integer > 7)");
        assert_valid_expression("($integer > 9 and $integer > 8) and $integer > 7");
        assert_valid_expression("$integer > 9 and $float < 5.5");
        assert_valid_expression("($integer > 9) and ($integer > 8)");
        assert_valid_expression("(($integer > 9) and ($integer > 8))");
    }

    #[test]
    fn invalid_and_expressions() {
        assert_invalid_expression("$integer > 9 && $integer > 8");
        assert_invalid_expression("$integer > 9 and < 5.5");
        assert_invalid_expression("$integer > 9 (and < 5.5)");
        assert_invalid_expression("($integer > 9)($integer > 8)");
        assert_invalid_expression("(($integer > 9)($integer > 8))");
        assert_invalid_expression("$integer > 9 and $integer > 8 and $integer > 7");
        assert_invalid_expression("$integer > 9 and $integer > 8 and $integer > 7 and $integer > 6");
        assert_invalid_expression("$integer > 9 and $integer > 8) and ($integer > 7 and $integer > 6");
    }

    #[test]
    fn valid_or_expressions() {
        assert_valid_expression("$integer > 9 or $integer > 8");
        assert_valid_expression("$date > 2021-01-01 or $integer > 8");
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
        assert_invalid_expression("$integer > 9 || $integer > 8");
        assert_invalid_expression("$integer > 9 or < 5.5");
        assert_invalid_expression("$integer > 9 (or < 5.5)");
        assert_invalid_expression("($integer > 9)($integer > 8)");
        assert_invalid_expression("(($integer > 9)($integer > 8))");
        assert_invalid_expression("$integer > 9 or $integer > 8 or $integer > 7");
        assert_invalid_expression("$integer > 9 or $integer > 8 or $integer > 7 or $integer > 6");
        assert_invalid_expression("$integer > 9 or $integer > 8) or ($integer > 7 or $integer > 6");
    }

    #[test]
    fn valid_and_or_expressions() {
        assert_valid_expression("$integer > 9 and $integer > 8 or $float < 5.5");
        assert_valid_expression("$integer > 9 and ($integer > 8 or $float < 5.5)");
        assert_valid_expression("($integer > 9 and $integer > 8) or $float < 5.5");
        assert_valid_expression("($integer > 9) and ($integer > 8) or ($float < 5.5)");
    }

    #[test]
    fn valid_or_and_expressions() {
        assert_valid_expression("$integer > 9 or $integer > 8 and $float < 5.5");
        assert_valid_expression("$integer > 9 or ($integer > 8 and $float < 5.5)");
        assert_valid_expression("($integer > 9 or $integer > 8) and $float < 5.5");
        assert_valid_expression("($integer > 9) or ($integer > 8) and ($float < 5.5)");
    }

    #[test]
    fn valid_and_and_expressions() {
        assert_valid_expression("$integer > 9 and ($integer > 8 and $float < 5.5)");
        assert_valid_expression("($integer > 9 and $integer > 8) and $float < 5.5");
    }

    #[test]
    fn invalid_and_and_expressions() {
        assert_invalid_expression("$integer > 9 and $integer > 8 and $float < 5.5");
        assert_invalid_expression("($integer > 9) and ($integer > 8) and ($float < 5.5)");
    }

    #[test]
    fn valid_or_or_expressions() {
        assert_valid_expression("$integer > 9 or ($integer > 8 or $float < 5.5)");
        assert_valid_expression("($integer > 9 or $integer > 8) or $float < 5.5");
    }

    #[test]
    fn invalid_or_or_expressions() {
        assert_invalid_expression("$integer > 9 or $integer > 8 or $float < 5.5");
        assert_invalid_expression("($integer > 9) or ($integer > 8) or ($float < 5.5)");
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
            Token {
                position: 12,
                separator: false,
                word: "qpanda",
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
            expression::evaluate("$id contains and", &tokens, &formats),
            Ok(HashSet::from([12]))
        );
        assert_eq!(
            expression::evaluate("$id starts-with qpa", &tokens, &formats),
            Ok(HashSet::from([12]))
        );
        assert_eq!(
            expression::evaluate("$id ends-with nda", &tokens, &formats),
            Ok(HashSet::from([12]))
        );
        assert_eq!(
            expression::evaluate("$id != a1", &tokens, &formats),
            Ok(HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]))
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
            expression::evaluate("$ipAddress == 8.8.8.8", &tokens, &formats),
            Ok(HashSet::from([7]))
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
            expression::evaluate("$ipAddress == 2001:4860:4860::8888", &tokens, &formats),
            Ok(HashSet::from([8]))
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
            expression::evaluate("$ipSocketAddress == 8.8.8.8:53", &tokens, &formats),
            Ok(HashSet::from([9]))
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
            expression::evaluate("$ipSocketAddress == [2001:4860:4860::8888]:53", &tokens, &formats),
            Ok(HashSet::from([10]))
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
