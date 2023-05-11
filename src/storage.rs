use leptos::*;
use std::collections::HashMap;

use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use serde::{Deserialize, Serialize};

use crate::sites::Site;

const STORAGE_KEY_DB: &str = "db";
const STORAGE_KEY_LAST_USER: &str = "last_user";

#[derive(Clone, Debug)]
pub struct EncryptedStorage {
    db: HashMap<String, EncryptedSites>,
    pub last_user: String,
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
            .get_item(STORAGE_KEY_DB)
            .ok()
            .flatten()
            .unwrap_or("[]".to_string());

        let last_user = local_storage
            .get_item(STORAGE_KEY_LAST_USER)
            .ok()
            .flatten()
            .unwrap_or("".to_string());

        let db: HashMap<String, EncryptedSites> =
            serde_json::from_str(&db_str).unwrap_or_else(|_| {
                local_storage.clear().unwrap();
                HashMap::default()
            });

        Self { db, last_user }
    }

    pub fn to_local_storage(&self) -> () {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        let json_data = serde_json::to_string(&self.db).unwrap();
        local_storage.set_item(STORAGE_KEY_DB, &json_data).unwrap();
        local_storage
            .set_item(STORAGE_KEY_LAST_USER, &self.last_user)
            .unwrap();
    }

    pub fn decrypt_sites(&self, name: &str, password: &str) -> Result<Vec<Site>, String> {
        if let Some(encrypted_sites) = self.db.get(name) {
            encrypted_sites.decrypt(password).map(|mut sites| {
                sites.sort_by(|s1, s2| s1.site_name.cmp(&s2.site_name));
                sites
            })
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

        let mut user_sites = self
            .db
            .get(login_name)
            .map(|enc_sites| enc_sites.decrypt(storage_password).ok())
            .flatten()
            .unwrap_or(Vec::new());

        if user_sites.iter().any(|s| s.site_name == site_name) {
            // site with that name already exists
            return;
        } else {
            user_sites.push(new_site);
            self.db.insert(
                login_name.to_string(),
                EncryptedSites::from_sites(&user_sites, storage_password),
            );
        }
    }

    pub fn update_site(
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

        let user_sites = self
            .db
            .get(login_name)
            .map(|encrypted_sites| encrypted_sites.decrypt(storage_password).ok())
            .flatten();

        let mut user_sites = match user_sites {
            Some(user_sites) => user_sites,
            None => return,
        };

        let res = user_sites
            .iter_mut()
            .find(|s| s.site_name == site_name)
            .map(|s| *s = new_site.clone());

        if res.is_some() {
            self.db.insert(
                login_name.to_string(),
                EncryptedSites::from_sites(&user_sites, storage_password),
            );
        };
    }

    pub fn delete_site(&mut self, login_name: &str, storage_password: &str, site_name: &str) -> () {
        let user_sites = self
            .db
            .get(login_name)
            .map(|encrypted_sites| encrypted_sites.decrypt(storage_password).ok())
            .flatten();

        let user_sites = match user_sites {
            Some(user_sites) => user_sites
                .into_iter()
                .filter(|s| s.site_name != site_name)
                .collect::<Vec<Site>>(),
            None => return,
        };

        self.db.insert(
            login_name.to_string(),
            EncryptedSites::from_sites(&user_sites, storage_password),
        );
    }
}
