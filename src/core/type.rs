use crate::core::traits::Condition;
use crate::{AsRegex, Result};
use regex::Regex;

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

impl<'a> AsRegex for Type<'a> {}
impl<'a> ToString for Type<'a> {
    fn to_string(&self) -> String {
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
        .to_string()
    }
}

/// Returns the opposite of the given type.
/// For example, `Type::Digit` will return `Type::NotDigit`.
/// Returns the same type if it is not a type that can be negated.
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

impl<'a> ToString for Input<'a> {
    /// Returns a string representation of the input.
    /// For example, `Input::Exactly(Type::Digit)` will return `\d`.
    ///
    /// # Example
    /// ```
    /// use magic_regexp::{Input, Type};
    ///
    /// let input = Input::Exactly(Type::Digit);
    /// assert_eq!(input.to_string(), r"\d");
    /// ```
    ///
    /// # Example
    /// ```
    /// use magic_regexp::{Input, Type};
    ///
    /// let input = Input::Exactly(Type::Text("abc"));
    /// assert_eq!(input.to_string(), "abc");
    /// let input = Input::Exactly(Type::Text("."));
    /// assert_eq!(input.to_string(), r"\.");
    /// ```
    fn to_string(&self) -> String {
        const ESCAPE_REPLACE_RE: &str = r"[.*+?^${}()|[\\]\\/]";

        match self {
            Input::OneOrMore(t) => format!("({}+)", t.to_string()),
            Input::Exactly(t) => match *t {
                Type::Text(t) => Regex::new(ESCAPE_REPLACE_RE)
                    .expect("Invalid replace_all regex")
                    .replace_all(t, r"\$&")
                    .to_string(),
                _ => t.to_string(),
            },
            Input::Maybe(t) => format!("({}?)", t.to_string()),
        }
    }
}

impl<'a> AsRegex for Input<'a> {
    fn as_regex(&self) -> Result<Regex> {
        Ok(Regex::new(&self.to_string())?)
    }
}

/// Returns a Regex, which chains the 2 given regexes with an `and` operator.
///
/// # Example
/// ```
/// use magic_regexp::{create_reg_exp, Input, Type};
///
/// let regex = create_reg_exp(Input::Exactly(Type::Digit).and(Input::Exactly(Type::Digit))).unwrap();
/// assert!(regex.is_match("12"));
/// assert!(regex.is_match("1 2"));
/// assert!(!regex.is_match("1"));
/// assert!(!regex.is_match("a"));
/// ```
///
/// # Example
/// ```
/// use magic_regexp::{create_reg_exp, Input, Type};
///
/// let regex = create_reg_exp((Input::Exactly(Type::Digit)).or(Input::Exactly(Type::Letter))).unwrap();
/// assert!(regex.is_match("1"));
/// assert!(regex.is_match("a"));
/// assert!(!regex.is_match("12"));
/// assert!(!regex.is_match("1a"));
/// ```
impl<'a> Condition for Input<'a> {}

impl AsRegex for Regex {}
impl Condition for Regex {}
