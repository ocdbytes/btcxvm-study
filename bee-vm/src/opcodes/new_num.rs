use crate::errors::OpCodeErrors;
use crate::opcodes::utils::check_if_in_range;
use crate::stack::stack::Stack;

pub fn new_num(vm_state: &mut Stack, num: String) -> Result<(), OpCodeErrors> {
    if check_if_in_range(&num)? {
        vm_state.push(num);
        return Ok(());
    }

    Err(OpCodeErrors::NumberNotInRange)
}
