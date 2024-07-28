use crate::errors::OpCodeErrors;
use crate::stack::stack::Stack;

pub fn negate(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let item_1 = match &vm_state.pop() {
        Some(val) => val,
        Err(_) => return Err(OpCodeErrors::MissingValue("negate: value 1".to_string())),
    };

    vm_state.push(format!("-{}", item_1));

    Ok(())
}
