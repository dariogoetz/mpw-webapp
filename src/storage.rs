use std::collections::HashMap;

use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use serde::{Deserialize, Serialize};

use crate::sites::Site;

const STORAGE_KEY: &str = "db";

#[derive(Clone, Debug)]
pub struct EncryptedStorage {
    db: HashMap<String, EncryptedSites>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EncryptedSites(String);

// TODO: proper error types (thiserror?)

impl EncryptedSites {
    pub fn decrypt(&self, password: &str) -> Result<Vec<Site>, String> {
        let mc = new_magic_crypt!(password, 256);
        let json_string = mc
            .decrypt_base64_to_string(&self.0)
            .map_err(|_err| "Decryption failed".to_string())?;
        Ok(serde_json::from_str(&json_string).unwrap_or(Vec::new()))
    }

    pub fn from_sites(sites: &[Site], password: &str) -> EncryptedSites {
        let mc = new_magic_crypt!(password, 256);
        let json_string = serde_json::to_string(sites).unwrap();

        EncryptedSites(mc.encrypt_str_to_base64(&json_string))
    }
}

impl EncryptedStorage {
    pub fn from_local_storage() -> Self {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        let db_str = local_storage
            .get_item(STORAGE_KEY)
            .ok()
            .flatten()
            .unwrap_or("[]".to_string());

        let db: HashMap<String, EncryptedSites> =
            serde_json::from_str(&db_str).unwrap_or_else(|_| {
                local_storage.clear().unwrap();
                HashMap::default()
            });

        Self { db }
    }

    pub fn to_local_storage(&self) -> () {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        let json_data = serde_json::to_string(&self.db).unwrap();
        local_storage.set_item(STORAGE_KEY, &json_data).unwrap();
    }

    pub fn names(&self) -> Vec<String> {
        self.db.iter().map(|(name, _)| name.to_string()).collect()
    }

    pub fn decrypt_sites(&self, name: &str, password: &str) -> Result<Vec<Site>, String> {
        if let Some(encrypted_sites) = self.db.get(name) {
            encrypted_sites.decrypt(password)
        } else {
            return Ok(Vec::new());
        }
    }

    pub fn add_site(
        &mut self,
        login_name: &str,
        storage_password: &str,
        site_name: &str,
        counter: i32,
        pw_type: &str,
    ) -> () {
        let new_site = Site {
            site_name: site_name.to_string(),
            counter,
            password_type: pw_type.to_string(),
        };

        let encrypted_sites = self.db.get(login_name);
        let mut user_sites = match encrypted_sites {
            Some(enc_sites) => {
                let user_sites = enc_sites.decrypt(storage_password);
                if user_sites.is_err() {
                    // do not do anything
                    return;
                }
                user_sites.unwrap()
            }
            None => Vec::new(),
        };

        // TODO: in case site with that site_name exists -> update

        user_sites.push(new_site);

        self.db.insert(
            login_name.to_string(),
            EncryptedSites::from_sites(&user_sites, storage_password),
        );
    }

    // TODO: delete_site
}
