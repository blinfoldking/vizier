use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;

#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct ApiKey {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub key_hash: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

#[async_trait::async_trait]
pub trait UserStorage {
    async fn get_user(&self, username: &str) -> Result<Option<User>>;
    async fn create_user(&self, username: &str, password_hash: &str) -> Result<User>;
    async fn update_password(&self, user_id: &str, password_hash: &str) -> Result<()>;
    async fn user_exists(&self) -> Result<bool>;

    async fn create_api_key(
        &self,
        user_id: &str,
        name: &str,
        key_hash: &str,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<ApiKey>;
    async fn get_api_key_by_hash(&self, key_hash: &str) -> Result<Option<ApiKey>>;
    async fn list_api_keys(&self, user_id: &str) -> Result<Vec<ApiKey>>;
    async fn delete_api_key(&self, key_id: &str) -> Result<()>;
    async fn update_api_key_last_used(&self, key_id: &str) -> Result<()>;
}
