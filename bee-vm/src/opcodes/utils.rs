use crate::errors::OpCodeErrors;

/// To convert string into `i32` integer
pub fn string_to_i32(s: &str) -> Result<i32, OpCodeErrors> {
    match s.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(OpCodeErrors::NumberNotInRange),
    }
}

/// To check if the input is in `i32` integer range.
pub fn check_if_in_range(s: &str) -> Result<bool, OpCodeErrors> {
    match s.parse::<i32>() {
        Ok(_) => Ok(true),
        Err(_) => Err(OpCodeErrors::NumberNotInRange),
    }
}

/// enum for type of the element in a stack_ops
#[derive(Debug)]
pub enum StringType {
    DECIMAL(i32),
    HEX(String),
    STRING(String),
}

#[allow(clippy::if_same_then_else)]
pub fn check_string_type(s: &str) -> StringType {
    if s.chars().all(|c| c.is_ascii_digit()) {
        StringType::DECIMAL(string_to_i32(s).unwrap())
    } else if s.starts_with("0x") && s[2..].chars().all(|c| c.is_ascii_hexdigit()) {
        StringType::HEX(s.to_string())
    } else if s.chars().all(|c| c.is_ascii_hexdigit()) {
        StringType::HEX(s.to_string())
    } else {
        StringType::STRING(s.to_string())
    }
}
