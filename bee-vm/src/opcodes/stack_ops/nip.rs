use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// **OP_NIP**
///
/// Removes the second-to-top stack_ops item.
///
/// [ OP_NIP 0x20 0x10 0x30 ]
/// => [ 0x20 0x30 ]
pub fn nip(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let top_element = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("nip : value 1".to_string())),
    })?;

    vm_state.pop_from_top();
    vm_state.push_to_top(top_element.to_string());

    Ok(())
}

#[cfg(test)]
mod test_opcode_nip {
    use crate::opcodes::stack_ops::nip::nip;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["5".to_string(), "1".to_string()], vec!["1".to_string()])]
    #[case(vec!["0".to_string(), "3".to_string(), "4".to_string()], vec!["0".to_string(), "4".to_string()])]
    fn test_nip(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = nip(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
