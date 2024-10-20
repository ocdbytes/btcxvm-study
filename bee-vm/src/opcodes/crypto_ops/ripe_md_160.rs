use crate::errors::OpCodeErrors;
use crate::stack::Stack;
use ripemd::{Digest, Ripemd160};
use sha2::Sha256;

pub fn ripe_md_160(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    if vm_state.length < 1 {
        return Err(OpCodeErrors::MissingValue(
            "Need at least one value for ripe md 160".to_string(),
        ));
    }

    let top_element = vm_state.pop_from_top().unwrap();

    let mut hasher = Ripemd160::new();
    hasher.update(top_element);
    let hasher_result = hasher.finalize();
    let hash = hex::encode(hasher_result);

    vm_state.push_to_top(hash);

    Ok(())
}

pub fn hash_160(vm_state: &mut Stack) -> Result<(), OpCodeErrors> {
    if vm_state.length < 1 {
        return Err(OpCodeErrors::MissingValue(
            "Need at least one value for ripe md 160".to_string(),
        ));
    }

    let top_element = vm_state.pop_from_top().unwrap();

    // sha 256
    let hash = Sha256::digest(top_element);

    // ripe md 160
    let mut hasher = Ripemd160::new();
    hasher.update(hash);
    let hasher_result = hasher.finalize();

    vm_state.push_to_top(hex::encode(hasher_result));
    Ok(())
}

#[cfg(test)]
mod test_opcode_ripe_md_160 {
    use crate::opcodes::crypto_ops::ripe_md_160::{hash_160, ripe_md_160};
    use crate::stack::Stack;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["hello".to_string()], vec!["108f07b8382412612c048d07d13f814118445acd".to_string()])]
    fn test_ripe_md_160(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        ripe_md_160(&mut initial_vm_stack)?;
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }

    #[rstest]
    #[case(vec!["hello".to_string()], vec!["b6a9c8c230722b7c748331a8b450f05566dc7d0f".to_string()])]
    fn test_hash_160(
        #[case] initial: Vec<String>,
        #[case] expected: Vec<String>,
    ) -> color_eyre::Result<()> {
        let mut initial_vm_stack = Stack::stack_from(initial);
        let expected_vm_stack = Stack::stack_from(expected);
        hash_160(&mut initial_vm_stack)?;
        assert_eq!(initial_vm_stack, expected_vm_stack);
        Ok(())
    }
}
