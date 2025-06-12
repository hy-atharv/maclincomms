use std::collections::HashMap;
use aes_gcm::{aead::Aead, AeadCore, Aes256Gcm, Key, KeyInit, Nonce};
use base64::{engine::general_purpose, Engine};
use chrono::{DateTime, Duration, Utc};
use disk_persist::DiskPersist;
use ed25519_dalek::{ed25519::signature::SignerMut, Signature, SignatureError, SigningKey, VerifyingKey};
use hkdf::Hkdf;
use rand_core::OsRng;
use sha2::Sha256;
use x25519_dalek::StaticSecret;

use crate::{crypto::room_keys::derive_roommessage_key, user_model::DmSessionEncryption_Key};

use super::dm_keys::derive_message_key;


pub fn encrypt_dm_message(sending_chain_key: [u8;32], plaintext: &str) -> String {
    let msg_key_slice = derive_message_key(sending_chain_key);
    let key = Key::<Aes256Gcm>::from_slice(&msg_key_slice);
    let nonce_slice = [0u8;12];
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(&nonce_slice);
    let ciphertext_res = cipher.encrypt(&nonce, plaintext.as_bytes());
    match ciphertext_res{
        Ok(ciphertext_bytes) => {
            //Encoding ciphertext bytes to base 64
            let ciphertext = general_purpose::STANDARD.encode(ciphertext_bytes);
            //RETURNING ENCRYPTED MESSAGE (Base64 Encoded)
            return ciphertext;
        }
        Err(err) => {
            //decryption error to be handled
            return "".to_string();
        }
    } 
}

pub fn encrypt_dm_chats_session(
    dm_session_keys: &mut HashMap<String, DmSessionEncryption_Key>,
    dm_chats_data: &mut HashMap<String, Vec<(String, String, String, String, bool, String)>>
) {


    //Users in dm_chats_data (can have new users too)
    let usernames: Vec<String> = dm_chats_data.keys().cloned().collect();

    //Users already existing in session_keys list
    let old_chat_users: Vec<String> = dm_session_keys.clone().into_keys().collect();

    //Check if all usernames that exist in session_keys_list, exist in recent chats data too
    //If some dont, generate session keys for them with timestamps and add the same timestamp on their messages
    let new_chat_users: Vec<String> = usernames.clone()
            .into_iter()
            .filter(|user| !dm_session_keys.contains_key(user))
            .collect();
    
    for user in new_chat_users{
        let s_key = generate_session_key();
        let utc_now = Utc::now();
        let utc_timestamp = utc_now.format("%Y-%m-%d %H:%M:%S%z").to_string();
        let s_key_nonce = Aes256Gcm::generate_nonce(&mut OsRng); //96 Bits Unique Per User Dm Session Key
        let s_key_nonce_bytes: [u8;12] = s_key_nonce.into();

        //Insert to Hashmap
        dm_session_keys.insert(
            user.clone(), 
            DmSessionEncryption_Key{
                key: s_key,
                timestamp: utc_timestamp.clone(),
                nonce: s_key_nonce_bytes
            }
        );

        //Adding same timestamp to messages
        let chats_data = dm_chats_data.get_mut(&user);
        match chats_data{
            Some(chats) => {
                for chat in chats.iter_mut(){
                    chat.3 = utc_timestamp.clone();
                }
            }
            None => {
                //no chats data
            }
        }
    }

    //Check utc timestamps of keys and for keys older than 24 hours (utc), generate new keys with new timestamps
    //Add the same timestamp on their messages
    //Since we already gave new keys to new_chat users, we can skip checking their keys
    for user in old_chat_users{
        let key_data = dm_session_keys.get(&user);
        match key_data{
            Some(data) => {
                let key_timestamp = data.timestamp.clone();
                if let Ok(parsed_time) = DateTime::parse_from_str(&key_timestamp, "%Y-%m-%d %H:%M:%S%z") {
                    let parsed_utc = parsed_time.with_timezone(&Utc);
                    let utc_now = Utc::now();
                    let utc_timestamp = utc_now.format("%Y-%m-%d %H:%M:%S%z").to_string();

                    if utc_now - parsed_utc >= Duration::hours(24) {
                        // Key is more than 24 hours old
                        //Derive new by HKDF
                        let hk = Hkdf::<Sha256>::new(None, &data.key);
                        let mut derived_key = [0u8;32];
                        hk.expand(&[0x02], &mut derived_key).unwrap();
                        let s_key_nonce = Aes256Gcm::generate_nonce(&mut OsRng); //96 Bits Unique Per User Dm Session Key
                        let s_key_nonce_bytes: [u8;12] = s_key_nonce.into();

                        //Inserting to dm_session_keys hashmap
                        dm_session_keys.insert(
                            user.clone(), 
                            DmSessionEncryption_Key{
                                key: derived_key,
                                timestamp: utc_timestamp.clone(),
                                nonce: s_key_nonce_bytes
                            }
                        );

                        //Adding same timestamp to messages
                        let chats_data = dm_chats_data.get_mut(&user);
                        match chats_data{
                            Some(chats) => {
                                for chat in chats.iter_mut().rev(){
                                    if !chat.3.is_empty(){
                                        break;
                                    }
                                    chat.3 = utc_timestamp.clone();
                                }
                            }
                            None => {
                                //no chats data
                            }
                        }
                    }
                    else if utc_now - parsed_utc < Duration::hours(24){
                        //Key is not expired and still can be used
                        //Adding same timestamp to new messages
                        let chats_data = dm_chats_data.get_mut(&user);
                        match chats_data{
                            Some(chats) => {
                                for chat in chats.iter_mut().rev(){
                                    if !chat.3.is_empty(){
                                        break;
                                    }
                                    chat.3 = key_timestamp.clone();
                                }
                            }
                            None => {
                                //no chats data
                            }
                        }
                    }
                }
            }
            None => {
                let s_key = generate_session_key();
                let utc_now = Utc::now();
                let utc_timestamp = utc_now.format("%Y-%m-%d %H:%M:%S%z").to_string();
                let s_key_nonce = Aes256Gcm::generate_nonce(&mut OsRng); //96 Bits Unique Per User Dm Session Key
                let s_key_nonce_bytes: [u8;12] = s_key_nonce.into();

                //Insert to Hashmap
                dm_session_keys.insert(
                    user, 
                    DmSessionEncryption_Key{
                        key: s_key,
                        timestamp: utc_timestamp,
                        nonce: s_key_nonce_bytes
                    }
                );
            }
        }
    }

    //Rewriting updated session_keys_list to persistent storage
    let persistent_dm_session_keys: DiskPersist<HashMap<String, DmSessionEncryption_Key>> = DiskPersist::init("persistent-dms-session-keys").unwrap();
    persistent_dm_session_keys.write(&dm_session_keys).unwrap();


    //Encrypting Chats
    for (_user, messages_data) in dm_chats_data.iter_mut() {
        let key_slice = dm_session_keys.get(_user).unwrap().key;
        let nonce_slice = dm_session_keys.get(_user).unwrap().nonce;

        let key = Key::<Aes256Gcm>::from_slice(&key_slice);
        let cipher = Aes256Gcm::new(&key);
        let nonce = Nonce::from_slice(&nonce_slice);

        for message in messages_data.iter_mut() {
            //Encrypting to ciphertext
            let ciphertext_res = cipher.encrypt(&nonce, message.1.as_bytes());
            match ciphertext_res{
                Ok(ciphertext_bytes) => {
                    //Encoding ciphertext bytes to base 64
                    let ciphertext = general_purpose::STANDARD.encode(ciphertext_bytes);
                    //UPDATING AS ENCRYPTED MESSAGE (Base64 Encoded)
                    message.1 = ciphertext;
                }
                Err(err) => {
                    //encryption error to be handled
                }
            }
        }
    }
}

pub fn generate_session_key() -> [u8;32] {
    let csprng = OsRng;
    let session_key = StaticSecret::random_from_rng(csprng);
    let session_key_32b = session_key.to_bytes();
    return session_key_32b;
}



//------------------ROOMS ENCRYPTION-------------------------------------------------

pub fn encrypt_senderkey_message(sending_chain_key: [u8;32], sender_key_bytes: &[u8]) -> Vec<u8> {
    let msg_key_slice = derive_message_key(sending_chain_key);
    let key = Key::<Aes256Gcm>::from_slice(&msg_key_slice);
    let nonce_slice = [0u8;12];
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(&nonce_slice);
    let ciphertext_res = cipher.encrypt(&nonce, sender_key_bytes);
    match ciphertext_res{
        Ok(ciphertext_bytes) => {
            return ciphertext_bytes;
        }
        Err(err) => {
            //encryption error to be handled
            return vec![0u8];
        }
    } 
}

pub fn encrypt_room_message(sending_chain_key: [u8;32], plaintext: &str) -> String {
    let msg_key_slice = derive_roommessage_key(sending_chain_key);
    let key = Key::<Aes256Gcm>::from_slice(&msg_key_slice);
    let nonce_slice = [0u8;12];
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(&nonce_slice);
    let ciphertext_res = cipher.encrypt(&nonce, plaintext.as_bytes());
    match ciphertext_res{
        Ok(ciphertext_bytes) => {
            //Encoding ciphertext bytes to base 64
            let ciphertext = general_purpose::STANDARD.encode(ciphertext_bytes);
            //RETURNING ENCRYPTED MESSAGE (Base64 Encoded)
            return ciphertext;
        }
        Err(err) => {
            //decryption error to be handled
            return "".to_string();
        }
    } 
}

pub fn sign_room_ciphertext(signature_priv_key: [u8;32], ciphertext: &str) -> String {
    let mut signing_key = SigningKey::from_bytes(&signature_priv_key);
    let signature = signing_key.sign(ciphertext.as_bytes());
    let signature_bytes = signature.to_bytes();
    let signature_b64 = general_purpose::STANDARD.encode(signature_bytes);
    return signature_b64;
}

pub fn verify_room_ciphertext(signature_pub_key: [u8;32], ciphertext: &str, signature_b64: &str) -> Result<(), SignatureError> {
    let signature_bytes: [u8;64] = general_purpose::STANDARD.decode(signature_b64).unwrap().try_into().unwrap();
    let signature = Signature::from_bytes(&signature_bytes); //64 Bytes ED25519 Small Signatures
    let verifying_key = VerifyingKey::from_bytes(&signature_pub_key)?;
    verifying_key.verify_strict(ciphertext.as_bytes(), &signature)?;
    return Ok(());
}