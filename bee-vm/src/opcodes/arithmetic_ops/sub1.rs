use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// **OP_1SUB**
///
/// 1 is subtracted from the input
///
/// [ OP_1SUB 0x02 0x01 ]
/// => [ 0x01 0x01 ]
pub fn sub_1(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let item_1 = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("sub_1 : value 1".to_string())),
    })?;
    let item_2 = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("sub_1 : value 2".to_string())),
    })?;

    vm_state.push_to_top(item_2.to_string());
    vm_state.push_to_top((item_1 - item_2).to_string());

    Ok(())
}

#[cfg(test)]
mod test_opcode_sub_1 {
    use crate::opcodes::arithmetic_ops::sub1::sub_1;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["1".to_string(), "3".to_string()], vec!["1".to_string(), "2".to_string()])]
    #[case(vec!["1".to_string(), "-1".to_string()], vec!["1".to_string(), "-2".to_string()])]
    fn test_sub_1(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = sub_1(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
