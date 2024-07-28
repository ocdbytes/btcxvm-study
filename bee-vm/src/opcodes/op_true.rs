use crate::stack::stack::Stack;

pub fn op_true(vm_state: &mut Stack) {
    vm_state.push("1".to_string());
}
