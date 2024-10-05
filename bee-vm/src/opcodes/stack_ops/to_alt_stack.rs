use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// **OP_TOALTSTACK**
///
/// Puts the input onto the top of the alt stack_ops. Removes it from the main stack_ops.
///
/// STACK = [ OP_TOALTSTACK 0x10 ]
///
/// ALT_STACK = []
///
/// EXEC ......
///
/// STACK = []
///
/// ALT_STACK = [ 0x10 ]
pub fn to_alt_stack(vm_state: &mut Stack, alt_stack: &mut Stack) -> Result<(), OpCodeErrors> {
    let top_stack_item = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => {
            return Err(OpCodeErrors::MissingValue(
                "to_alt_stack : value 1".to_string(),
            ))
        }
    })?;

    alt_stack.push_to_top(top_stack_item.to_string());

    Ok(())
}

#[cfg(test)]
mod test_opcode_to_alt_stack {
    use crate::opcodes::stack_ops::to_alt_stack::to_alt_stack;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["5".to_string(), "1".to_string()],
        vec!["1".to_string()],
        vec!["5".to_string()],
        vec!["1".to_string(), "1".to_string()]
    )]
    #[case(
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
        vec!["10".to_string()],
        vec!["1".to_string(), "2".to_string()],
        vec!["10".to_string(), "3".to_string()]
    )]
    fn test_to_alt_stack(
        #[case] initial: Vec<String>,
        #[case] initial_alt_stack: Vec<String>,
        #[case] expected: Vec<String>,
        #[case] expected_alt_stack: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let mut initial_vm_alt_stack = Stack::stack_from(initial_alt_stack);
        let expected_vm_stack = Stack::stack_from(expected);
        let expected_vm_alt_stack = Stack::stack_from(expected_alt_stack);
        let _res = to_alt_stack(&mut initial_vm_stack, &mut initial_vm_alt_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        assert_eq!(initial_vm_alt_stack, expected_vm_alt_stack);
        Ok(())
    }
}
