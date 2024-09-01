use crate::errors::OpCodeErrors;
use crate::stack::stack::Stack;

pub fn dup(vm_state: &mut Stack, number_of_duplicates: i32) -> Result<(), OpCodeErrors> {
    let mut dup_stack = Stack::new();

    for i in 0..number_of_duplicates {
        match vm_state.read_ele_from_top(i) {
            Some(ele) => dup_stack.push(ele.to_string()),
            None => return Err(OpCodeErrors::MissingValue("dup : value 1".to_string())),
        }
    }

    for i in 0..number_of_duplicates {
        let ele = dup_stack
            .read_ele_from_top(i)
            .expect("Not able to read the element from dup stack.");
        vm_state.push(ele.to_string());
    }

    Ok(())
}
