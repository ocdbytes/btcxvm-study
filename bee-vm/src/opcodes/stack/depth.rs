use crate::errors::OpCodeErrors;
use crate::stack::stack::Stack;

pub fn depth(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let stack_depth = vm_state.length;
    vm_state.push(stack_depth.to_string());
    Ok(())
}
