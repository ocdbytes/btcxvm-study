use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::stack::Stack;

pub fn pick(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let top_stack_element = string_to_i32(match &vm_state.pop() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("pick : value 1".to_string())),
    })?;

    match vm_state.read_ele_from_top(top_stack_element) {
        Some(val) => vm_state.push(val.to_string()),
        None => {
            return Err(OpCodeErrors::MissingValue(
                "pick : not able to read the element from stack for the given number.".to_string(),
            ))
        }
    }

    Ok(())
}
