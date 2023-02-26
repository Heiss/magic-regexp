use crate::Input::Maybe;
use regex::Regex;
use std::rc::Rc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("An regex error occurred")]
    RegexError(#[from] regex::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait AsRegex {
    /// Returns a regex that matches the given type.
    fn as_regex(&self) -> Result<Regex>;
}

/// This is a regex type that can be used to match a single character.
pub enum Type<'a> {
    Digit,
    NotDigit,
    WordBoundary,
    NotWordBoundary,
    Word,
    WordChar,
    NotWordChar,
    Text(&'a str),
    Char,
    Whitespace,
    NotWhitespace,
    Letter,
    NotLetter,
    LetterLowercase,
    NotLetterLowercase,
    LetterUppercase,
    NotLetterUppercase,
    Tab,
    NotTab,
    Linefeed,
    NotLinefeed,
    CarriageReturn,
    NotCarriageReturn,
}

/// Returns the opposite of the given type.
/// For example, `Type::Digit` will return `Type::NotDigit`.
///
/// Exceptions:
/// Type::Char will return Type::Char.
/// Type::Text will return Type::Text.
/// Type::Word will return Type::Word.
pub fn not(t: Type) -> Type {
    match t {
        Type::Digit => Type::NotDigit,
        Type::NotDigit => Type::Digit,
        Type::WordBoundary => Type::NotWordBoundary,
        Type::NotWordBoundary => Type::WordBoundary,
        Type::WordChar => Type::NotWordChar,
        Type::NotWordChar => Type::WordChar,
        Type::Whitespace => Type::NotWhitespace,
        Type::NotWhitespace => Type::Whitespace,
        Type::Letter => Type::NotLetter,
        Type::NotLetter => Type::Letter,
        Type::LetterLowercase => Type::NotLetterLowercase,
        Type::NotLetterLowercase => Type::LetterLowercase,
        Type::LetterUppercase => Type::NotLetterUppercase,
        Type::NotLetterUppercase => Type::LetterUppercase,
        Type::Tab => Type::NotTab,
        Type::NotTab => Type::Tab,
        Type::Linefeed => Type::NotLinefeed,
        Type::NotLinefeed => Type::Linefeed,
        Type::CarriageReturn => Type::NotCarriageReturn,
        Type::NotCarriageReturn => Type::CarriageReturn,
        _ => t,
    }
}

impl<'a> Type<'a> {
    fn as_str(&self) -> &str {
        match self {
            Type::Digit => r"\d",
            Type::NotDigit => r"\D",
            Type::WordBoundary => r"\b",
            Type::NotWordBoundary => r"\B",
            Type::Word => r"\b\w+\b",
            Type::WordChar => r"\w",
            Type::NotWordChar => r"\W",
            Type::Char => r".",
            Type::Text(text) => text,
            Type::Whitespace => r"\s",
            Type::NotWhitespace => r"\S",
            Type::Letter => r"[a-zA-Z]",
            Type::NotLetter => r"[^a-zA-Z]",
            Type::LetterLowercase => r"[a-z]",
            Type::NotLetterLowercase => r"[^a-z]",
            Type::LetterUppercase => r"[A-Z]",
            Type::NotLetterUppercase => r"[^A-Z]",
            Type::Tab => r"\t",
            Type::NotTab => r"^\t",
            Type::Linefeed => r"\n",
            Type::NotLinefeed => r"^\n",
            Type::CarriageReturn => r"\r",
            Type::NotCarriageReturn => r"^\r",
        }
    }
}

impl<'a> AsRegex for Type<'a> {
    ///
    /// For example, `Type::Digit.as_regex()` will return a regex that matches a single digit.
    ///
    /// # Example
    ///
    /// ```
    /// use regex::Regex;
    /// use magic_regexp::{Type, AsRegex};
    ///
    /// let regex = Type::Digit.as_regex().unwrap();
    /// assert!(regex.is_match("1"));
    /// assert!(regex.is_match("12"));
    /// assert!(!regex.is_match("a"));
    /// assert!(regex.is_match("1 2"));
    /// ```
    fn as_regex(&self) -> Result<Regex> {
        let val = Regex::new(self.as_str())?;
        Ok(val)
    }
}

/// This is a regex input that can be used to match a single character or a group of characters.
/// Can be used to create a regex that matches a single character or a group of characters.
/// For example, `Input::Exactly(Type::Digit)` will match a single digit.
///
/// # Example
/// ```
/// use magic_regexp::{create_reg_exp, Input, Type};
///
/// let regex = create_reg_exp(Input::Exactly(Type::Digit)).unwrap();
/// assert!(regex.is_match("1"));
/// assert!(regex.is_match("12"));
/// assert!(regex.is_match("1 2"));
/// ```
///
/// # Example
/// ```
/// use magic_regexp::{create_reg_exp, Input, Type};
///
/// let regex = create_reg_exp(Input::OneOrMore(Type::Digit)).unwrap();
/// assert!(regex.is_match("1"));
/// assert!(regex.is_match("12"));
/// assert!(regex.is_match("1 2"));
/// ```
///
/// # Example
/// ```
/// use magic_regexp::{create_reg_exp, Input, Type};
///
/// let regex = create_reg_exp(Input::Maybe(Type::Digit)).unwrap();
/// assert!(regex.is_match("1"));
/// assert!(regex.is_match(""));
/// assert!(regex.is_match("12"));
/// assert!(regex.is_match("a"));
/// assert!(regex.is_match("1 2"));
/// ```
pub enum Input<'a> {
    OneOrMore(Type<'a>),
    Exactly(Type<'a>),
    Maybe(Type<'a>),
}

pub enum Condition {
    And,
    Or,
}

pub struct CombinedInput<'a> {
    group_name: Option<String>,
    left: Input<'a>,
    right: Option<(Condition, Rc<CombinedInput<'a>>)>,
    optional: bool,
}

impl<'a> CombinedInput<'a> {
    pub fn and(self, right: CombinedInput<'a>) -> CombinedInput<'a> {
        CombinedInput {
            group_name: self.group_name,
            left: self.left,
            right: Some((Condition::And, Rc::new(right))),
            optional: self.optional,
        }
    }

    pub fn grouped_as(self, group_name: &'a str) -> CombinedInput<'a> {
        CombinedInput {
            group_name: Some(group_name.to_string()),
            left: self.left,
            right: self.right,
            optional: self.optional,
        }
    }

    pub fn optionally(self) -> CombinedInput<'a> {
        CombinedInput {
            group_name: None,
            left: self.left,
            right: self.right,
            optional: true,
        }
    }
}

impl<'a> AsRegex for CombinedInput<'a> {
    fn as_regex(&self) -> Result<Regex> {
        let mut regex = String::new();
        if self.optional {
            regex.push_str("(?:");
        }
        if let Some(group_name) = &self.group_name {
            regex.push_str(&format!("(?P<{}>", group_name));
        }
        match &self.left {
            Input::Exactly(t) => regex.push_str(t.as_str()),
            Input::OneOrMore(t) => regex.push_str(&format!("{}+", t.as_str())),
            Input::Maybe(t) => regex.push_str(&format!("{}?", t.as_str())),
        }
        if let Some(group_name) = &self.group_name {
            regex.push(')');
        }
        if self.optional {
            regex.push_str(")?");
        }
        if let Some((condition, right)) = &self.right {
            match condition {
                Condition::And => regex.push_str(&format!("{}+", right.as_regex()?.as_str())),
                Condition::Or => regex.push_str(&format!(
                    "{}|{}",
                    right.as_regex()?.as_str(),
                    right.as_regex()?.as_str()
                )),
            }
        }
        let val = Regex::new(&regex)?;
        Ok(val)
    }
}

impl<'a> Input<'a> {
    /// This defines the given input as a group.
    ///
    /// # Example
    /// ```
    /// use magic_regexp::{create_reg_exp, Input, Type};
    ///
    /// let regex = create_reg_exp(Input::Exactly(Type::WordChar).grouped_as("firstChar")).unwrap();
    /// assert_eq!(regex.as_str(), r"(?P<firstChar>\w)");
    /// let regex2 = create_reg_exp(Input::OneOrMore(Type::WordChar).grouped_as("firstChar")).unwrap();
    /// assert_eq!(regex2.as_str(), r"(?P<firstChar>\w+)");
    /// let regex = create_reg_exp(Type::Word);
    /// let regex2 = create_reg_exp((Input::OneOrMore(Type::WordChar)));
    /// assert_ne!(regex.unwrap().as_str(), regex2.unwrap().as_str());
    ///
    /// let regex = create_reg_exp(
    ///     Input::OneOrMore(Type::Digit)
    ///     .grouped_as("major")
    ///     .and(Input::Exactly(Type::Text(".")).as_combined())
    ///     .and(Input::OneOrMore(Type::Digit).grouped_as("minor"))
    ///     .and(Input::Exactly((Type::Text("."))).as_combined()
    ///         .and(Input::OneOrMore((Type::Char))
    ///             .as_combined()
    ///             .grouped_as("patch")
    ///         .optionally())
    ///    ));
    /// assert_eq!(regex.unwrap().as_str(),
    ///     r"(?P<major>\d+)\.(?P<minor>\d+)(?:\.(?P<patch>.+))?");
    /// ```
    pub fn grouped_as(self, name: &'a str) -> CombinedInput<'a> {
        CombinedInput {
            group_name: Some(name.to_string()),
            left: self,
            right: None,
            optional: false,
        }
    }

    pub fn and(self, other: CombinedInput<'a>) -> CombinedInput<'a> {
        CombinedInput {
            group_name: None,
            left: self,
            right: Some((Condition::And, Rc::new(other))),
            optional: false,
        }
    }

    pub fn as_combined(self) -> CombinedInput<'a> {
        CombinedInput {
            group_name: None,
            left: self,
            right: None,
            optional: false,
        }
    }
}

const ESCAPE_REPLACE_RE: &str = r"[.*+?^${}()|[\\]\\/]";

impl<'a> AsRegex for Input<'a> {
    fn as_regex(&self) -> Result<Regex> {
        let val = match self {
            Input::OneOrMore(t) => Regex::new(&format!("{}+", t.as_str()))?,
            Input::Exactly(t) => {
                let val = match *t {
                    Type::Text(t) => Regex::new(
                        Regex::new(ESCAPE_REPLACE_RE)?
                            .replace_all(t, r"\$&")
                            .to_string()
                            .as_str(),
                    )?,
                    _ => t.as_regex()?,
                };
                Regex::new(val.as_str())?
            }
            Input::Maybe(t) => Regex::new(&format!("{}?", t.as_str()))?,
        };
        Ok(val)
    }
}

pub fn create_reg_exp(input: impl AsRegex) -> Result<Regex> {
    input.as_regex()
}

#[cfg(test)]
mod tests {
    use super::{
        create_reg_exp, not,
        Input::{Exactly, OneOrMore},
        Type::Digit,
    };
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
