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
        .pop_from_top()
        .expect("Not able to get element from stack_ops.");
    let ele_2 = vm_state
        .pop_from_top()
        .expect("Not able to get element from stack_ops.");
    vm_state.push_to_top(ele_1);
    vm_state.push_to_top(ele_2);
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

    let mut temp_vec = Stack::new();
    for _ in 0..4 {
        temp_vec.push_to_top(
            vm_state
                .pop_from_top()
                .expect("Not able to get element from stack_ops."),
        );
    }

    vm_state.push_to_top(temp_vec.read_ele_from_top(2).unwrap().to_string());
    vm_state.push_to_top(temp_vec.read_ele_from_top(3).unwrap().to_string());
    vm_state.push_to_top(temp_vec.read_ele_from_top(0).unwrap().to_string());
    vm_state.push_to_top(temp_vec.read_ele_from_top(1).unwrap().to_string());

    Ok(())
}

#[cfg(test)]
mod test_opcode_swap {
    use crate::opcodes::stack_ops::swap::{swap, swap_2};
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["a".to_string(), "b".to_string()],
        vec!["b".to_string(), "a".to_string()]
    )]
    #[case(
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
        vec!["1".to_string(), "3".to_string(), "2".to_string()]
    )]
    fn test_swap(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        swap(&mut initial_vm_stack)?;
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }

    #[rstest]
    #[case(
        vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()],
        vec!["c".to_string(), "d".to_string(), "a".to_string(), "b".to_string()]
    )]
    #[case(
        vec!["w".to_string(), "x".to_string(), "y".to_string(), "z".to_string()],
        vec!["y".to_string(), "z".to_string(), "w".to_string(), "x".to_string()]
    )]
    fn test_swap_2(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        swap_2(&mut initial_vm_stack)?;
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
