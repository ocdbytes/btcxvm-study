use crate::errors::OpCodeErrors;
use crate::stack::Stack;

/// **OP_SWAP**
///
/// The top two items on the stack_ops are swapped.
///
/// [ OP_SWAP 1 2 3 4 ]
/// => [ 2 1 3 4 ]
pub fn swap(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    if vm_state.length < 2 {
        return Err(OpCodeErrors::MissingValues(
            "Stack length is less than 2".to_string(),
        ));
    }
    let ele_1 = vm_state
        .pop()
        .expect("Not able to get element from stack_ops.");
    let ele_2 = vm_state
        .pop()
        .expect("Not able to get element from stack_ops.");
    vm_state.push(ele_2);
    vm_state.push(ele_1);
    Ok(())
}

/// **OP_2SWAP**
///
/// Swaps the top two pairs of items.
pub fn swap_2(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    if vm_state.length < 4 {
        return Err(OpCodeErrors::MissingValues(
            "Stack length is less than 4".to_string(),
        ));
    }

    let mut temp_vec = Vec::new();
    for _ in 0..4 {
        temp_vec.push(
            vm_state
                .pop()
                .expect("Not able to get element from stack_ops."),
        );
    }

    vm_state.push(temp_vec[2].clone());
    vm_state.push(temp_vec[3].clone());
    vm_state.push(temp_vec[0].clone());
    vm_state.push(temp_vec[1].clone());

    Ok(())
}
