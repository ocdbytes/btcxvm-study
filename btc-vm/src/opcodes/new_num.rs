use crate::stack::stack::Stack;

pub fn new_num(vm_state: &mut Stack, num: String) {
    vm_state.push(num)
}
