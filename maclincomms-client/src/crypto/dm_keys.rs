use hkdf::Hkdf;
use rand_core::OsRng;
use sha2::Sha256;
use x25519_dalek::{PublicKey, StaticSecret};

use crate::user_model::DmDoubleRatchet_Keys;



pub fn generate_shared_rootkey(their_pub_key: [u8;32], my_priv_key: [u8;32]) -> [u8;32] {
    let my_priv = StaticSecret::from(my_priv_key);
    let their_pub = PublicKey::from(their_pub_key);
    let shared_secret = my_priv.diffie_hellman(&their_pub);
    let hk = Hkdf::<Sha256>::new(None, shared_secret.as_bytes());
    let mut root_key = [0u8;32];
    hk.expand(&[0x02], &mut root_key).unwrap();
    return root_key;
}

pub fn generate_dh_keypair() -> ([u8;32], [u8;32]) {
    let csprng = OsRng;
    let private_key = StaticSecret::random_from_rng(csprng);
    let public_key = PublicKey::from(&private_key);
    return (public_key.to_bytes(), private_key.to_bytes())
}

pub fn generate_sender_chainkey(root_key: [u8;32]) -> [u8;32] {
    let hk = Hkdf::<Sha256>::new(None, &root_key);
    let mut derived_sender_key: [u8; 32] = [0u8;32];
    hk.expand(&[0x01], &mut derived_sender_key).unwrap();
    return derived_sender_key;
}

pub fn generate_receiver_chainkey(root_key: [u8;32]) -> [u8;32] {
    let hk = Hkdf::<Sha256>::new(None, &root_key);
    let mut derived_receiver_key: [u8; 32] = [0u8;32];
    hk.expand(&[0x01], &mut derived_receiver_key).unwrap();
    return derived_receiver_key;
}

pub fn derive_message_key(chain_key: [u8;32]) -> [u8;32] {
    let hk = Hkdf::<Sha256>::new(None, &chain_key);
    let mut derived_message_key: [u8; 32] = [0u8;32];
    hk.expand(&[0x01], &mut derived_message_key).unwrap();
    return derived_message_key;
}

//FORWARD SECRECY
pub fn update_sending_chainkey(keys: &mut DmDoubleRatchet_Keys) {
    let hk = Hkdf::<Sha256>::new(None, &keys.sending_chain_key);
    let mut derived_sending_chainkey: [u8; 32] = [0u8;32];
    hk.expand(&[0x02], &mut derived_sending_chainkey).unwrap();
    keys.sending_chain_key = derived_sending_chainkey;
}

pub fn update_receiving_chainkey(keys: &mut DmDoubleRatchet_Keys) {
    let hk = Hkdf::<Sha256>::new(None, &keys.receiving_chain_key);
    let mut derived_receiving_chainkey: [u8; 32] = [0u8;32];
    hk.expand(&[0x02], &mut derived_receiving_chainkey).unwrap();
    keys.receiving_chain_key = derived_receiving_chainkey;
}