use crate::types::{
    CertificateMetadata, DataKey, ADMIN_KEY, COUNTER_KEY, ISSUERS_KEY, NAME_KEY, PENDING_ADMIN,
    SYMBOL_KEY,
};
use soroban_sdk::{Address, BytesN, Env, String, Vec};

pub struct Storage;

impl Storage {
    // Set the admin of the contract in the storage
    pub fn set_admin(env: &Env, admin: &Address) {
        env.storage().instance().set(&ADMIN_KEY, admin);
    }

    pub fn get_admin(env: &Env) -> Address {
        env.storage().instance().get(&ADMIN_KEY).unwrap()
    }

    pub fn set_pending_admin(env: &Env, address: &Address) {
        env.storage().instance().set(&PENDING_ADMIN, address);
    }

    pub fn get_pending_admin(env: &Env) -> Option<Address> {
        env.storage().instance().get(&PENDING_ADMIN)
    }

    pub fn clear_pending_admin(env: &Env) {
        env.storage().instance().remove(&ADMIN_KEY)
    }

    pub fn set_name(env: &Env, name: &String) {
        env.storage().instance().set(&NAME_KEY, name);
    }

    pub fn get_name(env: &Env) -> String {
        env.storage().instance().get(&NAME_KEY).unwrap()
    }

    pub fn set_symbol(env: &Env, symbol: &String) {
        env.storage().instance().set(&SYMBOL_KEY, symbol);
    }

    pub fn get_symbol(env: &Env) -> String {
        env.storage().instance().get(&SYMBOL_KEY).unwrap()
    }

    pub fn set_issuers(env: &Env, issuers: &Vec<BytesN<65>>) {
        env.storage().instance().set(&ISSUERS_KEY, issuers);
    }

    pub fn get_issuers(env: &Env) -> Vec<BytesN<65>> {
        env.storage().instance().get(&ISSUERS_KEY).unwrap()
    }

    pub fn set_token_counter(env: &Env, count: &u32) {
        env.storage().instance().set(&COUNTER_KEY, count);
    }

    pub fn get_token_counter(env: &Env) -> u32 {
        env.storage().instance().get(&COUNTER_KEY).unwrap()
    }

    pub fn set_certificate_owner(env: &Env, certificate_id: &u32, owner: &Address) {
        env.storage()
            .instance()
            .set(&DataKey::CertificateOwner(*certificate_id), owner);
    }

    pub fn get_certificate_owner(env: &Env, certificate_id: &u32) -> Option<Address> {
        env.storage()
            .instance()
            .get(&DataKey::CertificateOwner(*certificate_id))
    }

    pub fn set_certificate_metadata(
        env: &Env,
        certificate_id: &u32,
        metadata: &CertificateMetadata,
    ) {
        env.storage()
            .instance()
            .set(&DataKey::CertificateMetadata(*certificate_id), metadata);
    }

    pub fn get_certificate_metadata(
        env: &Env,
        certificate_id: &u32,
    ) -> Option<CertificateMetadata> {
        env.storage()
            .instance()
            .get(&DataKey::CertificateMetadata(*certificate_id))
    }

    pub fn register_new_certificate(env: &Env, address: &Address) {
        let count: u32 = env
            .storage()
            .instance()
            .get(&DataKey::CerticateRegister(address.clone()))
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::CerticateRegister(address.clone()), &(count + 1));
    }

    pub fn certificates_issued_to_user(env: &Env, address: &Address) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::CerticateRegister(address.clone()))
            .unwrap_or(0u32)
    }
}
