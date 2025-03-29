# 📜 Trazodus Certificate NFT Contract

This project implements a Verifiable Certificate NFT system on the [Stellar Soroban](https://soroban.stellar.org/) smart contract platform. It issues and verifies NFTs representing certificates (e.g., course completions), structured to align with **W3C Verifiable Credentials** standards.

The contract enables institutions to mint signed certificates on-chain, and allows any third party to verify their authenticity through cryptographic signature recovery — all with full compatibility for use in identity ecosystems.

## ✨ Key Features

- ✅ **NFT Minting** — Certificates are minted as unique, verifiable NFTs with course metadata and issuer signature.
- ✅ **Issuer Authorization** — Only approved issuers (public keys) can mint new certificates.
- ✅ **Signature-Based Verification** — On-chain signature recovery validates certificates against the issuer's known public key.
- ✅ **Metadata Support** — Includes recipient address, course ID, metadata URI, timestamp, and more.
- ✅ **W3C Verifiable Credentials Alignment** — Designed to follow W3C VC structure using `keccak256` and `ECDSA/secp256k1`.
- ✅ **Admin Role Transfer** — Secure transfer of contract admin role with 2-step confirmation.
- ✅ **Test Coverage** — Includes end-to-end tests for minting, verification, and signature flow.

## 📁 Project Structure

```text
nft-contract/
├── src/
│   ├── contract.rs     # Main contract logic (CertificateNFT implementation)
│   ├── error.rs        # Custom error definitions
│   ├── event.rs        # Events emitted by the contract (minting, admin changes, etc.)
│   ├── lib.rs          # Module entry point and re-exports
│   ├── storage.rs      # Persistent state handling (admin, issuers, certs)
│   ├── test.rs         # Unit & integration tests for contract functionality
│   ├── types.rs        # Structs & models, e.g., CertificateMetadata
│   └── utils.rs        # Utility functions for message building, signing, hashing
├── .gitignore          # Files and folders to exclude from version control
├── Cargo.toml          # Contract crate manifest
├── Makefile            # Useful make commands for build/test workflows
└── README.md           # Project overview and documentation

```

## 📦 Usage Overview

### 🛠️ Initialization

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

Initializes the contract and sets the contract admin. Only callable once.

### 👮 Issuer Management

```rust
pub fn add_issuer(env: Env, issuer_pubkey: BytesN<65>) -> Result<(), Error>
pub fn remove_issuer(env: Env, issuer_pubkey: BytesN<65>) -> Result<(), Error>
```

Only the admin can manage trusted issuer public keys.

### 🖊️ Mint Certificate

```rust
pub fn mint_certificate(
    env: Env,
    recipient: Address,
    course_id: String,
    metadata_uri: String,
    issuer: BytesN<65>,
    issued_date: u64,
    signature: BytesN<65>
) -> Result<u32, Error>
```

Mints a new certificate NFT if the issuer is trusted and the signature is valid. The `signature` must be generated off-chain using the same message construction logic defined in `utils.rs`.

### ✅ Verify Certificate

```rust
pub fn verify_certificate(
    env: Env,
    certificate_id: u32,
    certificate_data: Bytes
) -> Result<bool, Error>
```

Verifies that the given certificate data (used in signing) was signed by the issuer whose pubkey was stored during minting.

### 📄 Certificate Metadata

```rust
pub fn certificate_metadata(env: Env, certificate_id: u32) -> Result<CertificateDetail, Error>
```

Retrieves the full metadata and owner info for any issued certificate.

### 👤 Track User Issuance Count

```rust
pub fn get_certificates_issued_to_user(env: Env, address: Address) -> u32
```

Returns how many certificates have been issued to a given user address.

### 🔐 Admin Role Transfer

```rust
pub fn transfer_admin(env: Env, new_admin: Address) -> Result<(), Error>
pub fn accept_admin(env: Env) -> Result<(), Error>
```

Implements a 2-step role transfer system where the current admin proposes, and the new one accepts.

## 🧪 Running Tests

```sh
make test -- --no-capture
```

This will run all tests in `test.rs` and print logs (e.g., recovered public keys, keccak hashes, etc.) for verification.

> 🔧 Ensure you have Soroban CLI installed and configured properly.

## 🧠 W3C VC Compatibility

Each certificate NFT encodes metadata using a structure similar to:

```json
{
  "certificate": {
    "id": "course_id",
    "issuanceDate": "timestamp"
  },
  "issuer": {
    "id": "issuer_pubkey"
  },
  "recipient": {
    "id": "stellar_address"
  },
  "proof": {
    "type": "EcdsaSecp256k1Recovery2024",
    "signature": "r+s+v",
    "messageHash": "keccak256(message)"
  }
}
```

The message being signed is assembled using:

- course ID
- metadata URI
- recipient address
- completion date

And prefixed with a message domain like:

```text
"\x19Tradoxus Signed Message:\n"
```

## 🧑‍💻 Contributing

PRs welcome! Please ensure all new features are:

- Covered with tests
- Documented via inline comments
- Emitting events where appropriate
