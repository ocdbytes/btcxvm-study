use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// Marks transaction as invalid if top stack_ops value is not true. The top stack_ops value is removed.
pub fn verify(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let item = string_to_i32(match &vm_state.pop() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("verify : value 1".to_string())),
    })?;

    if item != 1 {
        return Err(OpCodeErrors::OpVerifyFailed);
    }

    Ok(())
}

#[cfg(test)]
mod test_opcode_verify {
    use crate::errors::OpCodeErrors;
    use crate::opcodes::arithmetic_ops::verify::verify;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["1".to_string()], vec![], false)]
    #[case(vec!["45550".to_string()], vec![], true)]
    fn test_verify(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
        #[case] errors: bool,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let res = verify(&mut initial_vm_stack);

        if errors {
            assert_eq!(res.unwrap_err(), OpCodeErrors::OpVerifyFailed);
        } else {
            assert_eq!(initial_vm_stack, expected_vm_stack);
        }

        Ok(())
    }
}
