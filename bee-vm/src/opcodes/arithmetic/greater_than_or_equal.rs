use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::stack::Stack;

pub fn greater_than_or_equal(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let item_1 = string_to_i32(match &vm_state.pop() {
        Some(val) => val,
        None => {
            return Err(OpCodeErrors::MissingValue(
                "greater_than_or_equal : value 1".to_string(),
            ))
        }
    })?;
    let item_2 = string_to_i32(match &vm_state.pop() {
        Some(val) => val,
        None => {
            return Err(OpCodeErrors::MissingValue(
                "greater_than_or_equal : value 2".to_string(),
            ))
        }
    })?;

    if item_1 >= item_2 {
        vm_state.push("1".to_string());
    } else {
        vm_state.push("0".to_string());
    }

    Ok(())
}
