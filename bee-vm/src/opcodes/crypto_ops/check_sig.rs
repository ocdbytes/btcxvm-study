use crate::errors::OpCodeErrors;
use crate::stack::Stack;
use secp256k1::ecdsa::Signature;
use secp256k1::{Message, PublicKey, Secp256k1};
use sha2::{Digest, Sha256};
use std::str::FromStr;

pub fn op_checksig(
    stack: &mut Stack,
    script: &[String],
    secp: &Secp256k1<secp256k1::All>,
) -> Result<(), OpCodeErrors> {
    if stack.elements.len() < 2 {
        return Err(OpCodeErrors::MissingValues(
            "At least 2 values needed for this operation".to_string(),
        ));
    }

    let signature = stack.pop_from_top().unwrap();
    let public_key = stack.pop_from_top().unwrap();

    let public_key =
        PublicKey::from_str(&public_key).expect("Not able to decode Public key from the stack.");
    let signature =
        Signature::from_str(&signature).expect("Not able to decode Signature from the stack.");

    let first_check_sig_occurrence = get_first_occurrence_of_checksig(script);
    let script_to_hash: &[String] = &script[..first_check_sig_occurrence - 2];

    let mut hasher = Sha256::new();

    for op in script_to_hash {
        hasher.update(op.as_bytes());
    }
    let script_hash = hasher.finalize();

    let message = Message::from_digest_slice(&script_hash).expect("");

    let result = secp.verify_ecdsa(&message, &signature, &public_key).is_ok();

    stack.push_to_top(if result {
        "1".to_string()
    } else {
        "0".to_string()
    });

    Ok(())
}

/// Function to get first occurrence of OP_CHECKSIG.
///
/// It is used to get the index where OP_CHECKSIG first
/// occurs.
fn get_first_occurrence_of_checksig(script: &[String]) -> usize {
    for (index, i) in script.iter().enumerate() {
        if i == "OP_CHECKSIG" {
            return index;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::stack::executor::execute_code;
    use rstest::rstest;
    use secp256k1::{All, Message, Secp256k1, SecretKey};
    use sha2::{Digest, Sha256};

    fn create_test_script() -> (Vec<String>, SecretKey, String, Secp256k1<All>) {
        let secp = Secp256k1::new();

        // Generate a key pair
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
            // First signature and verification will go here
            "OP_CODESEPARATOR".to_string(),
            "4".to_string(),
            "5".to_string(),
            "OP_ADD".to_string(),
            "OP_CODESEPARATOR".to_string(),
            "9".to_string(),
            "OP_EQUAL".to_string(),
            // Second signature and verification will go here
        ];

        (script, secret_key, public_key.to_string(), secp)
    }

    #[rstest]
    fn test_op_checksig_with_codeseparator_valid_signature() -> color_eyre::Result<()> {
        let (mut script, secret_key, public_key, secp) = create_test_script();

        let sig_script1 = vec!["3".to_string(), "OP_EQUAL".to_string()];
        let message1 = create_message(&sig_script1);
        let signature1 = secp.sign_ecdsa(&message1, &secret_key);

        // Add signature and public key to the script
        script.push(public_key.clone());
        script.push(signature1.to_string());
        script.push("OP_CHECKSIG".to_string());

        let sig_script2 = vec!["9".to_string(), "OP_EQUAL".to_string()];
        let message2 = create_message(&sig_script2);
        let signature2 = secp.sign_ecdsa(&message2, &secret_key);

        // Add signature and public key to the script
        script.push(public_key);
        script.push(signature2.to_string());
        script.push("OP_CHECKSIG".to_string());

        // Execute the script
        let (final_stack, _) = execute_code(script, secp)?;

        // Check the result
        // The signature should be valid
        assert_eq!(final_stack.read_ele_from_top(0).unwrap(), "1");

        Ok(())
    }

    #[rstest]
    fn test_op_checksig_with_codeseparator_invalid_signature() -> color_eyre::Result<()> {
        let (mut script, secret_key, public_key, secp) = create_test_script();

        let message = Message::from_digest_slice(&[0; 32])?;
        let invalid_signature = secp.sign_ecdsa(&message, &secret_key);

        // Add invalid signature and public key to the script
        script.push(public_key);
        script.push(invalid_signature.to_string());
        script.push("OP_CHECKSIG".to_string());

        // Execute the script
        let (final_stack, _) = execute_code(script, secp)?;

        // Check the result
        // The signature should be invalid
        assert_eq!(final_stack.read_ele_from_top(0).unwrap(), "0");

        Ok(())
    }

    #[rstest]
    fn test_op_checksig_without_codeseparator() -> color_eyre::Result<()> {
        let secp = Secp256k1::new();

        // Generate a key pair
        let secret_key = SecretKey::from_slice(&[0xcd; 32]).unwrap();
        let public_key = secret_key.public_key(&secp);

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
        let signature = secp.sign_ecdsa(&message, &secret_key);

        // Add signature, public key, and OP_CHECKSIG to the script
        script.push(public_key.to_string());
        script.push(signature.to_string());
        script.push("OP_CHECKSIG".to_string());

        // Execute the script
        let (final_stack, _) = execute_code(script.clone(), secp)?;

        // Check the result
        // The signature should be valid
        assert_eq!(final_stack.read_ele_from_top(0).unwrap(), "1");

        Ok(())
    }

    // ==================================
    // Helper Functions
    // ==================================

    fn create_message(script: &[String]) -> Message {
        let mut hasher = Sha256::new();
        for op in script {
            hasher.update(op.as_bytes());
        }
        let script_hash = hasher.finalize();
        Message::from_digest_slice(&script_hash).expect("32 bytes")
    }
}
