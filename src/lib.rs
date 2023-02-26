mod core;

pub use crate::core::{AsRegex, Error, Result};
use regex::Regex;

pub fn create_reg_exp(input: impl AsRegex) -> core::Result<Regex> {
    input.as_regex()
}

#[cfg(test)]
mod tests {
    use super::{create_reg_exp, not, Exactly, OneOrMore, Type::Digit};
    use crate::Input::Maybe;
    use crate::Type::Text;

    #[test]
    fn test_single_digit() {
        let input = Exactly(Digit);
        let regex = create_reg_exp(input).unwrap();
        assert!(regex.is_match("1"));
        assert!(regex.is_match("12"));
        assert!(regex.is_match("1 2"));
    }

    #[test]
    fn test_maybe_digit() {
        let input = Maybe(Digit);
        let regex = create_reg_exp(input).unwrap();
        assert!(regex.is_match("1"));
        assert!(regex.is_match(""));
        assert!(regex.is_match("12"));
        assert!(regex.is_match("1 2"));
    }
    #[test]
    fn test_one_or_more_digits() {
        let input = OneOrMore(Digit);
        let regex = create_reg_exp(input).unwrap();
        assert!(regex.is_match("1"));
        assert!(regex.is_match("12"));
        assert!(regex.is_match("1 2"));
        assert!(regex.is_match("123"));
        assert!(regex.is_match("12a3"));
    }

    #[test]
    fn test_not_digit() {
        let input = Exactly(not(Digit));
        let regex = create_reg_exp(input).unwrap();
        assert!(!regex.is_match("1"));
        assert!(regex.is_match("a"));
    }

    #[test]
    fn test_not_not_stuff() {
        let input = Exactly(not(not(Digit)));
        let regex = create_reg_exp(input).unwrap();
        assert!(regex.is_match("1"));
        assert!(!regex.is_match("a"));
    }

    #[test]
    fn test_exactly_text() {
        let input = Exactly(Text("welt".into()));
        let regex = create_reg_exp(input).unwrap();
        assert!(regex.is_match("Hallo welt"));
        assert!(!regex.is_match("Hallo Welt"));
    }
}
