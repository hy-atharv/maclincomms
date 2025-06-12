use ed25519_dalek::{SigningKey};
use rand_core::OsRng;

pub fn generate_signature_keypair() -> ([u8; 32], [u8; 32]) {
    let mut csprng = OsRng;

    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();

    let private_key_bytes = signing_key.to_bytes(); // [u8; 32]
    let public_key_bytes = verifying_key.to_bytes(); // [u8; 32]

    (public_key_bytes, private_key_bytes)
}
