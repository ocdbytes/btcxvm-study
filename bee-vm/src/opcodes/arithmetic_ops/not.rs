use crate::errors::OpCodeErrors;
use crate::stack::Stack;

/// **OP_NOT**
///
/// If the input is 0 or 1, it is flipped. Otherwise, the output will be 0.
///
/// [ 0x00 ]
/// => [ 0x01 ]
pub fn not(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let value = vm_state.pop();

    match value {
        Some(val) => {
            if val == "1" {
                vm_state.push("0".to_string());
            } else if val == "0" {
                vm_state.push("1".to_string());
            } else {
                vm_state.push("0".to_string());
            }
        }
        None => return Err(OpCodeErrors::MissingValue("not : value 1".to_string())),
    }

    Ok(())
}

#[cfg(test)]
mod test_opcode_not {
    use crate::opcodes::arithmetic_ops::not::not;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["0".to_string()], vec!["1".to_string()])]
    #[case(vec!["1".to_string()], vec!["0".to_string()])]
    #[case(vec!["5".to_string()], vec!["0".to_string()])]
    fn test_not(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = not(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
