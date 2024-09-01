use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::stack::Stack;

pub fn roll(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let top_stack_element = string_to_i32(match &vm_state.pop() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("roll : value 1".to_string())),
    })?;

    match vm_state.read_ele_from_top(top_stack_element) {
        Some(val) => {
            let mut temp_stack = Stack::new();

            // Moving the above vals to temp stack
            for i in 0..top_stack_element {
                if i != top_stack_element - 1 {
                    temp_stack.push(vm_state.pop().unwrap().to_string());
                }
            }

            // pushing the elements from temp stack to vm stack
            for i in 0..top_stack_element {
                if i != top_stack_element - 1 {
                    vm_state.push(vm_state.pop().unwrap().to_string());
                }
            }
            // pushing the nth element to the top of the stack
            vm_state.push(val.to_string());
        },
        None => return Err(OpCodeErrors::MissingValue("roll : value 1".to_string())),
    }

    Ok(())
}