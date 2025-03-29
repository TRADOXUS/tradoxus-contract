use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 100,
    NotAdmin = 101,
    NotIssuer = 102,
    CertificateNotFound = 103,
    Unauthorized = 104,
    InvalidSignature = 105,
    IssuerAlreadySet = 106,
    IssuerNotFound = 107,
    NoPendingAdmin = 108,
    InvalidData = 109,
}
