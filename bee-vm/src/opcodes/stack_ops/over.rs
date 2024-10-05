use crate::errors::OpCodeErrors;
use crate::stack::Stack;

/// **OP_OVER**
///
/// Copies the second-to-top stack_ops item to the top.
///
/// [ OP_OVER 0x30 0x20 0x10 ]
/// => [ 0x20 0x30 0x20 0x10 ]
pub fn over(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    match vm_state.read_ele_from_top(1) {
        Some(val) => vm_state.push_to_top(val.to_string()),
        None => return Err(OpCodeErrors::MissingValue("over : value 1".to_string())),
    }
    Ok(())
}

/// **OP_2OVER**
///
/// Copies the pair of items two spaces back in the stack_ops to the front.
pub fn over_2(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let mut stack_temp = Stack::new();

    match vm_state.read_ele_from_top(2) {
        Some(val) => stack_temp.push_to_top(val.to_string()),
        None => return Err(OpCodeErrors::MissingValue("over_2 : value 1".to_string())),
    }
    match vm_state.read_ele_from_top(3) {
        Some(val) => stack_temp.push_to_top(val.to_string()),
        None => return Err(OpCodeErrors::MissingValue("over_2 : value 2".to_string())),
    }

    vm_state.push_to_top(stack_temp.read_ele_from_top(0).unwrap().to_string());
    vm_state.push_to_top(stack_temp.read_ele_from_top(1).unwrap().to_string());

    Ok(())
}

#[cfg(test)]
mod test_opcode_over {
    use crate::opcodes::stack_ops::over::{over, over_2};
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["5".to_string(), "1".to_string()], vec!["5".to_string(), "1".to_string(), "5".to_string()])]
    #[case(vec!["0".to_string(), "3".to_string(), "4".to_string()], vec!["0".to_string(), "3".to_string(), "4".to_string(), "3".to_string()])]
    fn test_over(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = over(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }

    #[rstest]
    #[case(vec!["0".to_string(), "1".to_string(), "2".to_string(), "3".to_string()], vec!["0".to_string(), "1".to_string(), "2".to_string(), "3".to_string(), "0".to_string(), "1".to_string()])]
    fn test_2_over(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = over_2(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
