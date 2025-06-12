use std::collections::HashMap;
use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit, Nonce};
use base64::{engine::general_purpose, Engine};
use crate::user_model::{DmSessionEncryption_Key};




pub fn decrypt_dm_message(receiving_msg_key: [u8;32], ciphertext: &str) -> String {

    let nonce_slice = [0u8;12]; //Zeroes Nonce
    let key = Key::<Aes256Gcm>::from_slice(&receiving_msg_key);
    let cipher = Aes256Gcm::new(&key);
    let nonce: &sha2::digest::generic_array::GenericArray<u8, sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UTerm, sha2::digest::consts::B1>, sha2::digest::consts::B1>, sha2::digest::consts::B0>, sha2::digest::consts::B0>> = Nonce::from_slice(&nonce_slice);

    //Decoding base 64 ciphertext to bytes
    let ciphertext_bytes = general_purpose::STANDARD.decode(ciphertext).unwrap();
    //Decrypting to plaintext
    let plaintext_res = cipher.decrypt(&nonce, ciphertext_bytes.as_ref());
    match plaintext_res{
        Ok(plaintext_bytes) => {
            let plaintext_str_res = String::from_utf8(plaintext_bytes);
            match plaintext_str_res{
                Ok(plaintext) => {
                    //Return the Decrypted Plaintext
                    return plaintext;
                }
                Err(err) => {
                    //utf8 to string conversion error to be handled
                    return "".to_string();
                }
            }
        }
        Err(err) => {
            //decryption error to be handled
            return "".to_string();
        }
    }
}




pub fn decrypt_dm_chats_session(
    dm_session_keys: HashMap<String, DmSessionEncryption_Key>,
    mut dm_chats_data: HashMap<String, Vec<(String, String, String, String, bool, String)>>
) -> HashMap<String, Vec<(String, String, String, String, bool, String)>> {

    for (_user, messages_data) in dm_chats_data.iter_mut() {
        let key_slice = dm_session_keys.get(_user).unwrap().key;
        let nonce_slice = dm_session_keys.get(_user).unwrap().nonce;

        let key = Key::<Aes256Gcm>::from_slice(&key_slice);
        let cipher = Aes256Gcm::new(&key);
        let nonce: &sha2::digest::generic_array::GenericArray<u8, sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UTerm, sha2::digest::consts::B1>, sha2::digest::consts::B1>, sha2::digest::consts::B0>, sha2::digest::consts::B0>> = Nonce::from_slice(&nonce_slice);

        for message in messages_data.iter_mut() {
            //Decoding base 64 ciphertext to bytes
            let ciphertext_bytes = general_purpose::STANDARD.decode(&message.1).unwrap();
            //Decrypting to plaintext
            let plaintext_res = cipher.decrypt(&nonce, ciphertext_bytes.as_ref());
            match plaintext_res{
                Ok(plaintext_bytes) => {
                    let plaintext_str_res = String::from_utf8(plaintext_bytes);
                    match plaintext_str_res{
                        Ok(plaintext) => {
                            //UPDATING AS DECRYPTED MESSAGE
                            message.1 = plaintext;
                        }
                        Err(err) => {
                            //utf8 to string conversion error to be handled
                        }
                    }
                }
                Err(err) => {
                    //decryption error to be handled
                }
            }
        }
    }

    return dm_chats_data;
}



//------------------ROOMS DECRYPTION-------------------------------------------------


pub fn decrypt_senderkey_message(receiving_msg_key: [u8;32], encrypted_sender_key_bytes: &[u8]) -> Vec<u8> {
    let nonce_slice = [0u8;12]; //Zeroes Nonce
    let key = Key::<Aes256Gcm>::from_slice(&receiving_msg_key);
    let cipher = Aes256Gcm::new(&key);
    let nonce: &sha2::digest::generic_array::GenericArray<u8, sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UTerm, sha2::digest::consts::B1>, sha2::digest::consts::B1>, sha2::digest::consts::B0>, sha2::digest::consts::B0>> = Nonce::from_slice(&nonce_slice);
    //Decrypting
    let decrypted_res = cipher.decrypt(&nonce, encrypted_sender_key_bytes);
    match decrypted_res{
        Ok(decrypted_bytes) => {
            return decrypted_bytes;
        }
        Err(err) => {
            //decryption error to be handled
            return vec![0u8]
        }
    }
}


pub fn decrypt_room_message(receiving_msg_key: [u8;32], ciphertext: &str) -> String {

    let nonce_slice = [0u8;12]; //Zeroes Nonce
    let key = Key::<Aes256Gcm>::from_slice(&receiving_msg_key);
    let cipher = Aes256Gcm::new(&key);
    let nonce: &sha2::digest::generic_array::GenericArray<u8, sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UTerm, sha2::digest::consts::B1>, sha2::digest::consts::B1>, sha2::digest::consts::B0>, sha2::digest::consts::B0>> = Nonce::from_slice(&nonce_slice);

    //Decoding base 64 ciphertext to bytes
    let ciphertext_bytes = general_purpose::STANDARD.decode(ciphertext).unwrap();
    //Decrypting to plaintext
    let plaintext_res = cipher.decrypt(&nonce, ciphertext_bytes.as_ref());
    match plaintext_res{
        Ok(plaintext_bytes) => {
            let plaintext_str_res = String::from_utf8(plaintext_bytes.clone());
            match plaintext_str_res{
                Ok(plaintext) => {
                    //Return the Decrypted Plaintext
                    return plaintext;
                }
                Err(err) => {
                    //utf8 to string conversion error to be handled
                    return "".to_string();
                }
            }
        }
        Err(err) => {
            //decryption error to be handled
            return "".to_string();
        }
    }
}