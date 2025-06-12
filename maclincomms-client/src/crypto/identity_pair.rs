use base64::{engine::general_purpose, Engine};
use disk_persist::DiskPersist;
use x25519_dalek::{PublicKey, StaticSecret};
use rand_core::{OsRng};

use crate::user_model::UserIdentityKeys;

pub fn generate_identity_keypair() -> String {

    let csprng = OsRng;

    let private_key = StaticSecret::random_from_rng(csprng);
 
    let public_key = PublicKey::from(&private_key);

    let private_key_b64 = general_purpose::STANDARD.encode(private_key.to_bytes());

    let public_key_b64 = general_purpose::STANDARD.encode(public_key.as_bytes());

    //Storing private key in persistent storage
    let persistent_storage: DiskPersist<UserIdentityKeys> = DiskPersist::init("persistent-user-identity-keypair").unwrap();

    let id_keypair_data = UserIdentityKeys {
        public_identity_key: public_key_b64.clone(),
        private_identity_key: private_key_b64
    };

    persistent_storage.write(&id_keypair_data).unwrap();

    return public_key_b64;
}