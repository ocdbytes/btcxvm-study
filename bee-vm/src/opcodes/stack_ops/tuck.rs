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

    let top = vm_state
        .pop_from_top()
        .expect("Stack should have at least 2 elements");
    let second = vm_state
        .pop_from_top()
        .expect("Stack should have at least 2 elements");

    vm_state.push_to_top(top.clone());
    vm_state.push_to_top(second);
    vm_state.push_to_top(top);

    Ok(())
}

#[cfg(test)]
mod test_opcode_tuck {
    use crate::opcodes::stack_ops::tuck::tuck;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()],
        vec!["1".to_string(), "2".to_string(), "3".to_string(), "5".to_string(), "4".to_string(), "5".to_string()]
    )]
    fn test_tuck(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = tuck(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
