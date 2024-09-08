use crate::errors::OpCodeErrors;
use crate::stack::Stack;

/// **OP_OVER**
///
/// Copies the second-to-top stack_ops item to the top.
///
/// [ OP_OVER 0x30 0x20 0x10 ]
/// => [ 0x20 0x30 0x20 0x10 ]
pub fn over(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    match vm_state.read_ele_from_top(1) {
        Some(val) => vm_state.push(val.to_string()),
        None => return Err(OpCodeErrors::MissingValue("over : value 1".to_string())),
    }
    Ok(())
}

/// **OP_2OVER**
///
/// Copies the pair of items two spaces back in the stack_ops to the front.
pub fn over_2(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    match vm_state.read_ele_from_top(2) {
        Some(val) => vm_state.push(val.to_string()),
        None => return Err(OpCodeErrors::MissingValue("over_2 : value 1".to_string())),
    }
    match vm_state.read_ele_from_top(3) {
        Some(val) => vm_state.push(val.to_string()),
        None => return Err(OpCodeErrors::MissingValue("over_2 : value 2".to_string())),
    }
    Ok(())
}
