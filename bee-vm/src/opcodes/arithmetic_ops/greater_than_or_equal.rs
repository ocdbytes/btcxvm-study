use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// **OP_GREATERTHANOREQUAL**
///
/// stack_ops format :
///
/// [     ]
/// [  b  ]
/// [  a  ]
///
/// Returns 1 if a is greater than or equal to b, 0 otherwise.
pub fn greater_than_or_equal(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let item_1 = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => {
            return Err(OpCodeErrors::MissingValue(
                "greater_than_or_equal : value 1".to_string(),
            ))
        }
    })?;
    let item_2 = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => {
            return Err(OpCodeErrors::MissingValue(
                "greater_than_or_equal : value 2".to_string(),
            ))
        }
    })?;

    if item_1 >= item_2 {
        vm_state.push_to_top("1".to_string());
    } else {
        vm_state.push_to_top("0".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod test_opcode_greater_than_or_equal {
    use crate::opcodes::arithmetic_ops::greater_than_or_equal::greater_than_or_equal;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["5".to_string(), "1".to_string()], vec!["0".to_string()])]
    #[case(vec!["0".to_string(), "1".to_string()], vec!["1".to_string()])]
    #[case(vec!["7".to_string(), "7".to_string()], vec!["1".to_string()])]
    fn test_greater_than_or_equal(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = greater_than_or_equal(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
