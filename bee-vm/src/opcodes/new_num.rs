use crate::errors::OpCodeErrors;
use crate::opcodes::utils::{check_if_in_range, check_string_type, StringType};
use crate::stack::Stack;

/// To input an element into the stack_ops
pub fn new_num(vm_state: &mut Stack, input: String) -> Result<(), OpCodeErrors> {
    let input_type = check_string_type(&input);

    match input_type {
        StringType::STRING(val) => {
            vm_state.push(val);
        }
        StringType::HEX(val) => {
            vm_state.push(val);
        }
        StringType::DECIMAL(val) => {
            if check_if_in_range(&input)? {
                vm_state.push(val.to_string());
            }
        }
    }

    Err(OpCodeErrors::NumberNotInRange)
}
