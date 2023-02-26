/*!
This crate provides a library for creating regular expressions in a more readable way.
It stands on the shoulders of the [regex](https://crates.io/crates/regex) crate.

For more specific details on the API for regular expressions, please see around the enums in the left sidebar.

# Example: Find a date

```rust
use magic_regexp::{Digit, Times, create_reg_exp, Exactly, Condition, Text};
use regex::Regex;

const TO_SEARCH: &'static str = "On 2010-03-14, foo happened. On 2014-10-14, bar happened.";
let input = Times(Digit, 4).and(Exactly(Text("-".to_string()))).and(Times(Digit, 2)).and(Exactly(Text(("-".to_string())))).and(Times(Digit, 2));
assert_eq!(input.to_string(), r"\d{4}-\d{2}-\d{2}");

let input = Times(Digit, 4).grouped_as("year").and(Exactly(Text("-".to_string()))).and(Times(Digit, 2).grouped_as("month")).and(Exactly(Text(("-".to_string())))).and(Times(Digit, 2).grouped_as("day"));
let re = create_reg_exp(input).unwrap();
assert!(re.is_match("2014-01-01"));
assert_eq!(re.find_iter(TO_SEARCH).count(), 2);

for caps in re.captures_iter(TO_SEARCH) {
    // Note that all of the unwraps are actually OK for this regex
    // because the only way for the regex to match is if all of the
    // capture groups match. This is not true in general though!
    println!("year: {}, month: {}, day: {}",
    caps.get(1).unwrap().as_str(),
    caps.get(2).unwrap().as_str(),
    caps.get(3).unwrap().as_str());
}
```
*/

mod core;
pub use crate::core::*;

use regex::Regex;

/// Returns the regex, which represents the given statement.
/// This is only for convenience and compatibility with magic-regex from npm.
///
/// # Example
/// ```
/// use magic_regexp::{Digit, Times, create_reg_exp, Exactly, Condition, Text};
/// use regex::Regex;
///
/// const TO_SEARCH: &'static str = "On 2010-03-14, foo happened. On 2014-10-14, bar happened.";
/// let input = Times(Digit, 4).and(Exactly(Text("-".to_string()))).and(Times(Digit, 2)).and(Exactly(Text(("-".to_string())))).and(Times(Digit, 2));
/// assert_eq!(input.to_string(), r"\d{4}-\d{2}-\d{2}");
/// let input = Times(Digit, 4).grouped_as("year").and(Exactly(Text("-".to_string()))).and(Times(Digit, 2).grouped_as("month")).and(Exactly(Text(("-".to_string())))).and(Times(Digit, 2).grouped_as("day"));
/// let re = create_reg_exp(input).unwrap();
/// assert!(re.is_match("2014-01-01"));
/// assert_eq!(re.find_iter(TO_SEARCH).count(), 2);
/// for caps in re.captures_iter(TO_SEARCH) {
///     // Note that all of the unwraps are actually OK for this regex
///     // because the only way for the regex to match is if all of the
///     // capture groups match. This is not true in general though!
/// println!("year: {}, month: {}, day: {}",
///         caps.get(1).unwrap().as_str(),
///         caps.get(2).unwrap().as_str(),
///         caps.get(3).unwrap().as_str());
/// }
/// ```
pub fn create_reg_exp(input: impl AsRegex) -> Result<Regex> {
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
        assert!(!regex.is_match("12"));
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
