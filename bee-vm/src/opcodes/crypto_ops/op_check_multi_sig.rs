use crate::errors::OpCodeErrors;
use crate::opcodes::crypto_ops::hash_script;
use crate::stack::Stack;
use k256::ecdsa::signature::Verifier;
use k256::ecdsa::Signature;
use k256::ecdsa::VerifyingKey;

pub fn op_checkmultisig(stack: &mut Stack, script: &[String]) -> Result<(), OpCodeErrors> {
    // Ensure minimum stack size (n_pubkeys + pubkeys + m_sigs + sigs + dummy)
    if stack.elements.len() < 4 {
        return Err(OpCodeErrors::MissingValues(
            "At least 4 values needed for this operation".to_string(),
        ));
    }

    // Get number of public keys (n)
    let n_pubkeys = stack
        .pop_from_top()
        .unwrap()
        .parse::<usize>()
        .map_err(|_| OpCodeErrors::InvalidValue("Invalid number of public keys".to_string()))?;

    // Validate stack has enough elements for public keys
    if stack.elements.len() < n_pubkeys + 2 {
        return Err(OpCodeErrors::MissingValues(
            "Not enough values for the specified number of public keys".to_string(),
        ));
    }

    // Pop and parse public keys
    let mut pubkeys = Vec::with_capacity(n_pubkeys);
    for _ in 0..n_pubkeys {
        let pubkey_str = stack.pop_from_top().unwrap();
        let public_key: VerifyingKey = VerifyingKey::from_sec1_bytes(
            &hex::decode(pubkey_str).map_err(|_| OpCodeErrors::InvalidPublicKey)?,
        )
        .map_err(|_| OpCodeErrors::InvalidPublicKey)?;
        pubkeys.push(public_key);
    }

    // Get required number of signatures (m)
    let required_sigs = stack
        .pop_from_top()
        .unwrap()
        .parse::<usize>()
        .map_err(|_| {
            OpCodeErrors::InvalidValue("Invalid number of required signatures".to_string())
        })?;

    // Validate m <= n
    if required_sigs > n_pubkeys {
        return Err(OpCodeErrors::InvalidValue(
            "Required signatures cannot be greater than the number of public keys".to_string(),
        ));
    }

    // Check stack size for signatures
    if stack.elements.len() < required_sigs + 1 {
        return Err(OpCodeErrors::MissingValues(
            "Not enough values for the specified number of signatures".to_string(),
        ));
    }

    // Pop and parse signatures with their hash types
    let mut signatures = Vec::with_capacity(required_sigs);
    for _ in 0..required_sigs {
        let sig_hex = stack.pop_from_top().unwrap();
        let signature: Signature = Signature::from_slice(
            &hex::decode(sig_hex).map_err(|_| OpCodeErrors::InvalidSignature)?,
        )
        .map_err(|_| OpCodeErrors::InvalidSignature)?;
        signatures.push(hex::encode(signature.to_bytes()));
    }

    // Pop the dummy value (Bitcoin consensus bug feature)
    stack.pop_from_top();

    // Create script hash for signature verification
    let mut script_for_hash = script.to_vec();

    // Remove all signatures from the script before hashing
    for sig in &signatures {
        script_for_hash.retain(|op| !op.contains(&sig.to_string()));
    }
    let script_hash = hash_script(script_for_hash)?;

    // ========================
    // Verify signatures
    // ========================

    let mut valid_sigs = 0;

    'sig_loop: for sig in signatures.iter() {
        for pub_key in pubkeys.iter().take(n_pubkeys) {
            if pub_key
                .verify(
                    script_hash.as_slice(),
                    &Signature::from_slice(
                        &hex::decode(sig).map_err(|_| OpCodeErrors::InvalidSignature)?,
                    )
                    .map_err(|_| OpCodeErrors::InvalidSignature)?,
                )
                .is_ok()
            {
                valid_sigs += 1;
                continue 'sig_loop;
            }
        }
    }

    // Push result to stack
    stack.push_to_top(if valid_sigs >= required_sigs {
        "1".to_string()
    } else {
        "0".to_string()
    });

    Ok(())
}

#[cfg(test)]
mod check_multisig_test {
    use crate::opcodes::crypto_ops::hash_script;
    use crate::stack::executor::execute_code;
    use hex;
    use k256::ecdsa::signature::Signer;
    use k256::ecdsa::Signature;
    use k256::ecdsa::SigningKey;
    use k256::elliptic_curve::rand_core::OsRng;
    use rstest::rstest;

    /// Creates a test environment with necessary components:
    /// - A basic script with arithmetic operations
    /// - Three key pairs for testing different signature combinations
    fn create_test_script() -> (Vec<String>, Vec<SigningKey>, Vec<String>) {
        // Generate three key pairs
        let secret_keys: Vec<SigningKey> =
            (0..3).map(|_i| SigningKey::random(&mut OsRng)).collect();
        let public_keys: Vec<String> = secret_keys
            .iter()
            .map(|sk| hex::encode(sk.verifying_key().to_sec1_bytes()))
            .collect();

        let script = vec![
            "1".to_string(),
            "2".to_string(),
            "OP_ADD".to_string(),
            "OP_CODESEPARATOR".to_string(),
            "3".to_string(),
            "OP_EQUAL".to_string(),
        ];

        (script, secret_keys, public_keys)
    }

    #[rstest]
    fn test_op_checkmultisig_with_codeseparator_valid_signatures() -> color_eyre::Result<()> {
        let (mut script, signing_keys, public_keys) = create_test_script();

        // Create message from the part of script after OP_CODESEPARATOR
        let mut sig_script = vec![
            "3".to_string(),
            "OP_EQUAL".to_string(),
            "0".to_string(),
            "2".to_string(),
        ];
        sig_script.extend(public_keys.clone());
        sig_script.extend(vec!["3".to_string(), "OP_CHECKMULTISIG".to_string()]);

        let message = hash_script(sig_script)?;

        // Create properly formatted signatures with hash type
        let signature1: Signature = signing_keys[0].sign(message.as_slice());
        let signature2: Signature = signing_keys[1].sign(message.as_slice());

        // Build the complete script
        script.push("0".to_string()); // Dummy value for off-by-one error
        script.push(hex::encode(signature1.to_bytes()));
        script.push(hex::encode(signature2.to_bytes()));
        script.push("2".to_string()); // Number of required signatures
        script.extend(public_keys);
        script.push("3".to_string()); // Number of public keys
        script.push("OP_CHECKMULTISIG".to_string());

        let (final_stack, _) = execute_code(script)?;
        assert_eq!(final_stack.read_ele_from_top(0).unwrap(), "1");
        Ok(())
    }

    #[rstest]
    fn test_op_checkmultisig_with_codeseparator_invalid_signature() -> color_eyre::Result<()> {
        let (mut script, signing_keys, public_keys) = create_test_script();

        let mut sig_script = vec![
            "3".to_string(),
            "OP_EQUAL".to_string(),
            "0".to_string(),
            "2".to_string(),
        ];
        sig_script.extend(public_keys.clone());
        sig_script.extend(vec!["3".to_string(), "OP_CHECKMULTISIG".to_string()]);

        let invalid_message = &[0; 32];

        // Create one valid and one invalid signature, both properly formatted
        let valid_signature: Signature = signing_keys[0].sign(invalid_message.as_slice());
        let invalid_signature: Signature = signing_keys[1].sign(invalid_message.as_slice());

        script.push("0".to_string());
        script.push(hex::encode(valid_signature.to_bytes()));
        script.push(hex::encode(invalid_signature.to_bytes()));
        script.push("2".to_string());
        script.extend(public_keys);
        script.push("3".to_string());
        script.push("OP_CHECKMULTISIG".to_string());

        let (final_stack, _) = execute_code(script)?;
        assert_eq!(final_stack.read_ele_from_top(0).unwrap(), "0");
        Ok(())
    }

    #[rstest]
    fn test_op_checkmultisig_without_codeseparator() -> color_eyre::Result<()> {
        // Generate key pairs
        let secret_keys: Vec<SigningKey> =
            (0..3).map(|_i| SigningKey::random(&mut OsRng)).collect();
        let public_keys: Vec<String> = secret_keys
            .iter()
            .map(|sk| hex::encode(sk.verifying_key().to_sec1_bytes()))
            .collect();

        // Create script without OP_CODESEPARATOR
        let mut script = vec![
            "2".to_string(),
            "3".to_string(),
            "OP_ADD".to_string(),
            "5".to_string(),
            "OP_EQUAL".to_string(),
        ];

        let mut sign_script = script.clone();
        sign_script.push("0".to_string());
        sign_script.push("2".to_string());
        sign_script.extend(public_keys.clone());
        sign_script.extend(vec!["3".to_string(), "OP_CHECKMULTISIG".to_string()]);

        // Create properly formatted signatures
        let message = hash_script(sign_script)?;
        let signature1: Signature = secret_keys[0].sign(message.as_slice());
        let signature2: Signature = secret_keys[1].sign(message.as_slice());

        script.push("0".to_string());
        script.push(hex::encode(signature1.to_bytes()));
        script.push(hex::encode(signature2.to_bytes()));
        script.push("2".to_string());
        script.extend(public_keys);
        script.push("3".to_string());
        script.push("OP_CHECKMULTISIG".to_string());

        let (final_stack, _) = execute_code(script)?;
        assert_eq!(final_stack.read_ele_from_top(0).unwrap(), "1");
        Ok(())
    }

    #[rstest]
    fn test_op_checkmultisig_insufficient_signatures() -> color_eyre::Result<()> {
        let (mut script, secret_keys, public_keys) = create_test_script();

        // Create a properly formatted signature
        let message = hash_script(script.clone())?;
        let signature1: Signature = secret_keys[0].sign(message.as_slice());

        // Add multisig data with insufficient signatures
        script.push("0".to_string());
        script.push(hex::encode(signature1.to_bytes())); // Only one signature when two are required
        script.push("2".to_string());
        script.extend(public_keys);
        script.push("3".to_string());
        script.push("OP_CHECKMULTISIG".to_string());

        let result = execute_code(script);
        assert!(result.is_err());
        Ok(())
    }
}
