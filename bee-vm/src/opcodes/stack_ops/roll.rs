use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// **OP_ROLL**
///
/// The item n back in the stack_ops is moved to the top.
///
/// [ OP_ROLL 2 4 3 2 1 ]
/// => [ 3 4 2 1 ]
pub fn roll(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    if vm_state.length < 2 {
        return Err(OpCodeErrors::MissingValues(
            "OP_ROLL: Insufficient items on stack_ops".to_string(),
        ));
    }

    let n_bytes = vm_state
        .pop()
        .expect("[roll] : Not able to pop the element from main stack_ops.");
    let n_i32 = string_to_i32(&n_bytes)?;

    if n_i32 >= vm_state.length {
        return Err(OpCodeErrors::NIsLargerThanOrEqualToStackSize(
            "OP_ROLL: n is larger than or equal to the stack_ops size".to_string(),
        ));
    }

    if n_i32 > 0 {
        let len = vm_state.elements.len();
        let item = vm_state.elements.remove(len - 1 - n_i32 as usize);
        vm_state.elements.push(item);
    }

    Ok(())
}
