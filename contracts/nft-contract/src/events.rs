use soroban_sdk::{symbol_short, Address, Env};

pub struct Events;

impl Events {
    pub fn mint(env: &Env, to: &Address, certificate_id: u32) {
        let topics = (symbol_short!("mint"), to);
        env.events().publish(topics, certificate_id);
    }

    pub fn add_issuer(env: &Env, count: u32) {
        let topics = symbol_short!("issuer");
        env.events().publish((topics,), count);
    }

    pub fn remove_issuer(env: &Env, count: u32) {
        let topics = symbol_short!("issuer");
        env.events().publish((topics,), count);
    }

    pub fn admin_transfer_initiated(env: &Env, new_admin: &Address) {
        let topics = symbol_short!("admin");
        env.events().publish((topics,), new_admin);
    }

    pub fn admin_transfer_completed(env: &Env, new_admin: &Address) {
        let topics = symbol_short!("admin");
        env.events().publish((topics,), new_admin);
    }
}
