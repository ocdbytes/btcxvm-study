pub mod check_sig;
pub mod op_check_multi_sig;
pub mod ripe_md_160;
pub mod sha_1;
pub mod sha_256;

use crate::errors::OpCodeErrors;
use sha2::{Digest, Sha256};

pub const SIGHASH_ALL: u8 = 0x01;
pub const SIGHASH_NONE: u8 = 0x02;
pub const SIGHASH_SINGLE: u8 = 0x03;
pub const SIGHASH_ANYONECANPAY: u8 = 0x80;

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
