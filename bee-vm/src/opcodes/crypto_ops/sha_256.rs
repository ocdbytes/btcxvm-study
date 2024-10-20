use crate::errors::OpCodeErrors;
use crate::stack::Stack;
use sha2::{Digest, Sha256};

pub fn sha_256(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    if vm_state.length < 1 {
        return Err(OpCodeErrors::MissingValue(
            "Need at least one value for sha256".to_string(),
        ));
    }

    let top_element = vm_state.pop_from_top().unwrap();
    let hash = Sha256::digest(top_element);

    vm_state.push(hex::encode(hash));

    Ok(())
}

pub fn hash_256(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    if vm_state.length < 1 {
        return Err(OpCodeErrors::MissingValue(
            "Need at least one value for sha256".to_string(),
        ));
    }

    let top_element = vm_state.pop_from_top().unwrap();
    let hash = Sha256::digest(top_element);
    let hash_final = Sha256::digest(hash);

    vm_state.push(hex::encode(hash_final));

    Ok(())
}

#[cfg(test)]
mod test_opcode_sha_256 {
    use crate::errors::OpCodeErrors;
    use crate::opcodes::crypto_ops::sha_256::{hash_256, sha_256};
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["hello".to_string()], vec!["2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".to_string()])]
    fn test_sha_256(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        sha_256(&mut initial_vm_stack)?;
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }

    #[rstest]
    #[case(vec!["hello".to_string()], vec!["9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50".to_string()])]
    fn test_hash_256(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> Result<(), OpCodeErrors> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        hash_256(&mut initial_vm_stack)?;
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
