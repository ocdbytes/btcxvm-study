mod basic_script_test {
    use crate::{execute_code, stack::Stack};
    use rstest::rstest;
    use secp256k1::{Message, Secp256k1, SecretKey};
    use sha2::{Digest, Sha256};

    #[rstest]
    fn test_p2sh() -> color_eyre::Result<()> {
        let secp = Secp256k1::new();
        let btc_address = "0227352074b4577c5dc4d56c87aada5c1ccb898cf4ac4d9ad71b4db11aa40c464e";
        let btc_priv_key = "ddbc8c57b741d82b5740252b2544de950ffb76aab68e663d356c55ee87ddcde5";
        let pub_key_hash = "455884c3c2f8229e74331f86454f46157a8df0a0";

        let private_key_bytes = hex::decode(btc_priv_key)?;
        let secret_key = SecretKey::from_slice(&private_key_bytes)?;
        let public_key = secret_key.public_key(&secp).to_string();
        // Extra check !
        assert_eq!(btc_address, public_key);

        let reedem_script: Vec<&str> =
            vec![&public_key, "OP_HASH160", pub_key_hash, "OP_EQUALVERIFY"];
        let message = create_message(reedem_script);
        let signature = secp.sign_ecdsa(&message, &secret_key).to_string();

        // Bee VM P2SH script version :
        // ----------------------------
        // <public key>
        // OP_HASH160
        // <public key hash>
        // OP_EQUALVERIFY
        // <public key>
        // <signature of hash of redeem script>
        // OP_CHECKSIG
        let vm_input_state: Vec<String> = vec![
            public_key.clone(), // public key
            "OP_HASH160".to_string(),
            pub_key_hash.to_string(), // public key hash
            "OP_EQUALVERIFY".to_string(),
            public_key, // public key
            signature,  // signature
            "OP_CHECKSIG".to_string(),
        ];

        let exec = execute_code(vm_input_state, secp);
        assert_eq!(exec.is_ok(), true);
        let (main_stack, _) = exec.unwrap();
        assert_eq!(main_stack, Stack::stack_from(vec!["1".to_string()]));
        Ok(())
    }

    #[rstest]
    fn test_p2pkh() -> color_eyre::Result<()> {
        let secp = Secp256k1::new();
        let btc_address = "0227352074b4577c5dc4d56c87aada5c1ccb898cf4ac4d9ad71b4db11aa40c464e";
        let btc_priv_key = "ddbc8c57b741d82b5740252b2544de950ffb76aab68e663d356c55ee87ddcde5";
        let pub_key_hash = "455884c3c2f8229e74331f86454f46157a8df0a0";

        let private_key_bytes = hex::decode(btc_priv_key)?;
        let secret_key = SecretKey::from_slice(&private_key_bytes)?;
        let public_key = secret_key.public_key(&secp).to_string();
        // Extra check !
        assert_eq!(btc_address, public_key);

        // Bee VM P2PKH script version :
        // ------------------------------------
        // <public key>
        // OP_DUP
        // OP_HASH160
        // <public key hash>
        // OP_EQUALVERIFY
        // OP_CHECKSIG
        // ------------------------------------

        let vm_input_state: Vec<String> = vec![
            public_key.clone(), // public key
            "OP_DUP".to_string(),
            "OP_HASH160".to_string(),
            pub_key_hash.to_string(), // public key hash
            "OP_EQUALVERIFY".to_string(),
            "OP_CHECKSIG".to_string(),
        ];

        let exec = execute_code(vm_input_state, secp);
        assert_eq!(exec.is_ok(), true);
        let (main_stack, _) = exec.unwrap();
        assert_eq!(main_stack, Stack::stack_from(vec!["1".to_string()]));
        Ok(())
    }

    fn create_message(script: Vec<&str>) -> Message {
        let mut hasher = Sha256::new();
        for op in script {
            hasher.update(op.as_bytes());
        }
        let script_hash = hasher.finalize();
        Message::from_digest_slice(&script_hash).expect("32 bytes")
    }
}
