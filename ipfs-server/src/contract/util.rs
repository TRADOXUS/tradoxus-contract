use k256::ecdsa::VerifyingKey;
use sha3::{Digest, Keccak256};

/// Returns compressed public key (in hexstring, without `0x`).
pub fn compress_public_key(pk: &VerifyingKey) -> String {
    let encoded = pk.to_encoded_point(true); // compressed = true
    hex::encode(encoded.as_bytes())
}

/// Serialize uncompressed public key (in hexstring, without `0x`).
pub fn hex_public_key(pk: &VerifyingKey) -> String {
    let encoded = pk.to_encoded_point(false); // uncompressed = false
    hex::encode(encoded.as_bytes())
}

pub fn hash_keccak256(message: &str) -> [u8; 32] {
    let mut hasher = Keccak256::default();
    hasher.update(message);
    hasher.finalize().into()
}
