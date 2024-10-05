use crate::errors::OpCodeErrors;
use crate::stack::Stack;

/// **OP_DEPTH**
///
/// Puts the number of stack_ops items onto the stack_ops.
///
/// [ 0x10 0x20 0x30 ]
/// => [ 0x03 0x10 0x20 0x30 ]
pub fn depth(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let stack_depth = vm_state.length;
    vm_state.push_to_top(stack_depth.to_string());
    Ok(())
}

#[cfg(test)]
mod test_opcode_depth {
    use crate::opcodes::stack_ops::depth::depth;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["5".to_string(), "1".to_string()], vec!["5".to_string(), "1".to_string(), "2".to_string()])]
    #[case(vec!["2".to_string(), "3".to_string(), "4".to_string()], vec!["2".to_string(), "3".to_string(), "4".to_string(), "3".to_string()])]
    fn test_depth(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = depth(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
