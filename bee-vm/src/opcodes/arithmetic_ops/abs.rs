use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// **OP_ABS**
///
/// The input is made positive.
///
/// [ OP_ABS 0x-1 ]
/// => [ 0x01 ]
pub fn abs(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let item_1 = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("abs : value 1".to_string())),
    })?;

    vm_state.push_to_top(item_1.abs().to_string());

    Ok(())
}

#[cfg(test)]
mod test_opcode_abs {
    use crate::opcodes::arithmetic_ops::abs::abs;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["5".to_string()], vec!["5".to_string()])]
    #[case(vec!["-5".to_string()], vec!["5".to_string()])]
    fn test_abs(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = abs(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
