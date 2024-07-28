use super::utils::string_to_u32;
use crate::stack::stack::Stack;

pub fn add(vm_state: &mut Stack) {
    let item_1 = string_to_u32(&vm_state.pop().unwrap());
    let item_2 = string_to_u32(&vm_state.pop().unwrap());

    vm_state.push((item_1 + item_2).to_string());
}
