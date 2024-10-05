use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// **OP_WITHIN**
///
/// Returns 1 if x is within the specified range (left-inclusive), 0 otherwise.
///
/// => 0 5 4 OP_WITHIN
///
/// => STACK :
/// [  4  ]
/// [  5  ]
/// [  0  ]
///
/// STACK = [ 1 ]
///
/// => 4 is within range [0,5)
pub fn within(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let item_1 = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("within : value 1".to_string())),
    })?;
    let item_2 = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("within : value 2".to_string())),
    })?;
    let item_3 = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("within : value 3".to_string())),
    })?;

    if item_2 <= item_3 && item_3 < item_1 {
        vm_state.push_to_top("1".to_string());
    } else {
        vm_state.push_to_top("0".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod test_opcode_within {
    use crate::opcodes::arithmetic_ops::within::within;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["2".to_string(), "1".to_string(), "4".to_string()], vec!["1".to_string()])]
    #[case(vec!["0".to_string(), "5".to_string(), "4".to_string()], vec!["0".to_string()])]
    fn test_within(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = within(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
