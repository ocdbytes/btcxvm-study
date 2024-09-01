use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::stack::Stack;

pub fn nip(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let top_element = string_to_i32(match &vm_state.pop() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("nip : value 1".to_string())),
    })?;

    vm_state.pop();
    vm_state.push(top_element.to_string());

    Ok(())
}
