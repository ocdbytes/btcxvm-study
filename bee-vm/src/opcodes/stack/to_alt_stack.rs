use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::stack::Stack;

pub fn to_alt_stack(vm_state: &mut Stack, alt_stack: &mut Stack) -> Result<(), OpCodeErrors> {
    let top_stack_item = string_to_i32(match &vm_state.pop() {
        Some(val) => val,
        None => {
            return Err(OpCodeErrors::MissingValue(
                "to_alt_stack : value 1".to_string(),
            ))
        }
    })?;

    alt_stack.push(top_stack_item.to_string());

    Ok(())
}
