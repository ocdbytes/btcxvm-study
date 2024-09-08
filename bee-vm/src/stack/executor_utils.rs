/// To check if the string is numeric.
pub fn is_numeric_string(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_numeric() {
            return false;
        }
    }

    true
}

/// To check if the string is numeric and in range `1 - 16`
pub fn is_op_range(s: &str) -> bool {
    let num = s.parse::<u32>().unwrap();
    if num > 0 && num <= 16 {
        return true;
    }

    false
}
