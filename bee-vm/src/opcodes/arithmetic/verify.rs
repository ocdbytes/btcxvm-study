use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::stack::Stack;

pub fn verify(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let item = string_to_i32(match &vm_state.pop() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("verify : value 1".to_string())),
    })?;

    if item != 1 {
        return Err(OpCodeErrors::OpVerifyFailed);
    }

    Ok(())
}
