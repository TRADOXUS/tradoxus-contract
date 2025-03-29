//! Certificate NFT with W3C VC-style verification
//! This Soroban smart contract allows an admin to authorize issuers who can mint
//! verifiable certificate NFTs. Each certificate contains metadata and a signature
//! that can be verified on-chain to confirm authenticity. This aligns with W3C
//! Verifiable Credentials principles.

use soroban_sdk::{
    contract, contractimpl, panic_with_error, Address, Bytes, BytesN, Env, String, Vec,
};

use crate::{errors::Error, events::Events, storage::Storage, types::*, utils::*};

#[contract]
pub struct CertificateNFT;

#[contractimpl]
impl CertificateNFT {
    pub fn __constructor(env: Env, name: String, symbol: String) {
        Storage::set_name(&env, &name);
        Storage::set_symbol(&env, &symbol);
    }

    /// Initializes the contract and sets the admin address.
    /// Can only be called once.
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&ADMIN_KEY) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        Storage::set_admin(&env, &admin);
        Storage::set_issuers(&env, &Vec::<BytesN<65>>::new(&env));
        Storage::set_token_counter(&env, &0u32);
        Ok(())
    }

    /// Transfers admin role to a new pending admin.
    pub fn transfer_admin(env: Env, new_admin: Address) -> Result<(), Error> {
        let current_admin = Storage::get_admin(&env);
        current_admin.require_auth();
        Storage::set_pending_admin(&env, &new_admin);
        Events::admin_transfer_initiated(&env, &new_admin);
        Ok(())
    }

    /// Accepts the admin role. Only callable by the new pending admin.
    pub fn accept_admin(env: Env) -> Result<(), Error> {
        let pending_admin = Storage::get_pending_admin(&env)
            .unwrap_or_else(|| panic_with_error!(&env, Error::NoPendingAdmin));

        pending_admin.require_auth();

        Storage::set_admin(&env, &pending_admin);
        Storage::clear_pending_admin(&env);

        Events::admin_transfer_completed(&env, &pending_admin);
        Ok(())
    }

    /// Adds a new issuer. Only callable by the admin.
    pub fn add_issuer(env: Env, issuer: BytesN<65>) -> Result<(), Error> {
        let admin = Storage::get_admin(&env);

        admin.require_auth();

        let mut issuers = Storage::get_issuers(&env);

        if issuers.contains(&issuer) {
            panic_with_error!(&env, Error::IssuerAlreadySet);
        }

        if !issuers.contains(&issuer) {
            issuers.push_back(issuer);
        }

        Storage::set_issuers(&env, &issuers);

        Events::add_issuer(&env, issuers.len());

        Ok(())
    }

    /// Removes an existing issuer. Only callable by the admin.
    pub fn remove_issuer(env: Env, issuer: BytesN<65>) -> Result<(), Error> {
        let admin = Storage::get_admin(&env);

        admin.require_auth();

        let mut issuers = Storage::get_issuers(&env);

        if let Some(index) = issuers.first_index_of(&issuer) {
            issuers.remove(index);
        } else {
            panic_with_error!(&env, Error::IssuerNotFound)
        }

        Storage::set_issuers(&env, &issuers);

        Events::remove_issuer(&env, issuers.len());

        Ok(())
    }

    /// Mints a new certificate NFT. Only authorized issuers can mint.
    /// Signature is verified against a deterministic messagenv.
    pub fn mint_certificate(
        env: Env,
        recipient: Address,
        course_id: String,
        metadata_uri: String,
        issuer: BytesN<65>,
        issued_date: u64,
        signature: BytesN<65>,
    ) -> Result<u32, Error> {
        let issuers = Storage::get_issuers(&env);

        if !issuers.contains(&issuer) {
            panic_with_error!(&env, Error::NotIssuer);
        }

        let mut certificate_id = Storage::get_token_counter(&env);

        // Construct the canonical message to be signed
        let message =
            build_certificate_message(&env, &recipient, &course_id, &metadata_uri, &issued_date);

        // Validate that the provided signature matches the message and issuer pubkey
        if !verify_issuer_signature(&env, &issuer, &signature, &message) {
            panic_with_error!(&env, Error::InvalidSignature);
        }

        let metadata = CertificateMetadata {
            course_id,
            issued_date,
            issuer: issuer.clone(),
            metadata_uri,
            signature,
            recipient: recipient.clone(),
        };

        Storage::set_certificate_metadata(&env, &certificate_id, &metadata);

        Storage::set_certificate_owner(&env, &certificate_id, &recipient);

        Storage::register_new_certificate(&env, &recipient);

        certificate_id += 1;

        Storage::set_token_counter(&env, &certificate_id);

        Events::mint(&env, &recipient, certificate_id);

        Ok(certificate_id - 1u32)
    }

    /// Verifies a stored certificate by checking its original signature and public key
    pub fn verify_certificate(
        env: Env,
        certificate_id: u32,
        certificate_data: Bytes,
    ) -> Result<bool, Error> {
        let cert = Storage::get_certificate_metadata(&env, &certificate_id)
            .ok_or_else(|| panic_with_error!(&env, Error::CertificateNotFound))?;

        Ok(verify_issuer_signature(
            &env,
            &cert.issuer,
            &cert.signature,
            &certificate_data,
        ))
    }

    pub fn name(env: Env) -> String {
        Storage::get_name(&env)
    }

    pub fn symbol(env: Env) -> String {
        Storage::get_symbol(&env)
    }

    pub fn issued_certificates(env: Env) -> u32 {
        Storage::get_token_counter(&env)
    }

    pub fn owner_of(env: Env, certificate_id: u32) -> Address {
        Storage::get_certificate_owner(&env, &certificate_id)
            .unwrap_or_else(|| panic_with_error!(&env, Error::CertificateNotFound))
    }

    /// Retrieves the metadata of a certificate given its token ID
    pub fn certificate_metadata(env: Env, certificate_id: u32) -> Result<CertificateDetail, Error> {
        let owner = Storage::get_certificate_owner(&env, &certificate_id)
            .unwrap_or_else(|| panic_with_error!(&env, Error::CertificateNotFound));

        let metadata = Storage::get_certificate_metadata(&env, &certificate_id)
            .unwrap_or_else(|| panic!("Metadata not found"));

        Ok(CertificateDetail { owner, metadata })
    }

    /// Returns number of certificates issued to user
    pub fn user_issued_certificates(env: Env, address: Address) -> u32 {
        Storage::certificates_issued_to_user(&env, &address)
    }
}
