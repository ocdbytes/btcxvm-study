use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::stack::Stack;

pub fn bool_or(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let item_1 = string_to_i32(match &vm_state.pop() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("bool_or : value 1".to_string())),
    })?;
    let item_2 = string_to_i32(match &vm_state.pop() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("bool_or : value 2".to_string())),
    })?;

    if item_1 != 0 || item_2 != 0 {
        vm_state.push("1".to_string());
    } else {
        vm_state.push("0".to_string());
    }

    Ok(())
}
