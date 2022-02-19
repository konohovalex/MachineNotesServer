use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref UPPER_CASE_LETTER_REGEX: Regex = Regex::new("[[:upper:]]").unwrap();
    pub static ref LOWER_CASE_LETTER_REGEX: Regex = Regex::new("[[:lower:]]").unwrap();
    pub static ref DIGITS_REGEX: Regex = Regex::new("[[:digit:]]").unwrap();
    pub static ref SYMBOLS_REGEX: Regex = Regex::new("[[:punct:]]").unwrap();
}
