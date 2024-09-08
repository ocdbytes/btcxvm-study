use crate::errors::OpCodeErrors;
use crate::stack::Stack;

/// **OP_TUCK**
///
/// The item at the top of the stack_ops is copied and inserted before the second-to-top item.
///
/// [ OP_TUCK 1 2 3 4 ]
/// => [ 1 2 1 3 4 ]
pub fn tuck(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    if vm_state.length < 2 {
        return Err(OpCodeErrors::MissingValues(
            "Stack length is less than 2.".to_string(),
        ));
    }

    let mut temp_vec = Vec::new();
    for _ in 0..2 {
        temp_vec.push(
            vm_state
                .pop()
                .expect("Not able to get element from stack_ops."),
        );
    }

    vm_state.push(temp_vec[0].clone());
    vm_state.push(temp_vec[1].clone());
    vm_state.push(temp_vec[0].clone());

    Ok(())
}
