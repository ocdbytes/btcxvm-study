use crate::stack::stack::Stack;

pub fn op_false(vm_state: &mut Stack) {
    vm_state.push("0".to_string());
}
