use crate::errors::OpCodeErrors;
use crate::stack::Stack;

/// **OP_DROP / [OP_2DROP]**
///
/// Removes the top stack_ops item.
///
/// [Removes the top two stack_ops items.]
///
/// [ 0x10 0x20 ]
/// => [ 0x20 ]
pub fn op_drop(vm_state: &mut Stack, number_of_drops: i32) -> Result<(), OpCodeErrors> {
    for _ in 0..number_of_drops {
        vm_state.pop_from_top();
    }
    Ok(())
}

#[cfg(test)]
mod test_opcode_op_drop {
    use crate::opcodes::stack_ops::drop::op_drop;
    use crate::stack::Stack;
    use rstest::rstest;

    // OP_DROP
    #[rstest]
    #[case(vec!["5".to_string(), "1".to_string()], vec!["5".to_string()])]
    #[case(vec!["2".to_string(), "3".to_string(), "4".to_string()], vec!["2".to_string(), "3".to_string()])]
    fn test_op_drop(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = op_drop(&mut initial_vm_stack, 1);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }

    // OP_2DROP
    #[rstest]
    #[case(vec!["5".to_string(), "1".to_string()], vec![])]
    #[case(vec!["2".to_string(), "3".to_string(), "4".to_string()], vec!["2".to_string()])]
    fn test_op_2_drop(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = op_drop(&mut initial_vm_stack, 2);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
