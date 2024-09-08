use crate::errors::OpCodeErrors;
use crate::opcodes::utils::{check_string_type, StringType};
use crate::stack::Stack;

/// **OP_SIZE**
///
/// Pushes the string length of the top element of the stack_ops (without popping it).
///
/// [ OP_SIZE "fruit" ]
/// => [ 5 "fruit" ]
pub fn size(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    if vm_state.length == 0 {
        return Err(OpCodeErrors::MissingValue("Stack is empty.".to_string()));
    }

    let string_type = check_string_type(vm_state.read_ele_from_top(0).unwrap());

    match string_type {
        StringType::STRING(val) => {
            vm_state.push((val.len() as i32).to_string());
        }
        StringType::DECIMAL(val) => {
            vm_state.push((val.to_be_bytes().to_vec().len() as i32).to_string())
        }
        StringType::HEX(val) => {
            let decoded_val = hex::decode(&val).unwrap();
            vm_state.push((decoded_val.len() as i32).to_string())
        }
    }

    Ok(())
}
