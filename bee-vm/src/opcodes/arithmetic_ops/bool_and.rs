use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// **OP_BOOLAND**
///
/// stack_ops format :
///
/// [     ]
/// [  b  ]
/// [  a  ]
///
/// If both a and b are not 0, the output is 1. Otherwise, 0.
pub fn bool_and(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let item_1 = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("bool_and : value 1".to_string())),
    })?;
    let item_2 = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("bool_and : value 2".to_string())),
    })?;

    if item_1 != 0 && item_2 != 0 {
        vm_state.push_to_top("1".to_string());
    } else {
        vm_state.push_to_top("0".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod test_opcode_bool_and {
    use crate::opcodes::arithmetic_ops::bool_and::bool_and;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["5".to_string(), "1".to_string()], vec!["1".to_string()])]
    #[case(vec!["0".to_string(), "2".to_string()], vec!["0".to_string()])]
    fn test_bool_and(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = bool_and(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
