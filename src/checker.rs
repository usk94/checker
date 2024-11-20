use regex::Regex;

pub fn check_number(input: &str) -> bool {
    input.parse::<f64>().is_ok()
}

pub fn check_boolean(input: &str) -> bool {
    input == "true" || input == "false"
}

pub fn check_japanese(input: &str) -> bool {
    input.chars().any(|c| matches!(c, '\u{30e0}'..='\u{9fcf}' | '\u{3005}'..='\u{3006}' | '\u{3040}'..='\u{309F}' | '\u{30A0}'..='\u{30FF}'))
}

pub fn check_phone_number(input: &str) -> bool {
    let phone_regex = Regex::new(r"^\d{2,4}-\d{2,4}-\d{4}$").unwrap();
    phone_regex.is_match(input)
}
