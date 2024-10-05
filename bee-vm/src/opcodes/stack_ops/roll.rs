use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// **OP_ROLL**
///
/// The item n back in the stack_ops is moved to the top.
///
/// [ OP_ROLL 2 4 3 2 1 ]
/// => [ 3 4 2 1 ]
pub fn roll(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    if vm_state.length < 2 {
        return Err(OpCodeErrors::MissingValues(
            "OP_ROLL: Insufficient items on stack_ops".to_string(),
        ));
    }

    let n_bytes = vm_state
        .pop_from_top()
        .expect("[roll] : Not able to pop the element from main stack_ops.");
    let n_i32 = string_to_i32(&n_bytes)?;

    if n_i32 >= vm_state.length {
        return Err(OpCodeErrors::NIsLargerThanOrEqualToStackSize(
            "OP_ROLL: n is larger than or equal to the stack_ops size".to_string(),
        ));
    }

    if n_i32 > 0 {
        let len = vm_state.elements.len();
        let item = vm_state.elements.remove(len - n_i32 as usize);
        vm_state.length -= 1;
        vm_state.push_to_top(item);
    }

    Ok(())
}

#[cfg(test)]
mod test_opcode_roll {
    use crate::opcodes::stack_ops::roll::roll;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "2".to_string()],
        vec!["1".to_string(), "3".to_string(), "4".to_string(), "2".to_string()]
    )]
    fn test_roll(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = roll(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
