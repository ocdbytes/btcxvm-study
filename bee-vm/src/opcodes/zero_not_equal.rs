use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// Returns 0 if the input is 0. 1 otherwise.
pub fn zero_not_equal(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let item = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => {
            return Err(OpCodeErrors::MissingValue(
                "zero_not_equal : value 1".to_string(),
            ))
        }
    })?;

    if item == 0 {
        vm_state.push_to_top("0".to_string());
    } else {
        vm_state.push_to_top("1".to_string());
    }

    Ok(())
}
