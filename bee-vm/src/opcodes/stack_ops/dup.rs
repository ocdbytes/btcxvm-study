use crate::errors::OpCodeErrors;
use crate::stack::Stack;

/// **OP_DUP / [OP_2DUP] / [[OP_3DUP]]**
///
/// Duplicates the top stack_ops item.
///
/// [Duplicates the top two stack_ops items (in same order)]
///
/// [[Duplicates the top three stack_ops items (in same order)]]
///
/// [ 0x10 0x20 ]
/// => [ 0x10 0x10 0x20 ]
pub fn dup(vm_state: &mut Stack, number_of_duplicates: i32) -> Result<(), OpCodeErrors> {
    let mut dup_stack = Stack::new();

    for i in 0..number_of_duplicates {
        match vm_state.read_ele_from_top(i) {
            Some(ele) => dup_stack.push_to_top(ele.to_string()),
            None => return Err(OpCodeErrors::MissingValue("dup : value 1".to_string())),
        }
    }

    for i in 0..number_of_duplicates {
        let ele = dup_stack
            .read_ele_from_top(i)
            .expect("Not able to read the element from dup stack_ops.");
        vm_state.push_to_top(ele.to_string());
    }

    Ok(())
}

#[cfg(test)]
mod test_opcode_dup {
    use crate::opcodes::stack_ops::dup::dup;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["5".to_string(), "1".to_string()], vec!["5".to_string(), "1".to_string(), "1".to_string()])]
    #[case(vec!["2".to_string(), "3".to_string(), "4".to_string()], vec!["2".to_string() ,"3".to_string(), "4".to_string(), "4".to_string()])]
    fn test_dup(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = dup(&mut initial_vm_stack, 1);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }

    #[rstest]
    #[case(vec!["5".to_string(), "1".to_string()], vec!["5".to_string(), "1".to_string(), "5".to_string(), "1".to_string()])]
    #[case(vec!["2".to_string(), "3".to_string(), "4".to_string()], vec!["2".to_string(), "3".to_string(), "4".to_string(), "3".to_string(), "4".to_string()])]
    fn test_2_dup(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = dup(&mut initial_vm_stack, 2);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }

    #[rstest]
    #[case(vec!["2".to_string(), "3".to_string(), "4".to_string()], vec!["2".to_string(), "3".to_string(), "4".to_string(), "2".to_string(), "3".to_string(), "4".to_string()])]
    fn test_3_dup(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        let _res = dup(&mut initial_vm_stack, 3);
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
