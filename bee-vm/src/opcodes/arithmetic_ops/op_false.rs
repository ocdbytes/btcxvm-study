use crate::stack::Stack;

/// **OP_FALSE**
///
/// Pushes 0 on top of the stack_ops
pub fn op_false(vm_state: &mut Stack) {
    vm_state.push("0".to_string());
}

#[cfg(test)]
mod test_opcode_op_false {
    use crate::opcodes::arithmetic_ops::op_false::op_false;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["1".to_string(), "3".to_string()], vec!["1".to_string(), "3".to_string(), "0".to_string()])]
    fn test_op_false(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        op_false(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
