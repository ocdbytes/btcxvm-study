use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// **OP_PICK**
///
/// The item n back in the stack_ops is copied to the top.
///
/// [ OP_PICK 2 4 3 2 1 ]
/// => [ 3 4 3 2 1 ]
pub fn pick(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let top_stack_element = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("pick : value 1".to_string())),
    })?;

    match vm_state.read_ele_from_top(top_stack_element) {
        Some(val) => vm_state.push_to_top(val.to_string()),
        None => {
            return Err(OpCodeErrors::MissingValue(
                "pick : not able to read the element from stack_ops for the given number."
                    .to_string(),
            ))
        }
    }

    Ok(())
}

#[cfg(test)]
mod test_opcode_pick {
    use crate::opcodes::stack_ops::pick::pick;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["0".to_string(), "3".to_string(), "4".to_string(), "1".to_string()],
        vec!["0".to_string(), "3".to_string(), "4".to_string(), "3".to_string()]
    )]
    fn test_pick(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = pick(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
