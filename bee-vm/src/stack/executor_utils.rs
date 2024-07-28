/// To check if the string is numeric.
pub fn is_numeric_string(s: &String) -> bool {
    for c in s.chars() {
        if c.is_numeric() == false {
            return false;
        }
    }

    true
}

/// To check if the string is numeric and in range `1 - 16`
pub fn is_op_range(s: &String) -> bool {
    let num = s.parse::<u32>().unwrap();
    if num > 0 && num <= 16 {
        return true;
    }

    false
}
