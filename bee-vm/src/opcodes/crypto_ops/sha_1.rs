use crate::errors::OpCodeErrors;
use crate::stack::Stack;
use sha1::{Digest, Sha1};

pub fn sha_1(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    if vm_state.length < 1 {
        return Err(OpCodeErrors::MissingValue(
            "Need at least one value for sha1".to_string(),
        ));
    }

    let top_element = vm_state.pop_from_top().unwrap();

    let mut hasher = Sha1::new();
    hasher.update(top_element);
    let hasher_result = hasher.finalize();
    let hash = hex::encode(hasher_result);

    vm_state.push_to_top(hash);

    Ok(())
}

#[cfg(test)]
mod test_opcode_sha_1 {
    use crate::opcodes::crypto_ops::sha_1::sha_1;
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["hello".to_string()], vec!["aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d".to_string()])]
    fn test_sha_1(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        sha_1(&mut initial_vm_stack)?;
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
