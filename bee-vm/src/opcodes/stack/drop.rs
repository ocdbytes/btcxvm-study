use crate::errors::OpCodeErrors;
use crate::stack::stack::Stack;

pub fn drop(vm_state: &mut Stack, number_of_drops: i32) -> Result<(), OpCodeErrors> {
    for i in 0..number_of_drops {
        vm_state.pop();
    }
    Ok(())
}
