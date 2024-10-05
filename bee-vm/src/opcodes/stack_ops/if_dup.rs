use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// **OP_IFDUP**
///
/// If the top stack_ops value is not 0, duplicate it.
///
/// [ OP_IFDUP 0x10 ]
/// => [ 0x10 0x10 ]
pub fn if_dup(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let top_stack_item = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("if_dup : value 1".to_string())),
    })?;

    if top_stack_item != 0 {
        vm_state.push_to_top(top_stack_item.to_string());
    }

    vm_state.push_to_top(top_stack_item.to_string());

    Ok(())
}

#[cfg(test)]
mod test_opcode_if_dup {
    use crate::opcodes::stack_ops::if_dup::if_dup;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["5".to_string(), "1".to_string()], vec!["5".to_string(), "1".to_string(), "1".to_string()])]
    #[case(vec!["4".to_string(), "3".to_string(), "0".to_string()], vec!["4".to_string(), "3".to_string(), "0".to_string()])]
    fn test_if_dup(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = if_dup(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
