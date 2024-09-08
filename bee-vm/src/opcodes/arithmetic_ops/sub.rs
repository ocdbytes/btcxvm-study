use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// **OP_SUB**
///
/// stack_ops format :
///
/// [     ]
/// [  b  ]
/// [  a  ]
///
/// b is subtracted from a.
pub fn sub(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let item_1 = string_to_i32(match &vm_state.pop() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("sub : value 1".to_string())),
    })?;
    let item_2 = string_to_i32(match &vm_state.pop() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("sub : value 2".to_string())),
    })?;

    vm_state.push((item_1 - item_2).to_string());

    Ok(())
}

#[cfg(test)]
mod test_opcode_sub {
    use crate::opcodes::arithmetic_ops::sub::sub;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["1".to_string(), "3".to_string()], vec!["2".to_string()])]
    #[case(vec!["-1".to_string(), "-3".to_string()], vec!["-2".to_string()])]
    fn test_sub(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = sub(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
