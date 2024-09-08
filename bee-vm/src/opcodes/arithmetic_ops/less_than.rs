use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// **OP_LESSTHAN**
///
/// stack_ops format :
///
/// [     ]
/// [  b  ]
/// [  a  ]
///
/// Returns 1 if a is less than b, 0 otherwise.
pub fn less_than(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let item_1 = string_to_i32(match &vm_state.pop() {
        Some(val) => val,
        None => {
            return Err(OpCodeErrors::MissingValue(
                "less_than : value 1".to_string(),
            ))
        }
    })?;
    let item_2 = string_to_i32(match &vm_state.pop() {
        Some(val) => val,
        None => {
            return Err(OpCodeErrors::MissingValue(
                "less_than : value 2".to_string(),
            ))
        }
    })?;

    if item_1 < item_2 {
        vm_state.push("1".to_string());
    } else {
        vm_state.push("0".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod test_opcode_less_than {
    use crate::opcodes::arithmetic_ops::less_than::less_than;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["5".to_string(), "1".to_string()], vec!["1".to_string()])]
    #[case(vec!["0".to_string(), "1".to_string()], vec!["0".to_string()])]
    fn test_less_than(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = less_than(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
