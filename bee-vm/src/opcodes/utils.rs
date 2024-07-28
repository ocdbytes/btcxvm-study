use crate::errors::OpCodeErrors;

pub fn string_to_i32(s: &String) -> Result<i32, OpCodeErrors> {
    match s.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(OpCodeErrors::NumberNotInRange),
    }
}

pub fn check_if_in_range(s: &String) -> Result<bool, OpCodeErrors> {
    match s.parse::<i32>() {
        Ok(_) => Ok(true),
        Err(_) => Err(OpCodeErrors::NumberNotInRange),
    }
}
