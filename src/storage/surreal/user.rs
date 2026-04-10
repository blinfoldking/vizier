use anyhow::Result;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::storage::{
    surreal::SurrealStorage,
    user::{ApiKey, User, UserStorage},
};

#[async_trait::async_trait]
impl UserStorage for SurrealStorage {
    async fn get_user(&self, username: &str) -> Result<Option<User>> {
        let mut result = self
            .conn
            .query("SELECT * FROM user WHERE username = $username")
            .bind(("username", username.to_string()))
            .await?;

        let users: Vec<User> = result.take(0)?;
        Ok(users.into_iter().next())
    }

    async fn create_user(&self, username: &str, password_hash: &str) -> Result<User> {
        let user = User {
            user_id: Uuid::new_v4().to_string(),
            username: username.to_string(),
            password_hash: password_hash.to_string(),
            created_at: Utc::now(),
        };

        let created: Option<User> = self
            .conn
            .create(("user", user.user_id.clone()))
            .content(user)
            .await?;

        created.ok_or_else(|| anyhow::anyhow!("Failed to create user"))
    }

    async fn update_password(&self, user_id: &str, password_hash: &str) -> Result<()> {
        let _: Option<User> = self
            .conn
            .update(("user", user_id.to_string()))
            .merge(serde_json::json!({ "password_hash": password_hash.to_string() }))
            .await?;

        Ok(())
    }

    async fn user_exists(&self) -> Result<bool> {
        let mut result = self
            .conn
            .query("SELECT count() FROM user GROUP BY count")
            .await?;
        let count: Option<i64> = result.take((0, "count"))?;
        Ok(count.map_or(false, |c| c > 0))
    }

    async fn create_api_key(
        &self,
        user_id: &str,
        name: &str,
        key_hash: &str,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<ApiKey> {
        let api_key = ApiKey {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            name: name.to_string(),
            key_hash: key_hash.to_string(),
            expires_at,
            created_at: Utc::now(),
            last_used_at: None,
        };

        let created: Option<ApiKey> = self
            .conn
            .create(("api_key", api_key.id.clone()))
            .content(api_key)
            .await?;

        created.ok_or_else(|| anyhow::anyhow!("Failed to create API key"))
    }

    async fn get_api_key_by_hash(&self, key_hash: &str) -> Result<Option<ApiKey>> {
        let mut result = self
            .conn
            .query("SELECT * FROM api_key WHERE key_hash = $hash AND (expires_at IS NONE OR expires_at > $now)")
            .bind(("hash", key_hash.to_string()))
            .bind(("now", Utc::now()))
            .await?;

        let keys: Vec<ApiKey> = result.take(0)?;
        Ok(keys.into_iter().next())
    }

    async fn list_api_keys(&self, user_id: &str) -> Result<Vec<ApiKey>> {
        let mut result = self
            .conn
            .query("SELECT * FROM api_key WHERE user_id = $user_id")
            .bind(("user_id", user_id.to_string()))
            .await?;

        let keys: Vec<ApiKey> = result.take(0)?;
        Ok(keys)
    }

    async fn delete_api_key(&self, key_id: &str) -> Result<()> {
        let _: Option<ApiKey> = self.conn.delete(("api_key", key_id.to_string())).await?;

        Ok(())
    }

    async fn update_api_key_last_used(&self, key_id: &str) -> Result<()> {
        let _: Option<ApiKey> = self
            .conn
            .update(("api_key", key_id.to_string()))
            .merge(serde_json::json!({ "last_used_at": Utc::now() }))
            .await?;

        Ok(())
    }
}
