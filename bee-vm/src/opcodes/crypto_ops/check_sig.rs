use crate::errors::OpCodeErrors;
use crate::opcodes::crypto_ops::hash_script;
use crate::stack::Stack;
use k256::ecdsa::signature::Verifier;
use k256::ecdsa::Signature;
use k256::ecdsa::VerifyingKey;

pub fn op_checksig(stack: &mut Stack, script: &[String]) -> Result<(), OpCodeErrors> {
    // Check if we have enough elements on the stack
    if stack.elements.len() < 2 {
        return Err(OpCodeErrors::MissingValues(
            "At least 2 values needed for this operation".to_string(),
        ));
    }

    // Pop signature and public key from stack
    let signature_hex = stack.pop_from_top().unwrap();
    let public_key_str = stack.pop_from_top().unwrap();

    // Parse the public key/verifying key
    let public_key: VerifyingKey = VerifyingKey::from_sec1_bytes(
        &hex::decode(public_key_str).map_err(|_| OpCodeErrors::InvalidPublicKey)?,
    )
    .map_err(|_| OpCodeErrors::InvalidPublicKey)?;

    // Create a copy of the script for modification
    let mut script_for_hash = script.to_vec();

    // Remove all signatures from the script before hashing
    script_for_hash.retain(|op| op != &signature_hex);

    // Hash the modified script
    let script_hash = hash_script(script_for_hash)?; // same as message

    let signature: Signature = Signature::from_slice(
        &hex::decode(signature_hex).map_err(|_| OpCodeErrors::InvalidSignature)?,
    )
    .map_err(|_| OpCodeErrors::InvalidSignature)?;

    // Verify the signature
    let result = public_key
        .verify(script_hash.as_slice(), &signature)
        .is_ok();

    // Push result to stack (1 for success, 0 for failure)
    stack.push_to_top(if result { "1" } else { "0" }.to_string());

    Ok(())
}

#[cfg(test)]
mod check_sigs_test {
    use crate::opcodes::crypto_ops::hash_script;
    use crate::stack::executor::execute_code;
    use hex;
    use k256::ecdsa::signature::Signer;
    use k256::ecdsa::Signature;
    use k256::ecdsa::SigningKey;
    use k256::elliptic_curve::rand_core::OsRng;
    use rstest::rstest;

    /// Helper function to create a test script and necessary cryptographic components
    fn create_test_script() -> (Vec<String>, SigningKey, String) {
        // Generate a deterministic key pair for testing
        let signing_key = SigningKey::random(&mut OsRng);
        let public_key = signing_key.verifying_key().to_sec1_bytes();

        // Create a basic script with OP_CODESEPARATOR
        let script = vec![
            "1".to_string(),
            "2".to_string(),
            "OP_ADD".to_string(),
            "OP_CODESEPARATOR".to_string(),
            "3".to_string(),
            "OP_EQUAL".to_string(),
        ];

        (script, signing_key, hex::encode(public_key))
    }

    #[rstest]
    fn test_op_checksig_with_codeseparator_valid_signature() -> color_eyre::Result<()> {
        let (mut script, signing_key, public_key) = create_test_script();

        // Create and sign first message
        let sig_script = vec![
            "3".to_string(),
            "OP_EQUAL".to_string(),
            public_key.clone(),
            "OP_CHECKSIG".to_string(),
        ];
        let message = hash_script(sig_script)?;
        let signature: Signature = signing_key.sign(message.as_slice());

        // Add signature and public key to the script
        script.push(public_key);
        script.push(hex::encode(signature.to_bytes()));
        script.push("OP_CHECKSIG".to_string());

        // Execute the script
        let (final_stack, _) = execute_code(script)?;

        // Check the result - should be valid
        assert_eq!(final_stack.read_ele_from_top(0).unwrap(), "1");

        Ok(())
    }

    #[rstest]
    fn test_op_checksig_with_invalid_signature() -> color_eyre::Result<()> {
        let (mut script, signing_key, public_key) = create_test_script();

        // Create an invalid signature by signing different data
        let message = &[0; 32];
        let invalid_signature: Signature = signing_key.sign(message.as_slice());

        script.push(public_key);
        script.push(hex::encode(invalid_signature.to_bytes()));
        script.push("OP_CHECKSIG".to_string());

        let (final_stack, _) = execute_code(script)?;
        assert_eq!(final_stack.read_ele_from_top(0).unwrap(), "0");

        Ok(())
    }
}
