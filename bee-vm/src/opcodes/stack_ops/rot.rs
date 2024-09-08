use crate::errors::OpCodeErrors;
use crate::stack::Stack;

/// **OP_ROT**
///
/// The 3rd item down the stack_ops is moved to the top.
///
/// [ OP_ROT 3 2 1 ]
/// => [ 1 3 2 ]
pub fn rot(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    if vm_state.length < 3 {
        return Err(OpCodeErrors::MissingValues(
            "Stack length is less than 3.".to_string(),
        ));
    }
    let mut temp_vec = Vec::new();
    for _ in 0..3 {
        temp_vec.push(
            vm_state
                .pop()
                .expect("Not able to get element from stack_ops."),
        );
    }

    vm_state.push(temp_vec[1].clone());
    vm_state.push(temp_vec[0].clone());
    vm_state.push(temp_vec[2].clone());
    Ok(())
}

/// **OP_2ROT**
///
/// The fifth and sixth items back are moved to the top of the stack_ops.
pub fn rot_2(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    if vm_state.length < 3 {
        return Err(OpCodeErrors::MissingValues(
            "Stack length is less than 6.".to_string(),
        ));
    }
    let mut temp_vec = Vec::new();
    for _ in 0..6 {
        temp_vec.push(
            vm_state
                .pop()
                .expect("Not able to get element from stack_ops."),
        );
    }

    for item in temp_vec.iter().take(6).skip(2) {
        vm_state.push(item.to_string());
    }

    vm_state.push(temp_vec[0].clone());
    vm_state.push(temp_vec[1].clone());
    Ok(())
}
