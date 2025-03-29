#![cfg(test)]
extern crate alloc;
extern crate std;

use super::{contract::*, types::*, utils::*};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::testutils::{AuthorizedFunction, AuthorizedInvocation};
use soroban_sdk::{Address, Env, IntoVal, String, Symbol, Vec};

struct TestContext {
    nft_address: Address,
    env: Env,
    admin: Address,
    client: CertificateNFTClient<'static>,
}

impl TestContext {
    fn new() -> Self {
        let env = Env::default();
        env.cost_estimate().budget().reset_unlimited();
        env.mock_all_auths();

        let admin = Address::generate(&env);

        let name = String::from_str(&env, "Traxodus Cerficates").to_val();
        let symbol = String::from_str(&env, "TxCerts").to_val();

        let nft_address = env.register(CertificateNFT, (&name, &symbol));
        let client = CertificateNFTClient::new(&env, &nft_address);

        client.initialize(&admin);

        TestContext {
            nft_address,
            env,
            admin,
            client,
        }
    }
}

#[test]
#[should_panic(expected = "#100")]
fn init_only_once() {
    let ctx = TestContext::new();
    ctx.client.initialize(&ctx.admin);
}

#[test]
fn test_transfer_admin() {
    let ctx = TestContext::new();

    let new_admin = Address::generate(&ctx.env);
    ctx.client.transfer_admin(&new_admin);

    assert_eq!(
        ctx.env.auths(),
        std::vec![(
            ctx.admin,
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    ctx.client.address.clone(),
                    Symbol::new(&ctx.env, "transfer_admin"),
                    (new_admin.clone(),).into_val(&ctx.env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    )
}

#[test]
fn test_claim_admin() {
    let ctx = TestContext::new();

    let new_admin = Address::generate(&ctx.env);
    ctx.client.transfer_admin(&new_admin);

    ctx.client.accept_admin();

    assert_eq!(
        ctx.env.auths(),
        std::vec![(
            new_admin,
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    ctx.client.address.clone(),
                    Symbol::new(&ctx.env, "accept_admin"),
                    Vec::new(&ctx.env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    )
}

#[test]
#[should_panic]
fn test_set_admin_fail() {
    let ctx = TestContext::new();

    let new_admin = Address::generate(&ctx.env);
    ctx.client.transfer_admin(&new_admin);

    assert_eq!(
        ctx.env.auths(),
        std::vec![(
            ctx.nft_address,
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    ctx.client.address.clone(),
                    Symbol::new(&ctx.env, "transfer_admin"),
                    (new_admin.clone(),).into_val(&ctx.env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    )
}

#[test]
#[should_panic]
fn test_claim_admin_fail() {
    let ctx = TestContext::new();

    let new_admin = Address::generate(&ctx.env);
    let fake_admin = Address::generate(&ctx.env);
    ctx.client.transfer_admin(&new_admin);

    ctx.client.accept_admin();

    assert_eq!(
        ctx.env.auths(),
        std::vec![(
            fake_admin,
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    ctx.client.address.clone(),
                    Symbol::new(&ctx.env, "accept_admin"),
                    Vec::new(&ctx.env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    )
}

#[test]
fn test_admin_add_issuer() {
    let ctx = TestContext::new();

    let new_issuer = gen_random_bytes::<65>(&ctx.env);
    ctx.client.add_issuer(&new_issuer);

    assert_eq!(
        ctx.env.auths(),
        std::vec![(
            ctx.admin,
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    ctx.client.address.clone(),
                    Symbol::new(&ctx.env, "add_issuer"),
                    (new_issuer.clone(),).into_val(&ctx.env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    )
}

#[test]
#[should_panic]
fn test_admin_add_issuer_fail() {
    let ctx = TestContext::new();

    let fake_admin = Address::generate(&ctx.env);
    let new_issuer = gen_random_bytes::<65>(&ctx.env);
    ctx.client.add_issuer(&new_issuer);

    assert_eq!(
        ctx.env.auths(),
        std::vec![(
            fake_admin,
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    ctx.client.address.clone(),
                    Symbol::new(&ctx.env, "add_issuer"),
                    (new_issuer.clone(),).into_val(&ctx.env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    )
}

#[test]
#[should_panic(expected = "#106")]
fn test_add_issuer_fails_if_already_exists() {
    let ctx = TestContext::new();
    let new_issuer = gen_random_bytes::<65>(&ctx.env);
    ctx.client.add_issuer(&new_issuer);
    ctx.client.add_issuer(&new_issuer);
}

#[test]
fn test_admin_remove_issuer() {
    let ctx = TestContext::new();

    let new_issuer = gen_random_bytes::<65>(&ctx.env);
    ctx.client.add_issuer(&new_issuer);
    ctx.client.remove_issuer(&new_issuer);

    assert_eq!(
        ctx.env.auths(),
        std::vec![(
            ctx.admin,
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    ctx.client.address.clone(),
                    Symbol::new(&ctx.env, "remove_issuer"),
                    (new_issuer.clone(),).into_val(&ctx.env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    )
}

#[test]
#[should_panic]
fn test_admin_remove_issuer_fail() {
    let ctx = TestContext::new();

    let fake_admin = Address::generate(&ctx.env);
    let new_issuer = gen_random_bytes::<65>(&ctx.env);
    ctx.client.add_issuer(&new_issuer);
    ctx.client.remove_issuer(&new_issuer);

    assert_eq!(
        ctx.env.auths(),
        std::vec![(
            fake_admin,
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    ctx.client.address.clone(),
                    Symbol::new(&ctx.env, "remove_issuer"),
                    (new_issuer.clone(),).into_val(&ctx.env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    )
}

#[test]
#[should_panic(expected = "#107")]
fn test_add_issuer_fails_if_not_found() {
    let ctx = TestContext::new();
    let rand_issuer = gen_random_bytes::<65>(&ctx.env);
    ctx.client.remove_issuer(&rand_issuer);
}

#[test]
fn test_signature() {
    let ctx = TestContext::new();
    let empty_bytes = gen_random_bytes::<65>(&ctx.env);

    let (sk, _, pk) = generate_keypair(&ctx.env);

    let metadata = CertificateMetadata {
        course_id: String::from_str(&ctx.env, "course:solidity-bootcamp-2025"),
        issued_date: ctx.env.ledger().timestamp(),
        recipient: Address::generate(&ctx.env),
        metadata_uri: String::from_str(
            &ctx.env,
            "ipfs://QmZkN8nsyK5cWmKL9BfEzDvDU1DqYKdEgqPtAGh8qMhjBd",
        ),
        signature: empty_bytes,
        issuer: pk.clone(),
    };

    let signature = sign(&ctx.env, metadata.clone(), sk);

    let message = &build_certificate_message(
        &ctx.env,
        &metadata.recipient,
        &metadata.course_id,
        &metadata.metadata_uri,
        &metadata.issued_date,
    );

    let valid = verify_issuer_signature(&ctx.env, &pk, &signature, message);
    assert!(valid);
}

#[test]
fn test_mint_certificate() {
    let ctx = TestContext::new();
    let empty_bytes = gen_random_bytes::<65>(&ctx.env);

    let (sk, _, new_issuer) = generate_keypair(&ctx.env);

    ctx.client.add_issuer(&new_issuer);

    let mut metadata = CertificateMetadata {
        course_id: String::from_str(&ctx.env, "course:solidity-bootcamp-2025"),
        issued_date: ctx.env.ledger().timestamp(),
        recipient: Address::generate(&ctx.env),
        metadata_uri: String::from_str(
            &ctx.env,
            "ipfs://QmZkN8nsyK5cWmKL9BfEzDvDU1DqYKdEgqPtAGh8qMhjBd",
        ),
        signature: empty_bytes,
        issuer: new_issuer.clone(),
    };

    let signature = sign(&ctx.env, metadata.clone(), sk);

    metadata.signature = signature;

    let certificate_id = ctx.client.mint_certificate(
        &metadata.recipient,
        &metadata.course_id,
        &metadata.metadata_uri,
        &metadata.issuer,
        &metadata.issued_date,
        &metadata.signature,
    );

    let issued_certificates = ctx.client.issued_certificates();
    let certificate_owner = ctx.client.owner_of(&certificate_id);
    let user_issued_certificate = ctx.client.user_issued_certificates(&metadata.recipient);

    assert_eq!(certificate_owner, metadata.recipient);
    assert_eq!(issued_certificates, 1);
    assert_eq!(user_issued_certificate, 1);
}

#[test]
#[should_panic(expected = "#105")]
fn test_mint_certificate_invalid_signature() {
    let ctx = TestContext::new();
    let empty_bytes = gen_random_bytes::<65>(&ctx.env);

    let (sk, _, new_issuer) = generate_keypair(&ctx.env);

    ctx.client.add_issuer(&new_issuer);

    let wrong_data = String::from_str(&ctx.env, "course:solidity-bootcamp-2030");

    let mut metadata = CertificateMetadata {
        course_id: String::from_str(&ctx.env, "course:solidity-bootcamp-2025"),
        issued_date: ctx.env.ledger().timestamp(),
        recipient: Address::generate(&ctx.env),
        metadata_uri: String::from_str(
            &ctx.env,
            "ipfs://QmZkN8nsyK5cWmKL9BfEzDvDU1DqYKdEgqPtAGh8qMhjBd",
        ),
        signature: empty_bytes,
        issuer: new_issuer.clone(),
    };

    let signature = sign(&ctx.env, metadata.clone(), sk);

    metadata.signature = signature;

    ctx.client.mint_certificate(
        &metadata.recipient,
        &wrong_data,
        &metadata.metadata_uri,
        &metadata.issuer,
        &metadata.issued_date,
        &metadata.signature,
    );
}

#[test]
#[should_panic(expected = "#102")]
fn test_mint_certificate_failed_not_issuer() {
    let ctx = TestContext::new();
    let empty_bytes = gen_random_bytes::<65>(&ctx.env);

    let (sk, _, new_issuer) = generate_keypair(&ctx.env);

    let mut metadata = CertificateMetadata {
        course_id: String::from_str(&ctx.env, "course:solidity-bootcamp-2025"),
        issued_date: ctx.env.ledger().timestamp(),
        recipient: Address::generate(&ctx.env),
        metadata_uri: String::from_str(
            &ctx.env,
            "ipfs://QmZkN8nsyK5cWmKL9BfEzDvDU1DqYKdEgqPtAGh8qMhjBd",
        ),
        signature: empty_bytes,
        issuer: new_issuer.clone(),
    };

    let signature = sign(&ctx.env, metadata.clone(), sk);

    metadata.signature = signature;

    ctx.client.mint_certificate(
        &metadata.recipient,
        &metadata.course_id,
        &metadata.metadata_uri,
        &metadata.issuer,
        &metadata.issued_date,
        &metadata.signature,
    );
}

#[test]
fn test_verify_certificate() {
    let ctx = TestContext::new();
    let empty_bytes = gen_random_bytes::<65>(&ctx.env);

    let (sk, _, new_issuer) = generate_keypair(&ctx.env);

    ctx.client.add_issuer(&new_issuer);

    let mut metadata = CertificateMetadata {
        course_id: String::from_str(&ctx.env, "course:solidity-bootcamp-2025"),
        issued_date: ctx.env.ledger().timestamp(),
        recipient: Address::generate(&ctx.env),
        metadata_uri: String::from_str(
            &ctx.env,
            "ipfs://QmZkN8nsyK5cWmKL9BfEzDvDU1DqYKdEgqPtAGh8qMhjBd",
        ),
        signature: empty_bytes,
        issuer: new_issuer.clone(),
    };

    let signature = sign(&ctx.env, metadata.clone(), sk);

    metadata.signature = signature;

    let certificate_id = ctx.client.mint_certificate(
        &metadata.recipient,
        &metadata.course_id,
        &metadata.metadata_uri,
        &metadata.issuer,
        &metadata.issued_date,
        &metadata.signature,
    );

    let certificate_data = &build_certificate_message(
        &ctx.env,
        &metadata.recipient,
        &metadata.course_id,
        &metadata.metadata_uri,
        &metadata.issued_date,
    );

    let verified = ctx
        .client
        .verify_certificate(&certificate_id, certificate_data);

    assert!(verified);
}

#[test]
fn test_verify_certificate_failed() {
    let ctx = TestContext::new();
    let empty_bytes = gen_random_bytes::<65>(&ctx.env);

    let (sk, _, new_issuer) = generate_keypair(&ctx.env);

    ctx.client.add_issuer(&new_issuer);

    let mut metadata = CertificateMetadata {
        course_id: String::from_str(&ctx.env, "course:solidity-bootcamp-2025"),
        issued_date: ctx.env.ledger().timestamp(),
        recipient: Address::generate(&ctx.env),
        metadata_uri: String::from_str(
            &ctx.env,
            "ipfs://QmZkN8nsyK5cWmKL9BfEzDvDU1DqYKdEgqPtAGh8qMhjBd",
        ),
        signature: empty_bytes,
        issuer: new_issuer.clone(),
    };

    let signature = sign(&ctx.env, metadata.clone(), sk);

    metadata.signature = signature;

    let certificate_id = ctx.client.mint_certificate(
        &metadata.recipient,
        &metadata.course_id,
        &metadata.metadata_uri,
        &metadata.issuer,
        &metadata.issued_date,
        &metadata.signature,
    );

    let wrong_data = String::from_str(&ctx.env, "course:solidity-bootcamp-2030");

    let certificate_data = &build_certificate_message(
        &ctx.env,
        &metadata.recipient,
        &wrong_data,
        &metadata.metadata_uri,
        &metadata.issued_date,
    );

    let verified = ctx
        .client
        .verify_certificate(&certificate_id, certificate_data);

    assert!(!verified);
}

#[test]
#[should_panic(expected = "#103")]
fn test_verify_certificate_failed_certificate_not_found() {
    let ctx = TestContext::new();
    let empty_bytes = gen_random_bytes::<65>(&ctx.env);

    let metadata = CertificateMetadata {
        course_id: String::from_str(&ctx.env, "course:solidity-bootcamp-2025"),
        issued_date: ctx.env.ledger().timestamp(),
        recipient: Address::generate(&ctx.env),
        metadata_uri: String::from_str(
            &ctx.env,
            "ipfs://QmZkN8nsyK5cWmKL9BfEzDvDU1DqYKdEgqPtAGh8qMhjBd",
        ),
        signature: empty_bytes.clone(),
        issuer: empty_bytes.clone(),
    };

    let certificate_id = 20;

    let certificate_data = &build_certificate_message(
        &ctx.env,
        &metadata.recipient,
        &metadata.course_id,
        &metadata.metadata_uri,
        &metadata.issued_date,
    );

    ctx.client
        .verify_certificate(&certificate_id, certificate_data);
}
