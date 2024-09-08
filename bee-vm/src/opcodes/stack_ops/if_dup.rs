use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// **OP_IFDUP**
///
/// If the top stack_ops value is not 0, duplicate it.
///
/// [ OP_IFDUP 0x10 ]
/// => [ 0x10 0x10 ]
pub fn if_dup(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let top_stack_item = string_to_i32(match &vm_state.pop() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("if_dup : value 1".to_string())),
    })?;

    if top_stack_item != 0 {
        vm_state.push(top_stack_item.to_string());
    }

    vm_state.push(top_stack_item.to_string());

    Ok(())
}
