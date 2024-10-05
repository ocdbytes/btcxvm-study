use crate::errors::OpCodeErrors;
use crate::stack::Stack;

/// **OP_ROT**
///
/// The 3rd item down the stack_ops is moved to the top.
///
/// [ OP_ROT 3 2 1 ]
/// => [ 2 3 1 ]
pub fn rot(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    if vm_state.length < 3 {
        return Err(OpCodeErrors::MissingValues(
            "Stack length is less than 3.".to_string(),
        ));
    }

    // Pop the top three elements from the stack
    let third = vm_state
        .pop_from_top()
        .expect("Failed to pop third element.");
    let second = vm_state
        .pop_from_top()
        .expect("Failed to pop second element.");
    let first = vm_state
        .pop_from_top()
        .expect("Failed to pop first element.");

    // Push them back in the correct rotated order
    vm_state.push_to_top(second);
    vm_state.push_to_top(third);
    vm_state.push_to_top(first);

    Ok(())
}

/// **OP_2ROT**
///
/// The fifth and sixth items back are moved to the top of the stack_ops.
pub fn rot_2(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    // Ensure there are at least 6 elements on the stack
    if vm_state.length < 6 {
        return Err(OpCodeErrors::MissingValues(
            "Stack length is less than 6.".to_string(),
        ));
    }

    let sixth = vm_state
        .pop_from_top()
        .expect("Failed to pop sixth element.");
    let fifth = vm_state
        .pop_from_top()
        .expect("Failed to pop fifth element.");
    let fourth = vm_state
        .pop_from_top()
        .expect("Failed to pop fourth element.");
    let third = vm_state
        .pop_from_top()
        .expect("Failed to pop third element.");
    let second = vm_state
        .pop_from_top()
        .expect("Failed to pop second element.");
    let first = vm_state
        .pop_from_top()
        .expect("Failed to pop first element.");

    // Inserting elements
    vm_state.push_to_top(third);
    vm_state.push_to_top(fourth);
    vm_state.push_to_top(fifth);
    vm_state.push_to_top(sixth);
    vm_state.push_to_top(first);
    vm_state.push_to_top(second);

    Ok(())
}

#[cfg(test)]
mod test_opcode_rot {
    use crate::opcodes::stack_ops::rot::{rot, rot_2};
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string()],
        vec!["1".to_string(), "3".to_string(), "4".to_string(), "2".to_string()]
    )]
    #[case(
        vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()],
        vec!["1".to_string(), "2".to_string(), "4".to_string(), "5".to_string(), "3".to_string()]
    )]
    fn test_rot(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = rot(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }

    #[rstest]
    #[case(
        vec![
        "1".to_string(), "2".to_string(), "3".to_string(),
        "4".to_string(), "5".to_string(), "6".to_string()
        ],
        vec![
        "3".to_string(), "4".to_string(), "5".to_string(),
        "6".to_string(), "1".to_string(), "2".to_string()
        ]
    )]
    #[case(
        vec![
        "a".to_string(), "b".to_string(), "c".to_string(),
        "d".to_string(), "e".to_string(), "f".to_string()
        ],
        vec![
        "c".to_string(), "d".to_string(), "e".to_string(),
        "f".to_string(), "a".to_string(), "b".to_string()
        ]
    )]
    #[case(
        vec![
        "x".to_string(), "y".to_string(), "z".to_string(),
        "1".to_string(), "2".to_string(), "3".to_string()
        ],
        vec![
        "z".to_string(), "1".to_string(), "2".to_string(),
        "3".to_string(), "x".to_string(), "y".to_string()
        ]
    )]
    fn test_rot_2(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = rot_2(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
