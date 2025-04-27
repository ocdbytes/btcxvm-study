use crate::{execute_code, opcodes::crypto_ops::hash_script, stack::Stack};
use k256::ecdsa::signature::Signer;
use k256::ecdsa::Signature;
use k256::ecdsa::SigningKey;
use k256::elliptic_curve::rand_core::OsRng;
use ripemd::{Digest, Ripemd160};
use rstest::rstest;
use sha2::Sha256;

#[rstest]
fn test_p2sh() -> color_eyre::Result<()> {
    let signing_key: SigningKey = SigningKey::random(&mut OsRng);
    let verifying_key = signing_key.verifying_key().to_sec1_bytes();
    let pub_key_hash = hash_160(hex::encode(verifying_key.clone()));

    let reedem_script: Vec<String> = vec![
        hex::encode(verifying_key.clone()),
        "OP_HASH160".into(),
        pub_key_hash.clone(),
        "OP_EQUALVERIFY".into(),
        hex::encode(verifying_key.clone()),
        "OP_CHECKSIG".into(),
    ];
    let script_hash = hash_script(reedem_script)?;
    let signature: Signature = signing_key.sign(script_hash.as_slice());

    // Bee VM P2SH script version :
    // ----------------------------
    // <public key>
    // OP_HASH160
    // <public key hash>
    // OP_EQUALVERIFY
    // <public key>
    // <signature of hash of redeem script>
    // OP_CHECKSIG
    // ----------------------------

    let vm_input_state: Vec<String> = vec![
        hex::encode(verifying_key.clone()), // public key
        "OP_HASH160".to_string(),
        pub_key_hash.to_string(), // public key hash
        "OP_EQUALVERIFY".to_string(),
        hex::encode(verifying_key),        // public key
        hex::encode(signature.to_bytes()), // signature
        "OP_CHECKSIG".to_string(),
    ];

    let exec = execute_code(vm_input_state);
    assert!(exec.is_ok());
    let (main_stack, _) = exec.unwrap();
    assert_eq!(main_stack, Stack::stack_from(vec!["1".to_string()]));
    Ok(())
}

#[rstest]
fn test_p2pkh() -> color_eyre::Result<()> {
    let signing_key: SigningKey = SigningKey::random(&mut OsRng);
    let verifying_key = signing_key.verifying_key().to_sec1_bytes();
    let pub_key_hash = hash_160(hex::encode(verifying_key.clone()));

    let reedem_script = vec![
        hex::encode(verifying_key.clone()),
        "OP_DUP".into(),
        "OP_HASH160".into(),
        pub_key_hash.clone(),
        "OP_EQUALVERIFY".into(),
        hex::encode(verifying_key.clone()),
        "OP_CHECKSIG".into(),
    ];
    let script_hash = hash_script(reedem_script)?;
    let signature: Signature = signing_key.sign(script_hash.as_slice());

    // Bee VM P2PKH script version :
    // ------------------------------------
    // <public key>
    // OP_DUP
    // OP_HASH160
    // <public key hash>
    // OP_EQUALVERIFY
    // <public key>
    // <signature of the above script>
    // OP_CHECKSIG
    // ------------------------------------

    let vm_input_state: Vec<String> = vec![
        hex::encode(verifying_key.clone()), // public key
        "OP_DUP".to_string(),
        "OP_HASH160".to_string(),
        pub_key_hash.to_string(), // public key hash
        "OP_EQUALVERIFY".to_string(),
        hex::encode(verifying_key),        // public key
        hex::encode(signature.to_bytes()), // signature
        "OP_CHECKSIG".to_string(),
    ];

    let exec = execute_code(vm_input_state);
    assert!(exec.is_ok());
    let (mut main_stack, _) = exec.unwrap();
    assert_eq!(main_stack.pop_from_top(), Some("1".to_string()));
    Ok(())
}

fn hash_160(data: String) -> String {
    // sha 256
    let hash = Sha256::digest(data);
    // ripe md 160
    let mut hasher = Ripemd160::new();
    hasher.update(hash);
    let hasher_result = hasher.finalize();
    hex::encode(hasher_result)
}
