# magic-regexp

This library wants to simplify your regex-experience without the need to learn regex everytime you need to work with it. So it implements an easy to read syntactic. Also it can be used to work with the excellent [regex](https://crates.io/crates/regex) crate. So you not have to implement random regex-strings anymore, instead you write native and readable `rust`-code.

## Documentation

The documentation can be found [here](https://heiss.github.io/magic-regexp/magic_regexp/index.html).

## Usage

To bring this crate into your repository, either add magic-regexp to your Cargo.toml, or run `cargo add magic-regexp`.

Here's a simple example that matches a date in YYYY-MM-DD format and prints the year, month and day:

```rust
use magic_regexp::{Digit, Times, create_reg_exp, Exactly, Condition, Text};
use regex::Regex;

fn main() {
    let input = Times(Digit, 4)
        .and(Exactly(Text("-".to_string())))
        .and(Times(Digit, 2))
        .and(Exactly(Text(("-".to_string()))))
        .and(Times(Digit, 2));
    assert_eq!(input.to_string(), r"\d{4}-\d{2}-\d{2}");

    let input = Times(Digit, 4)
        .grouped_as("year")
        .and(Exactly(Text("-".to_string())))
        .and(Times(Digit, 2)
            .grouped_as("month"))
        .and(Exactly(Text(("-".to_string()))))
        .and(Times(Digit, 2)
            .grouped_as("day"));
    let re = create_reg_exp(input).unwrap();
    assert!(re.is_match("2014-01-01"));
    
    const TO_SEARCH: &'static str = "On 2010-03-14, foo happened. On 2014-10-14, bar happened.";
    assert_eq!(re.find_iter(TO_SEARCH).count(), 2);

    // works with lib regex like before
    for caps in re.captures_iter(TO_SEARCH) {
        // Note that all of the unwraps are actually OK for this regex
        // because the only way for the regex to match is if all of the
        // capture groups match. This is not true in general though!
        println!("year: {}, month: {}, day: {}",
                 caps.get(1).unwrap().as_str(),
                 caps.get(2).unwrap().as_str(),
                 caps.get(3).unwrap().as_str());
    }
}
```

## Help wanted

This is my first rust library, I'm not sure if I'm doing everything right. So if you have any suggestions or find bugs, please let me know in the [github issues](https://github.com/Heiss/magic-regexp/issues).

## Inspiration

This library is heavily inspired by [magic-regexp](https://github.com/danielroe/magic-regexp).
