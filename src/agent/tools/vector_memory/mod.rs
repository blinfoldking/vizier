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
    vector_store::{InsertDocuments, VectorStoreIndex},
};
use rig_postgres::PostgresVectorStore;
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use slugify::slugify;
use sqlx::postgres::PgPoolOptions;

use crate::config::VectorMemoryConfig;
use crate::error::{VizierError, error};

// TODO: handle openai embedder
pub async fn init_vector_memory(config: VectorMemoryConfig) -> Result<(MemoryRead, MemoryWrite)> {
    let embedder = match &*config.model.provider {
        _ => {
            let client: ollama::Client = ollama::Client::builder()
                .base_url(config.model.base_url)
                .api_key(Nothing)
                .build()
                .unwrap();

            client.embedding_model_with_ndims(config.model.name, 384)
        }
    };

    let pool = PgPoolOptions::new().connect(&config.pg_connection).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let vector_store = Arc::new(PostgresVectorStore::with_defaults(embedder.clone(), pool));

    Ok((
        MemoryRead::new(vector_store.clone()),
        MemoryWrite::new(vector_store.clone(), embedder),
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

pub type MemoryRead = ReadVectorMemory;
pub struct ReadVectorMemory(Arc<PostgresVectorStore<ollama::EmbeddingModel>>);

impl MemoryRead {
    fn new(store: Arc<PostgresVectorStore<ollama::EmbeddingModel>>) -> Self {
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
    Arc<PostgresVectorStore<ollama::EmbeddingModel>>,
    ollama::EmbeddingModel,
);

impl MemoryWrite {
    fn new(
        store: Arc<PostgresVectorStore<ollama::EmbeddingModel>>,
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
        if let Err(err) = self.0.insert_documents(document).await {
            return error("memory_write", err);
        }

        Ok(())
    }
}
