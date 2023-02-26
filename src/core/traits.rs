use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("An regex error occurred")]
    RegexError(#[from] regex::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait AsRegex: ToString {
    /// Returns the regex, which represents the wanted statement.
    fn as_regex(&self) -> Result<Regex> {
        let regex = Regex::new(&self.to_string())?;
        Ok(regex)
    }
}

pub trait Condition: AsRegex + Sized {
    /// Returns the regex, which chains the two given statements with an `and` condition.
    fn and(self, other: impl AsRegex) -> Regex {
        Regex::new(&format!("{}{}", self.to_string(), other.to_string()))
            .expect("Invalid regex (and)")
    }
    /// Returns the regex, which chains the two given statements with an `or` condition.
    fn or(self, other: impl AsRegex) -> Regex {
        Regex::new(&format!("{}|{}", self.to_string(), other.to_string()))
            .expect("Invalid regex (or)")
    }
    /// Returns the regex, which sets the given statement to optional.
    fn optionally(self) -> Regex {
        Regex::new(&format!("({})?", self.to_string())).expect("Invalid regex (optionally)")
    }
}
