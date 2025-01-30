use crate::errors::OpCodeErrors;
use crate::opcodes::crypto_ops::{hash_script, parse_signature};
use crate::stack::Stack;
use secp256k1::{Message, PublicKey, Secp256k1};
use std::str::FromStr;

pub fn op_checkmultisig(
    stack: &mut Stack,
    script: &[String],
    secp: &Secp256k1<secp256k1::All>,
) -> Result<(), OpCodeErrors> {
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
        let pubkey =
            PublicKey::from_str(&pubkey_str).map_err(|_| OpCodeErrors::InvalidPublicKey)?;
        pubkeys.push(pubkey);
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
        let (signature, _hash_type) = parse_signature(&sig_hex)?;
        signatures.push(signature);
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

    // Create message from script hash
    let message =
        Message::from_digest_slice(&script_hash).map_err(|_| OpCodeErrors::MessageCreationError)?;

    // ========================
    // Verify signatures
    // ========================

    let mut valid_sigs = 0;

    'sig_loop: for sig in signatures.iter() {
        for pub_key in pubkeys.iter().take(n_pubkeys) {
            if secp.verify_ecdsa(&message, sig, pub_key).is_ok() {
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
    use crate::stack::executor::execute_code;
    use hex;
    use rstest::rstest;
    use secp256k1::{All, Message, Secp256k1, SecretKey};
    use sha2::{Digest, Sha256};

    /// Creates a test environment with necessary components:
    /// - A basic script with arithmetic operations
    /// - Three key pairs for testing different signature combinations
    /// - Secp256k1 context for cryptographic operations
    fn create_test_script() -> (Vec<String>, Vec<SecretKey>, Vec<String>, Secp256k1<All>) {
        let secp = Secp256k1::new();

        // Generate three key pairs
        let secret_keys: Vec<SecretKey> = (0..3)
            .map(|i| SecretKey::from_slice(&[0xcd + i as u8; 32]).unwrap())
            .collect();
        let public_keys: Vec<String> = secret_keys
            .iter()
            .map(|sk| sk.public_key(&secp).to_string())
            .collect();

        let script = vec![
            "1".to_string(),
            "2".to_string(),
            "OP_ADD".to_string(),
            "OP_CODESEPARATOR".to_string(),
            "3".to_string(),
            "OP_EQUAL".to_string(),
        ];

        (script, secret_keys, public_keys, secp)
    }

    /// Helper function to create a DER-encoded signature with hash type
    fn create_signature_with_hashtype(signature: &secp256k1::ecdsa::Signature) -> String {
        // Convert DER signature to bytes
        let der_sig = signature.serialize_der();

        // Append SIGHASH_ALL (0x01) hash type
        let mut sig_with_hashtype = der_sig.to_vec();
        sig_with_hashtype.push(0x01);

        // Convert to hex string
        hex::encode(sig_with_hashtype)
    }

    #[rstest]
    fn test_op_checkmultisig_with_codeseparator_valid_signatures() -> color_eyre::Result<()> {
        let (mut script, secret_keys, public_keys, secp) = create_test_script();

        // Create message from the part of script after OP_CODESEPARATOR
        let mut sig_script = vec![
            "3".to_string(),
            "OP_EQUAL".to_string(),
            "0".to_string(),
            "2".to_string(),
        ];
        sig_script.extend(public_keys.clone());
        sig_script.extend(vec!["3".to_string(), "OP_CHECKMULTISIG".to_string()]);

        let message = create_message(&sig_script);

        // Create properly formatted signatures with hash type
        let signature1 =
            create_signature_with_hashtype(&secp.sign_ecdsa(&message, &secret_keys[0]));
        let signature2 =
            create_signature_with_hashtype(&secp.sign_ecdsa(&message, &secret_keys[1]));

        // Build the complete script
        script.push("0".to_string()); // Dummy value for off-by-one error
        script.push(signature1);
        script.push(signature2);
        script.push("2".to_string()); // Number of required signatures
        script.extend(public_keys);
        script.push("3".to_string()); // Number of public keys
        script.push("OP_CHECKMULTISIG".to_string());

        let (final_stack, _) = execute_code(script, secp)?;
        assert_eq!(final_stack.read_ele_from_top(0).unwrap(), "1");
        Ok(())
    }

    #[rstest]
    fn test_op_checkmultisig_with_codeseparator_invalid_signature() -> color_eyre::Result<()> {
        let (mut script, secret_keys, public_keys, secp) = create_test_script();

        let mut sig_script = vec![
            "3".to_string(),
            "OP_EQUAL".to_string(),
            "0".to_string(),
            "2".to_string(),
        ];
        sig_script.extend(public_keys.clone());
        sig_script.extend(vec!["3".to_string(), "OP_CHECKMULTISIG".to_string()]);

        let message = create_message(&sig_script);
        let invalid_message = Message::from_digest_slice(&[0; 32])?;

        // Create one valid and one invalid signature, both properly formatted
        let valid_signature =
            create_signature_with_hashtype(&secp.sign_ecdsa(&message, &secret_keys[0]));
        let invalid_signature =
            create_signature_with_hashtype(&secp.sign_ecdsa(&invalid_message, &secret_keys[1]));

        script.push("0".to_string());
        script.push(valid_signature);
        script.push(invalid_signature);
        script.push("2".to_string());
        script.extend(public_keys);
        script.push("3".to_string());
        script.push("OP_CHECKMULTISIG".to_string());

        let (final_stack, _) = execute_code(script, secp)?;
        assert_eq!(final_stack.read_ele_from_top(0).unwrap(), "0");
        Ok(())
    }

    #[rstest]
    fn test_op_checkmultisig_without_codeseparator() -> color_eyre::Result<()> {
        let secp = Secp256k1::new();

        // Generate key pairs
        let secret_keys: Vec<SecretKey> = (0..3)
            .map(|i| SecretKey::from_slice(&[0xcd + i as u8; 32]).unwrap())
            .collect();
        let public_keys: Vec<String> = secret_keys
            .iter()
            .map(|sk| sk.public_key(&secp).to_string())
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
        let message = create_message(&sign_script);
        let signature1 =
            create_signature_with_hashtype(&secp.sign_ecdsa(&message, &secret_keys[0]));
        let signature2 =
            create_signature_with_hashtype(&secp.sign_ecdsa(&message, &secret_keys[1]));

        script.push("0".to_string());
        script.push(signature1);
        script.push(signature2);
        script.push("2".to_string());
        script.extend(public_keys);
        script.push("3".to_string());
        script.push("OP_CHECKMULTISIG".to_string());

        let (final_stack, _) = execute_code(script, secp)?;
        assert_eq!(final_stack.read_ele_from_top(0).unwrap(), "1");
        Ok(())
    }

    #[rstest]
    fn test_op_checkmultisig_insufficient_signatures() -> color_eyre::Result<()> {
        let (mut script, secret_keys, public_keys, secp) = create_test_script();

        // Create a properly formatted signature
        let message = create_message(&script);
        let signature = create_signature_with_hashtype(&secp.sign_ecdsa(&message, &secret_keys[0]));

        // Add multisig data with insufficient signatures
        script.push("0".to_string());
        script.push(signature); // Only one signature when two are required
        script.push("2".to_string());
        script.extend(public_keys);
        script.push("3".to_string());
        script.push("OP_CHECKMULTISIG".to_string());

        let result = execute_code(script, secp);
        assert!(result.is_err());
        Ok(())
    }

    /// Helper function to create a message hash from script
    fn create_message(script: &[String]) -> Message {
        let mut hasher = Sha256::new();
        for op in script {
            hasher.update(op.as_bytes());
        }
        let first_hash = hasher.finalize();

        // Double SHA256 as per Bitcoin protocol
        let mut hasher = Sha256::new();
        hasher.update(first_hash);
        let final_hash = hasher.finalize();

        Message::from_digest_slice(&final_hash).expect("32 bytes")
    }
}
