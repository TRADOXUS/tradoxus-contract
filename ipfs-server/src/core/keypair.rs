use crate::error::Error;

use thiserror::Error as StdError;

use k256::ecdsa::{RecoveryId, Signature, SigningKey, VerifyingKey};
use k256::elliptic_curve::rand_core::OsRng;
use k256::EncodedPoint;

use crate::utils::{compress_public_key, hash_keccak256};

#[derive(Debug, Clone, Copy, Eq, PartialEq, StdError)]
pub enum Secp256k1Error {
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Invalid signature format")]
    InvalidSignatureFormat,
    #[error("Missing secret key")]
    MissingSecretKey,
    #[error("Invalid public key")]
    InvalidPublicKey,
    #[error("Invalid secret key")]
    InvalidSecretKey,
    #[error("Invalid recovery ID")]
    InvalidRecoveryId,
    #[error("Invalid message")]
    InvalidMessage,
    #[error("Invalid input length")]
    InvalidInputLength,
    #[error("Tweak out of range")]
    TweakOutOfRange,
    #[error("Failed to sign")]
    SigningFailed,
    #[error("Invalid affine point")]
    InvalidAffine,
}

/// A wrapper for ECDSA keypairs using k256
pub struct Secp256k1KeyPair {
    pub public_key: VerifyingKey,
    pub secret_key: Option<SigningKey>,
}

impl std::fmt::Debug for Secp256k1KeyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.secret_key {
            Some(_) => f
                .debug_struct("Secp256k1KeyPair")
                .field("public_key", &compress_public_key(&self.public_key))
                .field("secret_key", &"OMIT".to_string())
                .finish(),
            None => f
                .debug_struct("Secp256k1KeyPair")
                .field("public_key", &self.public_key)
                .finish(),
        }
    }
}

impl Secp256k1KeyPair {
    /// Generate a new keypair (for testing/development)
    pub fn generate() -> Self {
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = VerifyingKey::from(&signing_key);

        Self {
            public_key: verifying_key,
            secret_key: Some(signing_key),
        }
    }

    /// Construct a public key from a hex string
    pub fn from_pubkey_hex(pubkey_hex: &str) -> Result<Self, Error> {
        let clean = pubkey_hex.strip_prefix("0x").unwrap_or(pubkey_hex);
        let bytes = hex::decode(clean)?;
        let point = EncodedPoint::from_bytes(&bytes)
            .map_err(|_| Error::CryptoError(Secp256k1Error::InvalidPublicKey))?;
        let public_key = VerifyingKey::from_encoded_point(&point)
            .map_err(|_| Error::CryptoError(Secp256k1Error::InvalidPublicKey))?;

        Ok(Self {
            public_key,
            secret_key: None,
        })
    }

    /// Sign a personal message with Ethereum prefix
    pub fn personal_sign(&self, message: &str) -> Result<Vec<u8>, Error> {
        let prefix = format!("\x19Traxodus Signed Message:\n{}{}", message.len(), message);
        self.sign_hashed(&prefix)
    }

    /// Sign a message with keccak256 hash
    pub fn sign_hashed(&self, message: &str) -> Result<Vec<u8>, Error> {
        let digest = hash_keccak256(message);
        let signing_key = self
            .secret_key
            .as_ref()
            .ok_or(Error::CryptoError(Secp256k1Error::MissingSecretKey))?;

        let sig = signing_key
            .sign_prehash_recoverable(&digest)
            .map_err(|_| Error::CryptoError(Secp256k1Error::SigningFailed))?;

        let mut sig_bytes = [0u8; 65];
        sig_bytes[..64].copy_from_slice(&sig.0.to_vec()); // r + s
        sig_bytes[64] = sig.1.into(); // v

        Ok(sig_bytes.into())
    }

    /// Recover public key from personal signed message
    pub fn recover_from_personal_signature(
        sig: &[u8],
        message: &str,
    ) -> Result<VerifyingKey, Error> {
        if sig.len() != 65 {
            return Err(Error::CryptoError(Secp256k1Error::InvalidSignatureFormat));
        }

        let prefix = format!("\x19Traxodus Signed Message:\n{}{}", message.len(), message);
        let digest = hash_keccak256(&prefix);

        let r_s = &sig[..64];
        let recovery_byte = sig[64];
        let recovery_id = match recovery_byte {
            0 | 1 => recovery_byte,
            27 | 28 => recovery_byte - 27,
            _ => return Err(Error::CryptoError(Secp256k1Error::InvalidRecoveryId)),
        };

        let recovery_id_final = RecoveryId::try_from(recovery_id as u8).unwrap();

        let signature = Signature::from_bytes(r_s.into()).expect("invalid r+s");

        let verifying_key =
            VerifyingKey::recover_from_prehash(&digest, &signature, recovery_id_final).unwrap();

        Ok(verifying_key)
    }
}
