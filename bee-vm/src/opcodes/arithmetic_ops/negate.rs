use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// **OP_NEGATE**
///
/// Sign of input is flipped
///
/// [ 0x01 0x02 ]
/// => [ 0x-1 0x02 ]
pub fn negate(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let value = vm_state.pop();
    let item_1 = match value {
        Some(val) => string_to_i32(&val)?,
        None => return Err(OpCodeErrors::MissingValue("negate: value 1".to_string())),
    };

    vm_state.push((0 - item_1).to_string());

    Ok(())
}

#[cfg(test)]
mod test_opcode_negate {
    use crate::opcodes::arithmetic_ops::negate::negate;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["5".to_string()], vec!["-5".to_string()])]
    #[case(vec!["-2".to_string()], vec!["2".to_string()])]
    fn test_negate(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = negate(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
