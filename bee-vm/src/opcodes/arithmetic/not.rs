use crate::errors::OpCodeErrors;
use crate::stack::stack::Stack;

pub fn not(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let value = vm_state.pop();

    match value {
        Some(val) => {
            if val == "1" {
                vm_state.push("0".to_string());
            } else if val == "0" {
                vm_state.push("1".to_string());
            } else {
                vm_state.push("0".to_string());
            }
        }
        None => return Err(OpCodeErrors::MissingValue("not : value 1".to_string())),
    }

    Ok(())
}
