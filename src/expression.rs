extern crate peg;

use crate::parser::Class;
use crate::parser::Term;
use crate::parser::Value;
use crate::tokenizer::Token;

peg::parser!(grammar expression() for str {
    // TODO return matching bool + Positions
    pub rule test(tokens: &Vec<Token>) -> bool
        = evaluate(tokens)

    // TODO return matching bool + Positions
    rule evaluate(tokens: &Vec<Token>) -> bool
        = l:class() " == " r:value() {
            let value = Value::new(&r, &l).unwrap(); // TODO error handling, how do we communicate error back?
            let terms = Term::from(tokens, &l);
            for term in terms {
                // TODO code is always the same, only operator is different, generalize
                if term.value == value {
                    return true;
                }
            }

            return false;
        }

    rule class() -> Class
        // TODO could probably use Class::from() with value instead of having to list all
        = "integer" { return Class::Integer }
        / "float" { return Class::Float }
        / "text" { return Class::Text }

    rule value() -> String
        // TODO generalize expression (everything between separators as per Tokenizer)
        = (n:$(['0'..='9']+) {
            return String::from(n);
        })
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ttt() {
        let tokens = vec![Token {
            position: 0,
            separator: false,
            text: String::from("9"),
        }];
        assert_eq!(expression::test("integer == 9", &tokens), Ok(true));
    }
}
