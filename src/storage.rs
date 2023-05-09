use std::collections::HashMap;

use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Site {
    pub site_name: String,
    pub counter: i32,
    pub password_type: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct UserSites {
    name: String,
    sites: Vec<Site>,
}

#[derive(Clone, Debug)]
pub struct EncryptedStorage {
    db: HashMap<String, EncryptedSites>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EncryptedSites(String);

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

const STORAGE_KEY: &str = "db";

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

    pub fn names(&self) -> Vec<String> {
        self.db.iter().map(|(name, _)| name.to_string()).collect()
    }

    pub fn get_user_sites(&self, name: &str, password: &str) -> Result<Vec<Site>, String> {
        let encrypted_sites = self.db.get(name);
        if encrypted_sites.is_none() {
            return Ok(Vec::new());
        };
        let encrypted_sites = encrypted_sites.unwrap();

        encrypted_sites.decrypt(password)
    }

    pub fn to_local_storage(&self) -> () {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        let json_data = serde_json::to_string(&self.db).unwrap();
        local_storage.set_item(STORAGE_KEY, &json_data).unwrap();
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

        let mut user_sites = self
            .db
            .get(login_name)
            .map(|encrypted_sites| encrypted_sites.decrypt(storage_password).ok())
            .flatten()
            .unwrap_or(Vec::new());

        user_sites.push(new_site);

        self.db.insert(
            login_name.to_string(),
            EncryptedSites::from_sites(&user_sites, storage_password),
        );
    }
}
