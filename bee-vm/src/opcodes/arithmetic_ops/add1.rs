use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;

/// **OP_1ADD**
///
/// 1 is added to the input
///
/// [ OP_1ADD 0x02 0x01 ]
/// => [ 0x03 0x01 ]
pub fn add_1(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let item_1 = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("add_1 : value 1".to_string())),
    })?;
    vm_state.push_to_top((item_1 + 1).to_string());

    Ok(())
}

#[cfg(test)]
mod test_opcode_add_1 {
    use crate::opcodes::arithmetic_ops::add1::add_1;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["5".to_string(), "1".to_string()], vec!["5".to_string(), "2".to_string()])]
    #[case(vec!["5".to_string(), "-2".to_string()], vec!["5".to_string(), "-1".to_string()])]
    fn test_add_1(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = add_1(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
