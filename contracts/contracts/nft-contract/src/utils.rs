use soroban_sdk::{vec, Address, Bytes, BytesN, Env, String, Vec};

/// Builds a deterministic message from certificate fields to be signed by the issuer
pub fn build_certificate_message(
    env: &Env,
    recipient: &Address,
    course_id: &String,
    metadata_uri: &String,
    issued_date: &u64,
) -> Bytes {
    let prefix = String::from_str(&env, "\x19Tradoxus Signed Message:\n");

    let prefix_bytes = string_to_bytes(env, prefix);
    let course_id_bytes = string_to_bytes(env, course_id.clone());
    let recipient_bytes = string_to_bytes(env, recipient.to_string());
    let metadata_uri_bytes = string_to_bytes(env, metadata_uri.clone());
    let completion_date_bytes = number_to_string_bytes(env, *issued_date);

    let message_len = course_id_bytes.len()
        + recipient_bytes.len()
        + metadata_uri_bytes.len()
        + completion_date_bytes.len();

    let len_bytes = number_to_string_bytes(env, message_len.into());

    let message: Bytes = concatenate_bytes(
        env,
        vec![
            env,
            prefix_bytes,
            len_bytes,
            course_id_bytes,
            recipient_bytes,
            metadata_uri_bytes,
            completion_date_bytes,
        ],
    );

    message
}

/// Verifies a secp256k1 signature against a keccak256 hash of the message
pub fn verify_issuer_signature(
    env: &Env,
    pub_key: &BytesN<65>,
    signature: &BytesN<65>, // 65-byte sig: r(32) + s(32) + v(1)
    message: &Bytes,
) -> bool {
    let hash = env.crypto().keccak256(message);

    let sig_array = signature.to_array();

    // Extract r + s (64 bytes)
    let r_s = &sig_array[..64];

    let mut final_sig = [0u8; 64];
    final_sig.copy_from_slice(r_s);
    let sig = BytesN::<64>::from_array(env, &final_sig);

    let recovery_code = sig_array[64];
    let recovery_id = match recovery_code {
        0 | 1 => recovery_code,
        27 | 28 => recovery_code - 27,
        _ => return false,
    };

    #[cfg(test)]
    {
        // To view this log run: `make test -- --no-capture`
        let signature = Signature::from_bytes(r_s.into()).expect("invalid r+s");
        let recovery_ = RecoveryId::try_from(recovery_id as u8).unwrap();
        let recovered_key =
            VerifyingKey::recover_from_prehash(&hash.to_array(), &signature, recovery_).unwrap();
        let expected_key = bytesn65_to_verifying_key(pub_key);

        match expected_key == recovered_key {
            true => std::println!("Test-only: keys match ✅"),
            false => {
                std::println!("❌ Failed to recover matching pubkey using k256.");
                std::println!(
                    "Expected: {:?}",
                    hex::encode(expected_key.to_encoded_point(false))
                );
                std::println!(
                    "Recovered: {:?}",
                    hex::encode(recovered_key.to_encoded_point(false))
                );
            }
        }
    }

    let recovered = env
        .crypto()
        .secp256k1_recover(&hash, &sig, recovery_id as u32);

    recovered == *pub_key
}

// convert a given string to a bytes array of its ascii characters
pub fn string_to_bytes(env: &Env, str1: String) -> Bytes {
    let str_len = str1.len() as usize;

    // how large should this buffer be?
    let mut buffer: [u8; 300000] = [0; 300000];

    str1.copy_into_slice(&mut buffer[..str_len]);
    Bytes::from_slice(&env, &buffer[0..str_len])
}

// represent a number as a series of bytes which stand for the ASCII code for each number
// i.e 012 => [48, 49, 50]
pub fn number_to_string_bytes(env: &Env, number: u64) -> Bytes {
    // Initialize an empty string to hold the result
    // how large should this buffer be?
    let mut buffer: [u8; 100000] = [0; 100000];
    let mut len = 0;

    // Convert the number to a positive value for simplicity
    let mut num = number;

    // if the number we are trying to convert is zero then just provide 48 which is ascii for buffer
    if num == 0 {
        buffer[0] = 48;
        len += 1
    };

    // Convert each digit of the number to a string and prepend it to the result
    while num > 0 {
        let digit = (num % 10) as u8; // Get the last digit

        buffer[len] = digit + 48; //add 48 to the number to get the ascii code
        num /= 10; // Move to the next digit
        len += 1; //increment index
    }

    // it is reversed because we start converting to ASCII bytes from the least significant digit instead of from the front
    // so we need to reverse this array so the items last attended to come back to the front where they are supposed to be
    let reversed_buffer = &mut buffer[0..len];
    reversed_buffer.reverse();

    Bytes::from_slice(&env, reversed_buffer)
}

// concatenate multiple bytes === abi.encodePacked implementation
pub fn concatenate_bytes(env: &Env, strings: Vec<Bytes>) -> Bytes {
    // create a byte buffer
    let mut concatenated_bytes = Bytes::new(env);

    for byte_group in strings {
        for byte in byte_group {
            concatenated_bytes.insert(concatenated_bytes.len(), byte)
        }
    }

    return concatenated_bytes;
}

#[cfg(test)]
use crate::types::CertificateMetadata;
#[cfg(test)]
use k256::ecdsa::{RecoveryId, Signature, SigningKey, VerifyingKey};
#[cfg(test)]
use rand::rngs::OsRng;
#[cfg(test)]
use rand::RngCore;
#[cfg(test)]
extern crate std;
#[cfg(test)]
pub fn gen_random_bytes<const N: usize>(env: &Env) -> BytesN<N> {
    let mut rng = OsRng;
    let mut random_bytes = [0u8; N];
    rng.try_fill_bytes(&mut random_bytes)
        .expect("unable to fill bytes");

    BytesN::from_array(env, &random_bytes)
}

#[cfg(test)]
pub fn sign(env: &Env, metadata: CertificateMetadata, signing_key: SigningKey) -> BytesN<65> {
    let message = build_certificate_message(
        &env,
        &metadata.recipient,
        &metadata.course_id,
        &metadata.metadata_uri,
        &metadata.issued_date,
    );

    let hash = env.crypto().keccak256(&message);

    let (sig, recovery_id) = signing_key
        .sign_prehash_recoverable(&hash.to_array())
        .unwrap();

    let mut sig_bytes = [0u8; 65];
    sig_bytes[..64].copy_from_slice(&sig.to_vec()); // r + s
    sig_bytes[64] = recovery_id.into(); // v

    BytesN::from_array(&env, &sig_bytes)
}

#[cfg(test)]
pub fn generate_keypair(env: &Env) -> (SigningKey, VerifyingKey, BytesN<65>) {
    let mut rng = OsRng;
    let signing_key = SigningKey::random(&mut rng);
    let verifying_key = VerifyingKey::from(&signing_key);

    let encoded = verifying_key.to_encoded_point(false);
    let pubkey_bytes = encoded.as_bytes();

    let mut pubkey_arr = [0u8; 65];
    pubkey_arr.copy_from_slice(pubkey_bytes);

    let public_key = BytesN::from_array(env, &pubkey_arr);

    (signing_key, verifying_key, public_key)
}

#[cfg(test)]
fn bytesn65_to_verifying_key(bytes: &BytesN<65>) -> VerifyingKey {
    let encoded = k256::EncodedPoint::from_bytes(bytes.to_array()).expect("invalid pubkey bytes");
    VerifyingKey::from_encoded_point(&encoded).expect("invalid encoded point")
}
