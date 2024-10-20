use crate::errors::OpCodeErrors;
use crate::stack::Stack;
use secp256k1::ecdsa::Signature;
use secp256k1::{Message, PublicKey, Secp256k1};
use sha2::{Digest, Sha256};
use std::str::FromStr;

pub fn op_checkmultisig(
    stack: &mut Stack,
    script: &[String],
    secp: &Secp256k1<secp256k1::All>,
) -> Result<(), OpCodeErrors> {
    if stack.elements.len() < 4 {
        return Err(OpCodeErrors::MissingValues(
            "At least 4 values needed for this operation".to_string(),
        ));
    }

    let n_pubkeys = stack
        .pop_from_top()
        .unwrap()
        .parse::<usize>()
        .map_err(|_| OpCodeErrors::MissingValues("Invalid number of public keys".to_string()))?;

    if stack.elements.len() < n_pubkeys + 2 {
        return Err(OpCodeErrors::MissingValues(
            "Not enough values for the specified number of public keys".to_string(),
        ));
    }

    // Pop public keys
    let mut pubkeys = Vec::with_capacity(n_pubkeys);
    for _ in 0..n_pubkeys {
        let pubkey_str = stack.pop_from_top().unwrap();
        let pubkey = PublicKey::from_str(&pubkey_str)
            .map_err(|_| OpCodeErrors::InvalidValue("Invalid public key".to_string()))?;
        pubkeys.push(pubkey);
    }

    // Pop the number of required signatures
    let required_sigs = stack
        .pop_from_top()
        .unwrap()
        .parse::<usize>()
        .map_err(|_| {
            OpCodeErrors::InvalidValue("Invalid number of required signatures".to_string())
        })?;

    // Check if the required signatures is not greater than the number of public keys
    if required_sigs > n_pubkeys {
        return Err(OpCodeErrors::InvalidValue(
            "Required signatures cannot be greater than the number of public keys".to_string(),
        ));
    }

    // Check if there are enough elements for the signatures
    if stack.elements.len() < required_sigs + 1 {
        return Err(OpCodeErrors::MissingValues(
            "Not enough values for the specified number of signatures".to_string(),
        ));
    }

    // Pop signatures
    let mut signatures = Vec::with_capacity(required_sigs);
    for _ in 0..required_sigs {
        let sig_str = stack.pop_from_top().unwrap();
        let sig = Signature::from_str(&sig_str)
            .map_err(|_| OpCodeErrors::InvalidValue("Invalid signature".to_string()))?;
        signatures.push(sig);
    }

    // Pop the dummy value (Bitcoin's off-by-one error)
    stack.pop_from_top();

    // Get the script to hash (everything before OP_CHECKMULTISIG)
    let first_checkmultisig_occurrence = get_first_occurrence_of_checkmultisig(script);
    let script_to_hash: &[String] =
        &script[..first_checkmultisig_occurrence - (required_sigs + n_pubkeys + 3)];

    // Hash the script
    let mut hasher = Sha256::new();
    for op in script_to_hash {
        hasher.update(op.as_bytes());
    }
    let script_hash = hasher.finalize();

    // Create the message from the script hash
    let message = Message::from_digest_slice(&script_hash).expect("Invalid message");

    // Verify signatures
    let mut valid_sigs = 0;
    let mut pubkey_index = 0;

    for sig in signatures.iter() {
        while pubkey_index < pubkeys.len() {
            if secp
                .verify_ecdsa(&message, sig, &pubkeys[pubkey_index])
                .is_ok()
            {
                valid_sigs += 1;
                pubkey_index += 1;
                break;
            }
            pubkey_index += 1;
        }
    }

    // Push the result to the stack
    stack.push_to_top(if valid_sigs == required_sigs {
        "1".to_string()
    } else {
        "0".to_string()
    });

    Ok(())
}

/// Function to get first occurrence of OP_CHECKMULTISIG.
fn get_first_occurrence_of_checkmultisig(script: &[String]) -> usize {
    for (index, i) in script.iter().enumerate() {
        if i == "OP_CHECKMULTISIG" {
            return index;
        }
    }
    0
}

#[cfg(test)]
mod check_multisig_test {
    use crate::stack::executor::execute_code;
    use rstest::rstest;
    use secp256k1::{All, Message, Secp256k1, SecretKey};
    use sha2::{Digest, Sha256};

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
            // Multisig verification will go here
        ];

        (script, secret_keys, public_keys, secp)
    }

    #[rstest]
    fn test_op_checkmultisig_with_codeseparator_valid_signatures() -> color_eyre::Result<()> {
        let (mut script, secret_keys, public_keys, secp) = create_test_script();

        let sig_script = vec!["3".to_string(), "OP_EQUAL".to_string()];
        let message = create_message(&sig_script);

        let signature1 = secp.sign_ecdsa(&message, &secret_keys[0]);
        let signature2 = secp.sign_ecdsa(&message, &secret_keys[1]);

        script.push("0".to_string()); // Dummy value for off-by-one error
        script.push(signature1.to_string());
        script.push(signature2.to_string());
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

        let sig_script = vec!["3".to_string(), "OP_EQUAL".to_string()];
        let message = create_message(&sig_script);

        // Create one valid signature and one invalid signature
        let valid_signature = secp.sign_ecdsa(&message, &secret_keys[0]);
        let invalid_signature =
            secp.sign_ecdsa(&Message::from_digest_slice(&[0; 32])?, &secret_keys[1]);

        script.push("0".to_string()); // Dummy value for off-by-one error
        script.push(valid_signature.to_string());
        script.push(invalid_signature.to_string());
        script.push("2".to_string()); // Number of required signatures
        script.extend(public_keys);
        script.push("3".to_string()); // Number of public keys
        script.push("OP_CHECKMULTISIG".to_string());

        let (final_stack, _) = execute_code(script, secp)?;
        assert_eq!(final_stack.read_ele_from_top(0).unwrap(), "0");

        Ok(())
    }

    #[rstest]
    fn test_op_checkmultisig_without_codeseparator() -> color_eyre::Result<()> {
        let secp = Secp256k1::new();

        // Generate three key pairs
        let secret_keys: Vec<SecretKey> = (0..3)
            .map(|i| SecretKey::from_slice(&[0xcd + i as u8; 32]).unwrap())
            .collect();
        let public_keys: Vec<String> = secret_keys
            .iter()
            .map(|sk| sk.public_key(&secp).to_string())
            .collect();

        // Create a simple script without OP_CODESEPARATOR
        let mut script = vec![
            "2".to_string(),
            "3".to_string(),
            "OP_ADD".to_string(),
            "5".to_string(),
            "OP_EQUAL".to_string(),
        ];

        // Sign the script hash
        let message = create_message(&script);
        let signature1 = secp.sign_ecdsa(&message, &secret_keys[0]);
        let signature2 = secp.sign_ecdsa(&message, &secret_keys[1]);

        script.push("0".to_string()); // Dummy value for off-by-one error
        script.push(signature1.to_string());
        script.push(signature2.to_string());
        script.push("2".to_string()); // Number of required signatures
        script.extend(public_keys);
        script.push("3".to_string()); // Number of public keys
        script.push("OP_CHECKMULTISIG".to_string());

        // Execute the script
        let (final_stack, _) = execute_code(script, secp)?;

        // Check the result
        assert_eq!(final_stack.read_ele_from_top(0).unwrap(), "1");

        Ok(())
    }

    #[rstest]
    fn test_op_checkmultisig_insufficient_signatures() -> color_eyre::Result<()> {
        let (mut script, _, public_keys, secp) = create_test_script();

        // Add multisig data to the script with insufficient signatures
        script.push("0".to_string()); // Dummy value for off-by-one error
        script.push("some_signature".to_string()); // Only one signature
        script.push("2".to_string()); // Number of required signatures
        script.extend(public_keys);
        script.push("3".to_string()); // Number of public keys
        script.push("OP_CHECKMULTISIG".to_string());

        let result = execute_code(script, secp);
        assert!(result.is_err());

        Ok(())
    }

    // Helper function
    fn create_message(script: &[String]) -> Message {
        let mut hasher = Sha256::new();
        for op in script {
            hasher.update(op.as_bytes());
        }
        let script_hash = hasher.finalize();
        Message::from_digest_slice(&script_hash).expect("32 bytes")
    }
}
