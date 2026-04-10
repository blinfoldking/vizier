use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    storage::{
        fs::FileSystemStorage,
        user::{ApiKey, User, UserStorage},
    },
    utils::build_path,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct UserStore {
    users: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct ApiKeyStore {
    keys: Vec<ApiKey>,
}

const USERS_PATH: &str = "users/users.json";
const API_KEYS_PATH: &str = "users/api_keys.json";

#[async_trait::async_trait]
impl UserStorage for FileSystemStorage {
    async fn get_user(&self, username: &str) -> Result<Option<User>> {
        let path = build_path(&self.workspace, &[USERS_PATH]);

        if !path.exists() {
            return Ok(None);
        }

        let raw = std::fs::read_to_string(&path)?;
        let store: UserStore = serde_json::from_str(&raw)?;

        Ok(store.users.into_iter().find(|u| u.username == username))
    }

    async fn create_user(&self, username: &str, password_hash: &str) -> Result<User> {
        let path = build_path(&self.workspace, &[USERS_PATH]);
        let _ = std::fs::create_dir_all(path.parent().unwrap())?;

        let mut store = if path.exists() {
            let raw = std::fs::read_to_string(&path)?;
            serde_json::from_str(&raw).unwrap_or_default()
        } else {
            UserStore::default()
        };

        let user = User {
            user_id: Uuid::new_v4().to_string(),
            username: username.to_string(),
            password_hash: password_hash.to_string(),
            created_at: Utc::now(),
        };

        store.users.push(user.clone());

        std::fs::write(path, serde_json::to_string_pretty(&store)?)?;

        Ok(user)
    }

    async fn update_password(&self, user_id: &str, password_hash: &str) -> Result<()> {
        let path = build_path(&self.workspace, &[USERS_PATH]);

        if !path.exists() {
            return Err(anyhow::anyhow!("User store not found"));
        }

        let raw = std::fs::read_to_string(&path)?;
        let mut store: UserStore = serde_json::from_str(&raw)?;

        if let Some(user) = store.users.iter_mut().find(|u| u.user_id == user_id) {
            user.password_hash = password_hash.to_string();
        } else {
            return Err(anyhow::anyhow!("User not found"));
        }

        std::fs::write(path, serde_json::to_string_pretty(&store)?)?;

        Ok(())
    }

    async fn user_exists(&self) -> Result<bool> {
        let path = build_path(&self.workspace, &[USERS_PATH]);

        if !path.exists() {
            return Ok(false);
        }

        let raw = std::fs::read_to_string(&path)?;
        let store: UserStore = serde_json::from_str(&raw).unwrap_or_default();

        Ok(!store.users.is_empty())
    }

    async fn create_api_key(
        &self,
        user_id: &str,
        name: &str,
        key_hash: &str,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<ApiKey> {
        let path = build_path(&self.workspace, &[API_KEYS_PATH]);
        let _ = std::fs::create_dir_all(path.parent().unwrap())?;

        let mut store = if path.exists() {
            let raw = std::fs::read_to_string(&path)?;
            serde_json::from_str(&raw).unwrap_or_default()
        } else {
            ApiKeyStore::default()
        };

        let api_key = ApiKey {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            name: name.to_string(),
            key_hash: key_hash.to_string(),
            expires_at,
            created_at: Utc::now(),
            last_used_at: None,
        };

        store.keys.push(api_key.clone());

        std::fs::write(path, serde_json::to_string_pretty(&store)?)?;

        Ok(api_key)
    }

    async fn get_api_key_by_hash(&self, key_hash: &str) -> Result<Option<ApiKey>> {
        let path = build_path(&self.workspace, &[API_KEYS_PATH]);

        if !path.exists() {
            return Ok(None);
        }

        let raw = std::fs::read_to_string(&path)?;
        let store: ApiKeyStore = serde_json::from_str(&raw)?;

        Ok(store
            .keys
            .into_iter()
            .find(|k| k.key_hash == key_hash && k.expires_at.map_or(true, |exp| exp > Utc::now())))
    }

    async fn list_api_keys(&self, user_id: &str) -> Result<Vec<ApiKey>> {
        let path = build_path(&self.workspace, &[API_KEYS_PATH]);

        if !path.exists() {
            return Ok(vec![]);
        }

        let raw = std::fs::read_to_string(&path)?;
        let store: ApiKeyStore = serde_json::from_str(&raw)?;

        Ok(store
            .keys
            .into_iter()
            .filter(|k| k.user_id == user_id)
            .collect())
    }

    async fn delete_api_key(&self, key_id: &str) -> Result<()> {
        let path = build_path(&self.workspace, &[API_KEYS_PATH]);

        if !path.exists() {
            return Err(anyhow::anyhow!("API key store not found"));
        }

        let raw = std::fs::read_to_string(&path)?;
        let mut store: ApiKeyStore = serde_json::from_str(&raw)?;

        store.keys.retain(|k| k.id != key_id);

        std::fs::write(path, serde_json::to_string_pretty(&store)?)?;

        Ok(())
    }

    async fn update_api_key_last_used(&self, key_id: &str) -> Result<()> {
        let path = build_path(&self.workspace, &[API_KEYS_PATH]);

        if !path.exists() {
            return Err(anyhow::anyhow!("API key store not found"));
        }

        let raw = std::fs::read_to_string(&path)?;
        let mut store: ApiKeyStore = serde_json::from_str(&raw)?;

        if let Some(key) = store.keys.iter_mut().find(|k| k.id == key_id) {
            key.last_used_at = Some(Utc::now());
        } else {
            return Err(anyhow::anyhow!("API key not found"));
        }

        std::fs::write(path, serde_json::to_string_pretty(&store)?)?;

        Ok(())
    }
}
