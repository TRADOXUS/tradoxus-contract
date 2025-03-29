use soroban_sdk::{contracttype, symbol_short, Address, BytesN, String, Symbol};

pub const ADMIN_KEY: Symbol = symbol_short!("ADMIN");
pub const COUNTER_KEY: Symbol = symbol_short!("COUNTER");
pub const ISSUERS_KEY: Symbol = symbol_short!("ISSUERS");
pub const PENDING_ADMIN: Symbol = symbol_short!("PENDING");
pub const NAME_KEY: Symbol = symbol_short!("NAME");
pub const SYMBOL_KEY: Symbol = symbol_short!("SYMBOL");

/// Struct representing the metadata of a certificate NFT
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CertificateMetadata {
    /// The ID of the course
    pub course_id: String,
    /// Completion date in UNIX timestamp format
    pub issued_date: u64,
    /// The issuer
    pub issuer: BytesN<65>,
    /// URI pointing to full certificate metadata (e.g., IPFS URL)
    pub metadata_uri: String,
    /// 65-byte ECDSA signature (r + s + v) of the hashed message
    pub signature: BytesN<65>,
    /// Address of recipient
    pub recipient: Address,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct CertificateDetail {
    pub owner: Address,
    pub metadata: CertificateMetadata,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    CertificateOwner(u32),
    CertificateMetadata(u32),
    CertificateUri(u32),
    CerticateRegister(Address),
}
