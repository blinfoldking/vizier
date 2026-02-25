use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use rig::Embed;
use rig::completion::ToolDefinition;
use rig::embeddings::EmbeddingsBuilder;
use rig::tool::Tool;
use rig::vector_store::request::VectorSearchRequest;
use rig::{
    client::{EmbeddingsClient, Nothing},
    providers::ollama,
    vector_store::{InsertDocuments, VectorStoreIndex},
};

use rig_surrealdb::SurrealVectorStore;
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use slugify::slugify;
use surrealdb::engine::local::Db;

use crate::config::VectorMemoryConfig;
use crate::database::schema::Memory;
use crate::dependencies::VizierDependencies;
use crate::error::{VizierError, error};

// TODO: handle openai embedder
pub async fn init_vector_memory(
    workspace: String,
    config: VectorMemoryConfig,
    deps: VizierDependencies,
) -> Result<(MemoryRead, MemoryWrite)> {
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

    let store = Arc::new(SurrealVectorStore::with_defaults(
        embedder.clone(),
        (*deps.database.conn).clone(),
    ));

    Ok((
        MemoryRead::new(store.clone()),
        MemoryWrite::new(store.clone(), embedder.clone()),
    ))
}

pub type MemoryRead = ReadVectorMemory;
pub struct ReadVectorMemory(Arc<SurrealVectorStore<Db, ollama::EmbeddingModel>>);

impl MemoryRead {
    fn new(store: Arc<SurrealVectorStore<Db, ollama::EmbeddingModel>>) -> Self {
        Self(store)
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

        match self.0.top_n::<Memory>(req).await {
            Err(err) => crate::error::error("read_memory", err),
            Ok(data) => return Ok(data.iter().map(|(_, _, memory)| memory.clone()).collect()),
        }
    }
}

pub type MemoryWrite = WriteVectorMemory;
pub struct WriteVectorMemory(
    Arc<SurrealVectorStore<Db, ollama::EmbeddingModel>>,
    ollama::EmbeddingModel,
);

impl MemoryWrite {
    fn new(
        store: Arc<SurrealVectorStore<Db, ollama::EmbeddingModel>>,
        model: ollama::EmbeddingModel,
    ) -> Self {
        Self(store, model)
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema, Clone)]
pub struct MemoryWriteArgs {
    #[schemars(description = "title of the memory")]
    pub title: String,

    #[schemars(description = "details of the memory")]
    pub content: String,
}

impl Tool for MemoryWrite {
    const NAME: &'static str = "memory_write";
    type Error = VizierError;
    type Args = MemoryWriteArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let parameters = serde_json::to_value(schema_for!(Self::Args)).unwrap();

        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "write or update a new memory".into(),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let slug = slugify!(&args.title).to_string();
        log::info!("write_memory: {:?}", slug.clone());

        let document = EmbeddingsBuilder::new(self.1.clone())
            .document(Memory {
                slug: slug.clone(),
                title: args.title,
                content: args.content,
                timestamp: Utc::now(),
            })
            .unwrap()
            .build()
            .await;

        if let Err(err) = document {
            return error("memory_write", err);
        }

        let document = document.unwrap();
        if let Err(err) = self.0.insert_documents(document).await {
            return error("memory_write", err);
        }

        Ok(format!("memory {slug} is written"))
    }
}
