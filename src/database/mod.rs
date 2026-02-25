use std::sync::Arc;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, RocksDb};

pub mod schema;

#[derive(Debug, Clone)]
pub struct VizierDatabases {
    pub conn: Arc<Surreal<Db>>,
}

impl VizierDatabases {
    pub async fn new(workspace: String) -> Result<Self> {
        let db = Surreal::new::<RocksDb>(format!("{workspace}/vizier.db")).await?;
        db.use_ns("vizier").use_db("v1").await?;

        let res = Self { conn: Arc::new(db) };

        Ok(res)
    }
}
