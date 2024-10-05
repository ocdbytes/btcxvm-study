use crate::errors::OpCodeErrors;
use crate::opcodes::utils::string_to_i32;
use crate::stack::Stack;
use std::cmp;

/// **OP_MAX**
///
/// stack_ops format :
///
/// [     ]
/// [  b  ]
/// [  a  ]
///
/// Returns the largest of a and b.
pub fn max(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    let item_1 = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("max : value 1".to_string())),
    })?;
    let item_2 = string_to_i32(match &vm_state.pop_from_top() {
        Some(val) => val,
        None => return Err(OpCodeErrors::MissingValue("max : value 2".to_string())),
    })?;

    vm_state.push_to_top(cmp::max(item_1, item_2).to_string());

    Ok(())
}

#[cfg(test)]
mod test_opcode_max {
    use crate::opcodes::arithmetic_ops::max::max;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["5".to_string(), "1".to_string()], vec!["5".to_string()])]
    #[case(vec!["0".to_string(), "1".to_string()], vec!["1".to_string()])]
    #[case(vec!["7".to_string(), "7".to_string()], vec!["7".to_string()])]
    fn test_max(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = max(&mut initial_vm_stack);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
