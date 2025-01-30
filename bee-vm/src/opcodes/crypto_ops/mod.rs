pub mod check_sig;
pub mod op_check_multi_sig;
pub mod ripe_md_160;
pub mod sha_1;
pub mod sha_256;

use crate::errors::OpCodeErrors;
use secp256k1::ecdsa::Signature;
use sha2::{Digest, Sha256};

pub const SIGHASH_ALL: u8 = 0x01;
pub const SIGHASH_NONE: u8 = 0x02;
pub const SIGHASH_SINGLE: u8 = 0x03;
pub const SIGHASH_ANYONECANPAY: u8 = 0x80;

pub fn parse_signature(signature_hex: &str) -> Result<(Signature, u8), OpCodeErrors> {
    // Convert hex string to bytes
    let sig_bytes = hex::decode(signature_hex).map_err(|_| OpCodeErrors::InvalidSignature)?;

    if sig_bytes.is_empty() {
        return Err(OpCodeErrors::InvalidSignature);
    }

    // The last byte is the hash type
    let hash_type = *sig_bytes.last().ok_or(OpCodeErrors::InvalidSignature)?;

    // Everything except the last byte is the DER signature
    let der_sig = &sig_bytes[..sig_bytes.len() - 1];

    // Parse the DER-formatted signature
    let signature = Signature::from_der(der_sig).map_err(|_| OpCodeErrors::InvalidSignature)?;

    Ok((signature, hash_type))
}

pub fn hash_script(script: Vec<String>) -> Result<[u8; 32], OpCodeErrors> {
    let mut hasher = Sha256::new();

    // Convert script operations to bytes and hash them
    for op in script {
        hasher.update(op.as_bytes());
    }

    // Double SHA256 as per Bitcoin protocol
    let first_hash = hasher.finalize();
    let mut hasher = Sha256::new();
    hasher.update(first_hash);

    let final_hash = hasher.finalize();
    let mut result = [0u8; 32];
    result.copy_from_slice(&final_hash);

    Ok(result)
}

/// Creates a DER-encoded signature with a hash type byte
pub fn create_der_signature(signature: &secp256k1::ecdsa::Signature, hash_type: u8) -> String {
    // Get the raw signature bytes (r,s values)
    let sig_bytes = signature.serialize_compact();

    // Create DER encoding:
    // Format: 0x30 [total-length] 0x02 [r-length] [r] 0x02 [s-length] [s] [hash-type]
    let r = &sig_bytes[..32];
    let s = &sig_bytes[32..];

    // Remove leading zeros and ensure positive numbers
    let r = remove_leading_zeros(r);
    let s = remove_leading_zeros(s);

    // Calculate lengths
    let r_len = r.len();
    let s_len = s.len();
    let total_len = r_len + s_len + 4; // +4 for the 0x02 markers and length bytes

    // Build the DER signature
    let mut der_sig = Vec::with_capacity(total_len + 3); // +3 for sequence marker, length, and hash type
    der_sig.push(0x30); // Sequence marker
    der_sig.push(total_len as u8);

    // Add r value
    der_sig.push(0x02); // Integer marker
    der_sig.push(r_len as u8);
    der_sig.extend_from_slice(r.as_slice());

    // Add s value
    der_sig.push(0x02); // Integer marker
    der_sig.push(s_len as u8);
    der_sig.extend_from_slice(s.as_slice());

    // Add hash type
    der_sig.push(hash_type);

    // Convert to hex string
    hex::encode(der_sig)
}

/// Helper function to remove leading zeros from signature components
fn remove_leading_zeros(data: &[u8]) -> Vec<u8> {
    let mut result = data.to_vec();
    while result.len() > 1 && result[0] == 0 && result[1] < 0x80 {
        result.remove(0);
    }
    // Ensure number is positive (add 0x00 if highest bit is set)
    if result[0] >= 0x80 {
        result.insert(0, 0);
    }
    result
}
