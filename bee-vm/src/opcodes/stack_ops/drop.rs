use crate::errors::OpCodeErrors;
use crate::stack::Stack;

/// **OP_DROP / [OP_2DROP]**
///
/// Removes the top stack_ops item.
///
/// [Removes the top two stack_ops items.]
///
/// [ 0x10 0x20 ]
/// => [ 0x20 ]
pub fn op_drop(vm_state: &mut Stack, number_of_drops: i32) -> Result<(), OpCodeErrors> {
    for _ in 0..number_of_drops {
        vm_state.pop();
    }
    Ok(())
}
