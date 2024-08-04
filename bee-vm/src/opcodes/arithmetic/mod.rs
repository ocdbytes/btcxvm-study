pub mod abs;
pub mod add;
pub mod add1;
pub mod bool_and;
pub mod bool_or;
pub mod greater_than;
pub mod greater_than_or_equal;
pub mod less_than;
pub mod less_than_or_equal;
pub mod max;
pub mod min;
pub mod negate;
pub mod not;
pub mod num_equal;
pub mod num_not_equal;
pub mod op_false;
pub mod op_true;
pub mod sub;
pub mod sub1;
pub mod verify;
pub mod within;

#[cfg(test)]
mod arithmetic_ops_test {
    use crate::opcodes::arithmetic::abs::abs;
    use crate::opcodes::arithmetic::add::add;
    use crate::stack::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(String::from("5"), String::from("5"))]
    #[case(String::from("-5"), String::from("5"))]
    fn test_abs(#[case] top_element: String, #[case] expected: String) -> color_eyre::Result<()> {
        let mut vm_state = Stack::new();
        vm_state.push(top_element);
        let _res = abs(&mut vm_state);
        assert_eq!(vm_state.pop().unwrap(), expected);
        Ok(())
    }

    #[rstest]
    #[case(String::from("5"), String::from("5"), String::from("10"))]
    #[case(String::from("-5"), String::from("5"), String::from("0"))]
    fn test_add(
        #[case] ele_1: String,
        #[case] ele_2: String,
        #[case] expected: String,
    ) -> color_eyre::Result<()> {
        let mut vm_state = Stack::new();
        vm_state.push(ele_1);
        vm_state.push(ele_2);
        let _res = add(&mut vm_state);
        assert_eq!(vm_state.pop().unwrap(), expected);
        Ok(())
    }
}
