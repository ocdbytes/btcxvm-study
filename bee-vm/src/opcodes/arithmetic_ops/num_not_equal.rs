use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// **OP_NUMNOTEQUAL**
///
/// stack_ops format :
///
/// [     ]
/// [  b  ]
/// [  a  ]
///
/// Returns 1 if the numbers are not equal, 0 otherwise.
pub fn num_not_equal(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let item_1 = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => {
            return Err(OpCodeErrors::MissingValue(
                "num_equal : value 1".to_string(),
            ))
        }
    })?;
    let item_2 = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => {
            return Err(OpCodeErrors::MissingValue(
                "num_equal : value 2".to_string(),
            ))
        }
    })?;

    if item_1 != item_2 {
        vm_state.push_to_top("1".to_string());
    } else {
        vm_state.push_to_top("0".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod test_opcode_num_not_equal {
    use crate::opcodes::arithmetic_ops::num_not_equal::num_not_equal;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["0".to_string(), "0".to_string()], vec!["0".to_string()])]
    #[case(vec!["1".to_string(), "0".to_string()], vec!["1".to_string()])]
    #[case(vec!["5".to_string(), "0".to_string()], vec!["1".to_string()])]
    fn test_num_not_equal(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = num_not_equal(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
