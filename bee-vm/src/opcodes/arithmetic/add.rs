use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::stack::Stack;

pub fn add(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let item_1 = string_to_i32(match &vm_state.pop() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("add : value 1".to_string())),
    })?;
    let item_2 = string_to_i32(match &vm_state.pop() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("add : value 2".to_string())),
    })?;

    vm_state.push((item_1 + item_2).to_string());

    Ok(())
}
