use hkdf::Hkdf;
use rand_core::OsRng;
use sha2::Sha256;
use x25519_dalek::{StaticSecret};

use crate::user_model::{Room_Keys, SenderKey};


pub fn generate_roomchain_key() -> [u8;32] {
    let csprng = OsRng;
    let chain_key = StaticSecret::random_from_rng(csprng);
    return chain_key.to_bytes();
}


// [CHAIN_KEY][PUBLIC_SIGNATURE_KEY]
pub fn compose_sender_key(chain_key: [u8;32], pub_sig_key: [u8;32]) -> [u8;64] {
    let sender_key: [u8;64] = [chain_key, pub_sig_key].concat().try_into().unwrap();
    return sender_key;
}


//FORWARD SECRECY
pub fn update_my_roomchainkey(my_keys: &mut Room_Keys) {
    let hk = Hkdf::<Sha256>::new(None, &my_keys.chain_key);
    let mut derived_chainkey: [u8; 32] = [0u8;32];
    hk.expand(&[0x02], &mut derived_chainkey).unwrap();
    my_keys.chain_key = derived_chainkey;
}

pub fn update_their_roomchainkey(s_key: &mut SenderKey) {
    let hk = Hkdf::<Sha256>::new(None, &s_key.chain_key);
    let mut derived_chainkey: [u8; 32] = [0u8;32];
    hk.expand(&[0x02], &mut derived_chainkey).unwrap();
    s_key.chain_key = derived_chainkey;
}

pub fn derive_roommessage_key(chain_key: [u8;32]) -> [u8;32] {
    let hk = Hkdf::<Sha256>::new(None, &chain_key);
    let mut derived_message_key: [u8; 32] = [0u8;32];
    hk.expand(&[0x01], &mut derived_message_key).unwrap();
    return derived_message_key;
}

