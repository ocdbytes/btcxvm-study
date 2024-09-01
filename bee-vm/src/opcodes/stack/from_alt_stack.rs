use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::stack::Stack;

pub fn from_alt_stack(vm_state: &mut Stack, alt_stack: &mut Stack) -> Result<(), OpCodeErrors> {
    let top_stack_item_alt_stack = string_to_i32(match &alt_stack.pop() {
        Some(val) => val,
        None => {
            return Err(OpCodeErrors::MissingValue(
                "from_alt_stack : value 1".to_string(),
            ))
        }
    })?;

    vm_state.push(top_stack_item_alt_stack.to_string());

    Ok(())
}
