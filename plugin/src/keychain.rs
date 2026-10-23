//! Small secrets in the macOS Keychain: one generic password per key, service = the app's bundle id, account = the key.
//! Never a file, never the page's storage.

use keyring::Entry;
use krizaka_tauri_kit::secrets::SecretStore;

pub struct Keychain {
    service: String,
}

impl Keychain {
    pub fn new(service: &str) -> Self {
        Keychain {
            service: service.to_string(),
        }
    }

    fn entry(&self, key: &str) -> Result<Entry, String> {
        Entry::new(&self.service, key).map_err(|e| e.to_string())
    }
}

impl SecretStore for Keychain {
    fn get(&self, key: &str) -> Result<Option<String>, String> {
        match self.entry(key)?.get_password() {
            Ok(value) => Ok(Some(value)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }

    fn set(&self, key: &str, value: &str) -> Result<(), String> {
        self.entry(key)?
            .set_password(value)
            .map_err(|e| e.to_string())
    }

    fn remove(&self, key: &str) -> Result<(), String> {
        match self.entry(key)?.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}
