use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// **OP_FROMALTSTACK**
///
/// Puts the input onto the top of the main stack_ops. Removes it from the alt stack_ops.
///
/// STACK = []
///
/// ALT_STACK = [ OP_TOALTSTACK 0x10 ]
///
/// EXEC .......
///
/// => STACK = [ 0x10 ]
///
/// => ALT_STACK = []
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
