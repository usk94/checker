use regex::Regex;
use std::fmt;

pub enum CheckerType {
    Number,
    Boolean,
    Japanese,
    PhoneNumber,
}

impl CheckerType {
    fn name(&self) -> String {
        match self {
            CheckerType::Number => String::from("数値"),
            CheckerType::Boolean => String::from("真偽値"),
            CheckerType::Japanese => String::from("日本語"),
            CheckerType::PhoneNumber => String::from("電話番号"),
        }
    }

    pub fn check(&self, input: &str) -> bool {
        match self {
            CheckerType::Number => check_number(input),
            CheckerType::Boolean => check_boolean(input),
            CheckerType::Japanese => check_japanese(input),
            CheckerType::PhoneNumber => check_phone_number(input),
        }
    }
}

impl fmt::Display for CheckerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

fn check_number(input: &str) -> bool {
    input.parse::<f64>().is_ok()
}

fn check_boolean(input: &str) -> bool {
    input == "true" || input == "false"
}

fn check_japanese(input: &str) -> bool {
    input.chars().any(|c| matches!(c, '\u{30e0}'..='\u{9fcf}' | '\u{3005}'..='\u{3006}' | '\u{3040}'..='\u{309F}' | '\u{30A0}'..='\u{30FF}'))
}

fn check_phone_number(input: &str) -> bool {
    let phone_regex = Regex::new(r"^\d{2,4}-\d{2,4}-\d{4}$").unwrap();
    phone_regex.is_match(input)
}
