use crate::errors::OpCodeErrors;
use crate::opcodes::crypto_ops::{hash_script, parse_signature};
use crate::stack::Stack;
use secp256k1::{Message, PublicKey, Secp256k1};
use std::str::FromStr;

pub fn op_checksig(
    stack: &mut Stack,
    script: &[String],
    secp: &Secp256k1<secp256k1::All>,
) -> Result<(), OpCodeErrors> {
    // Check if we have enough elements on the stack
    if stack.elements.len() < 2 {
        return Err(OpCodeErrors::MissingValues(
            "At least 2 values needed for this operation".to_string(),
        ));
    }

    // Pop signature and public key from stack
    let signature_hex = stack.pop_from_top().unwrap();
    let public_key_str = stack.pop_from_top().unwrap();

    // Parse the public key
    let public_key = match PublicKey::from_str(&public_key_str) {
        Ok(pk) => pk,
        Err(_) => return Err(OpCodeErrors::InvalidPublicKey),
    };

    // Parse signature and hash type
    let (signature, _hash_type) = parse_signature(&signature_hex)?;

    // Create a copy of the script for modification
    let mut script_for_hash = script.to_vec();

    // Remove all signatures from the script before hashing
    script_for_hash.retain(|op| op != &signature_hex);

    // Hash the modified script
    let script_hash = hash_script(script_for_hash)?;

    // Create message from script hash
    let message =
        Message::from_digest_slice(&script_hash).map_err(|_| OpCodeErrors::MessageCreationError)?;

    // Verify the signature
    let result = secp.verify_ecdsa(&message, &signature, &public_key).is_ok();

    // Push result to stack (1 for success, 0 for failure)
    stack.push_to_top(if result { "1" } else { "0" }.to_string());

    Ok(())
}

#[cfg(test)]
mod check_sigs_test {
    use crate::opcodes::crypto_ops::create_der_signature;
    use crate::opcodes::crypto_ops::{
        SIGHASH_ALL, SIGHASH_ANYONECANPAY, SIGHASH_NONE, SIGHASH_SINGLE,
    };
    use crate::stack::executor::execute_code;
    use hex;
    use rstest::rstest;
    use secp256k1::{All, Message, Secp256k1, SecretKey};
    use sha2::{Digest, Sha256};

    /// Helper function to create a test script and necessary cryptographic components
    fn create_test_script() -> (Vec<String>, SecretKey, String, Secp256k1<All>) {
        let secp = Secp256k1::new();

        // Generate a deterministic key pair for testing
        let secret_key = SecretKey::from_slice(&[0xcd; 32]).unwrap();
        let public_key = secret_key.public_key(&secp);

        // Create a basic script with OP_CODESEPARATOR
        let script = vec![
            "1".to_string(),
            "2".to_string(),
            "OP_ADD".to_string(),
            "OP_CODESEPARATOR".to_string(),
            "3".to_string(),
            "OP_EQUAL".to_string(),
        ];

        (
            script,
            secret_key,
            hex::encode(public_key.serialize()),
            secp,
        )
    }

    #[rstest]
    fn test_op_checksig_with_codeseparator_valid_signature() -> color_eyre::Result<()> {
        let (mut script, secret_key, public_key, secp) = create_test_script();

        // Create and sign first message
        let sig_script = vec![
            "3".to_string(),
            "OP_EQUAL".to_string(),
            public_key.clone(),
            "OP_CHECKSIG".to_string(),
        ];
        let message = create_message(&sig_script);
        let signature = secp.sign_ecdsa(&message, &secret_key);
        let der_signature = create_der_signature(&signature, 0x01); // SIGHASH_ALL

        // Add signature and public key to the script
        script.push(public_key);
        script.push(der_signature);
        script.push("OP_CHECKSIG".to_string());

        // Execute the script
        let (final_stack, _) = execute_code(script, secp)?;

        // Check the result - should be valid
        assert_eq!(final_stack.read_ele_from_top(0).unwrap(), "1");

        Ok(())
    }

    #[rstest]
    fn test_op_checksig_with_invalid_signature() -> color_eyre::Result<()> {
        let (mut script, secret_key, public_key, secp) = create_test_script();

        // Create an invalid signature by signing different data
        let message = Message::from_digest_slice(&[0; 32])?;
        let invalid_signature = secp.sign_ecdsa(&message, &secret_key);
        let der_signature = create_der_signature(&invalid_signature, 0x01);

        script.push(public_key);
        script.push(der_signature);
        script.push("OP_CHECKSIG".to_string());

        let (final_stack, _) = execute_code(script, secp)?;
        assert_eq!(final_stack.read_ele_from_top(0).unwrap(), "0");

        Ok(())
    }

    /// Creates a double-SHA256 hashed message from a script
    fn create_message(script: &[String]) -> Message {
        let mut hasher = Sha256::new();
        for op in script {
            hasher.update(op.as_bytes());
        }
        let first_hash = hasher.finalize();

        let mut hasher = Sha256::new();
        hasher.update(first_hash);
        let final_hash = hasher.finalize();

        Message::from_digest_slice(&final_hash).expect("32 bytes")
    }

    #[rstest]
    fn test_op_checksig_hash_types() -> color_eyre::Result<()> {
        let (script, secret_key, public_key, secp) = create_test_script();
        let sig_script = vec![
            "3".to_string(),
            "OP_EQUAL".to_string(),
            public_key.clone(),
            "OP_CHECKSIG".to_string(),
        ];

        // Test different hash types
        let hash_types = vec![
            SIGHASH_ALL,
            SIGHASH_NONE,
            SIGHASH_SINGLE,
            SIGHASH_ALL | SIGHASH_ANYONECANPAY,
            SIGHASH_NONE | SIGHASH_ANYONECANPAY,
            SIGHASH_SINGLE | SIGHASH_ANYONECANPAY,
        ];

        for hash_type in hash_types {
            let message = create_message(&sig_script);
            let signature = secp.sign_ecdsa(&message, &secret_key);
            let der_signature = create_der_signature(&signature, hash_type);

            let mut test_script = script.clone();
            test_script.push(public_key.clone());
            test_script.push(der_signature);
            test_script.push("OP_CHECKSIG".to_string());

            let (final_stack, _) = execute_code(test_script, secp.clone())?;
            assert_eq!(
                final_stack.read_ele_from_top(0).unwrap(),
                "1",
                "Signature verification failed for hash type: {:#04x}",
                hash_type
            );
        }

        Ok(())
    }
}
