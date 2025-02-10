use shuttle_runtime::SecretStore;
use once_cell::sync::Lazy;
use std::sync::Mutex;

static SECRETS: Lazy<Mutex<Option<SecretStore>>> = Lazy::new(|| Mutex::new(None));

pub fn set_secrets(secret_store: SecretStore) {
    let mut secrets = SECRETS.lock().unwrap();
    *secrets = Some(secret_store);
}

pub fn get_secret(key: &str) -> Option<String> {
    let secrets = SECRETS.lock().unwrap();
    secrets.as_ref()?.get(key).map(|s| s.to_string())
}
