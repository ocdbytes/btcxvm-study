use crate::errors::OpCodeErrors;
use crate::stack::Stack;

/// **OP_DEPTH**
///
/// Puts the number of stack_ops items onto the stack_ops.
///
/// [ 0x10 0x20 0x30 ]
/// => [ 0x03 0x10 0x20 0x30 ]
pub fn depth(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let stack_depth = vm_state.length;
    vm_state.push(stack_depth.to_string());
    Ok(())
}
