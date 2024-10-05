use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// **OP_FROMALTSTACK**
///
/// Puts the input onto the top of the main stack_ops. Removes it from the alt stack_ops.
///
/// STACK = []
///
/// ALT_STACK = [ OP_TOALTSTACK 0x10 ]
///
/// EXEC .......
///
/// => STACK = [ 0x10 ]
///
/// => ALT_STACK = []
pub fn from_alt_stack(vm_state: &mut Stack, alt_stack: &mut Stack) -> Result<(), OpCodeErrors> {
    let top_stack_item_alt_stack = string_to_i32(match &alt_stack.pop_from_top() {
        Some(val) => val,
        None => {
            return Err(OpCodeErrors::MissingValue(
                "from_alt_stack : value 1".to_string(),
            ))
        }
    })?;

    vm_state.push_to_top(top_stack_item_alt_stack.to_string());

    Ok(())
}

#[cfg(test)]
mod test_opcode_from_alt_stack {
    use crate::opcodes::stack_ops::from_alt_stack::from_alt_stack;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["5".to_string(), "1".to_string()],
        vec!["1".to_string()],
        vec!["5".to_string(), "1".to_string(), "1".to_string()],
        vec![]
    )]
    fn test_from_alt_stack(
        #[case] initial: Vec<String>,
        #[case] initial_alt_stack: Vec<String>,
        #[case] expected: Vec<String>,
        #[case] expected_alt_stack: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let mut initial_vm_alt_stack = Stack::stack_from(initial_alt_stack);
        let expected_vm_stack = Stack::stack_from(expected);
        let expected_vm_alt_stack = Stack::stack_from(expected_alt_stack);
        let _res = from_alt_stack(&mut initial_vm_stack, &mut initial_vm_alt_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        assert_eq!(initial_vm_alt_stack, expected_vm_alt_stack);
        Ok(())
    }
}
