use std::sync::Arc;

use anyhow::Result;
use rig::Embed;
use rig::completion::ToolDefinition;
use rig::embeddings::EmbeddingsBuilder;
use rig::tool::Tool;
use rig::vector_store::request::VectorSearchRequest;
use rig::{
    client::{EmbeddingsClient, Nothing},
    providers::ollama,
    vector_store::VectorStoreIndex,
};
use rig_sqlite::{
    Column, ColumnValue, SqliteVectorIndex, SqliteVectorStore, SqliteVectorStoreTable,
};
use rusqlite::ffi::sqlite3_api_routines;
use rusqlite::ffi::{sqlite3, sqlite3_auto_extension};
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use slugify::slugify;
use sqlite_vec::sqlite3_vec_init;
use tokio_rusqlite::Connection;

use crate::config::VectorMemoryConfig;
use crate::error::{VizierError, error};

type SqliteExtensionFn =
    unsafe extern "C" fn(*mut sqlite3, *mut *mut i8, *const sqlite3_api_routines) -> i32;

// TODO: handle openai embedder
pub async fn init_vector_memory(
    workspace: String,
    config: VectorMemoryConfig,
) -> Result<(
    SqliteVectorIndex<ollama::EmbeddingModel, Memory>,
    MemoryRead,
    MemoryWrite,
)> {
    unsafe {
        sqlite3_auto_extension(Some(std::mem::transmute::<*const (), SqliteExtensionFn>(
            sqlite3_vec_init as *const (),
        )));
    }

    let conn = Connection::open(format!("{}/db/memory.db", workspace)).await?;

    let embedder = match &*config.model.provider {
        _ => {
            let client: ollama::Client = ollama::Client::builder()
                .base_url(config.model.base_url)
                .api_key(Nothing)
                .build()
                .unwrap();

            client.embedding_model(config.model.name)
        }
    };

    let store = SqliteVectorStore::new(conn, &embedder.clone()).await?;

    Ok((
        store.clone().index(embedder.clone()),
        MemoryRead::new(store.clone(), embedder.clone()),
        MemoryWrite::new(store.clone(), embedder.clone()),
    ))
}

#[derive(Embed, Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default)]
pub struct Memory {
    pub id: String,
    pub title: String,
    #[serde(skip)]
    #[embed]
    pub content: String,
}

impl SqliteVectorStoreTable for Memory {
    fn name() -> &'static str {
        "documents"
    }

    fn schema() -> Vec<Column> {
        vec![
            Column::new("id", "TEXT PRIMARY KEY"),
            Column::new("title", "TEXT"),
            Column::new("content", "TEXT"),
        ]
    }

    fn id(&self) -> String {
        self.id.clone()
    }

    fn column_values(&self) -> Vec<(&'static str, Box<dyn ColumnValue>)> {
        vec![
            ("id", Box::new(self.id.clone())),
            ("title", Box::new(self.title.clone())),
            ("content", Box::new(self.content.clone())),
        ]
    }
}

pub type MemoryRead = ReadVectorMemory;
pub struct ReadVectorMemory(
    SqliteVectorStore<ollama::EmbeddingModel, Memory>,
    ollama::EmbeddingModel,
);

impl MemoryRead {
    fn new(
        store: SqliteVectorStore<ollama::EmbeddingModel, Memory>,
        embedder: ollama::EmbeddingModel,
    ) -> Self {
        Self(store, embedder)
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct MemoryReadArgs {
    #[schemars(description = "Terms, keywords, or prompt to search")]
    pub query: String,
}

impl Tool for MemoryRead {
    const NAME: &'static str = "memory_read";
    type Error = VizierError;
    type Args = MemoryReadArgs;
    type Output = Vec<Memory>;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let parameters = serde_json::to_value(schema_for!(Self::Args)).unwrap();

        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Search your memory for informations".into(),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        log::info!("read_memory: {}", args.query.clone());

        let req = VectorSearchRequest::builder()
            .query(args.query)
            .samples(5)
            .build()
            .unwrap();

        let embedder = self.1.clone();
        let store = self.0.clone();
        let index = store.index(embedder);
        match index.top_n::<Memory>(req).await {
            Err(err) => crate::error::error("read_memory", err),
            Ok(data) => return Ok(data.iter().map(|(_, _, memory)| memory.clone()).collect()),
        }
    }
}

pub type MemoryWrite = WriteVectorMemory;
pub struct WriteVectorMemory(
    SqliteVectorStore<ollama::EmbeddingModel, Memory>,
    ollama::EmbeddingModel,
);

impl MemoryWrite {
    fn new(
        store: SqliteVectorStore<ollama::EmbeddingModel, Memory>,
        model: ollama::EmbeddingModel,
    ) -> Self {
        Self(store, model)
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema, Clone)]
pub struct MemoryWriteArgs {
    pub title: String,

    #[schemars(description = "details of the memory")]
    pub content: String,
}

impl Tool for MemoryWrite {
    const NAME: &'static str = "memory_write";
    type Error = VizierError;
    type Args = MemoryWriteArgs;
    type Output = ();

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let parameters = serde_json::to_value(schema_for!(Self::Args)).unwrap();

        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Create and record a new memory".into(),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        log::info!("write_memory: {:?}", args.clone());

        let document = EmbeddingsBuilder::new(self.1.clone())
            .document(Memory {
                id: slugify!(&args.title.clone()),
                title: args.title,
                content: args.content,
            })
            .unwrap()
            .build()
            .await;

        if let Err(err) = document {
            return error("memory_write", err);
        }

        let document = document.unwrap();
        if let Err(err) = self.0.add_rows(document).await {
            return error("memory_write", err);
        }

        Ok(())
    }
}
