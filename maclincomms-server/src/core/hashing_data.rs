use argon2::{password_hash::{PasswordHasher, SaltString}, Argon2, PasswordHash, PasswordVerifier};
use ring::pbkdf2;
use rand::{rngs::OsRng, Rng};
use std::num::NonZeroU32;

const  PBKDF2_ITERATIONS: NonZeroU32 = match NonZeroU32::new(100_000) {
    Some(value) => value,
    None => panic!("Invalid value for PBKDF2 iterations"),
}; // Industry standard iterations

const SALT_LENGTH: usize = 16; // Salt length in bytes
const KEY_LENGTH: usize = 32;  // Derived key length (32 bytes for SHA-256)

pub fn hash_user_password(pass: String) -> (Vec<u8>, Vec<u8>) {
    // Generate a random salt
    let mut salt = vec![0u8; SALT_LENGTH];
    rand::thread_rng().fill(&mut salt[..]);

    // Create a buffer to store the derived key
    let mut derived_key = vec![0u8; KEY_LENGTH];

    // Perform PBKDF2 hashing
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256, // Use SHA-256 as the hash function
        PBKDF2_ITERATIONS,
        &salt,
        pass.as_bytes(),
        &mut derived_key,
    );

    (salt, derived_key)
}

pub fn verify_user_password(password: String, salt: &[u8], hashed_password: &[u8]) -> bool {
    pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA256,
        PBKDF2_ITERATIONS,
        &salt,
        password.as_bytes(),
        hashed_password,
    )
    .is_ok()
}


pub fn hash_room_password(pass: String) -> String {
    // Generate a random salt
    let mut salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    //Hash password to PHC String
    let password_hash = argon2.hash_password(pass.as_bytes(), &salt).unwrap().to_string();
    
    return password_hash;
}

pub fn verify_room_password(password: String, password_hash: String) -> bool {
    
    let argon2 = Argon2::default();

    let parsed_hash = PasswordHash::new(&password_hash).unwrap();

    //Verify pass hash
    let is_correct = argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok();
    
    return is_correct;
}