use std::sync::Arc;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use slugify::slugify;

use crate::agents::tools::VizierTool;
use crate::dependencies::VizierDependencies;
use crate::error::VizierError;
use crate::schema::{AgentId, SharedDocumentSummary};
use crate::storage::VizierStorage;
use crate::storage::shared_document::SharedDocumentStorage;

pub fn init_shared_document_tools(
    agent_id: String,
    deps: VizierDependencies,
) -> Result<(
    SharedDocumentRead,
    SharedDocumentWrite,
    SharedDocumentGet,
    SharedDocumentList,
)> {
    Ok((
        SharedDocumentRead::new(agent_id.clone(), deps.storage.clone()),
        SharedDocumentWrite::new(agent_id.clone(), deps.storage.clone()),
        SharedDocumentGet::new(agent_id.clone(), deps.storage.clone()),
        SharedDocumentList::new(agent_id.clone(), deps.storage.clone()),
    ))
}

pub type SharedDocumentRead = ReadSharedDocument;
pub struct ReadSharedDocument(AgentId, Arc<VizierStorage>);

impl ReadSharedDocument {
    fn new(agent_id: AgentId, store: Arc<VizierStorage>) -> Self {
        Self(agent_id, store)
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct SharedDocumentReadArgs {
    #[schemars(description = "Terms, keywords, or prompt to search")]
    pub query: String,
}

#[async_trait::async_trait]
impl VizierTool for SharedDocumentRead {
    type Input = SharedDocumentReadArgs;
    type Output = Vec<String>;

    fn name() -> String {
        "shared_document_read".to_string()
    }

    fn description(&self) -> String {
        "Search shared documents from all agents for information".into()
    }

    async fn call(&self, args: Self::Input) -> Result<Self::Output, VizierError> {
        let res = self
            .1
            .query_shared_documents(args.query, 10, 0.1)
            .await
            .map_err(|err| VizierError(err.to_string()))?;

        Ok(res.iter().map(|doc| doc.content.clone()).collect())
    }
}

pub type SharedDocumentWrite = WriteSharedDocument;
pub struct WriteSharedDocument(AgentId, Arc<VizierStorage>);

impl WriteSharedDocument {
    fn new(agent_id: AgentId, store: Arc<VizierStorage>) -> Self {
        Self(agent_id, store)
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema, Clone)]
pub struct SharedDocumentWriteArgs {
    #[schemars(description = "title of the document")]
    pub title: String,

    #[schemars(description = "content of the document")]
    pub content: String,

    #[schemars(description = "optional custom slug (auto-generated from title if not provided)")]
    pub slug: Option<String>,
}

#[async_trait::async_trait]
impl VizierTool for SharedDocumentWrite {
    type Input = SharedDocumentWriteArgs;
    type Output = String;

    fn name() -> String {
        "shared_document_write".to_string()
    }

    fn description(&self) -> String {
        "Write or update a shared document for all agents to see".into()
    }

    async fn call(&self, args: Self::Input) -> Result<Self::Output, VizierError> {
        let slug = args.slug.clone().unwrap_or_else(|| slugify!(&args.title));

        let _ = self
            .1
            .write_shared_document(
                self.0.clone(),
                args.slug,
                args.title.clone(),
                args.content.clone(),
            )
            .await
            .map_err(|err| VizierError(err.to_string()))?;

        Ok(format!("shared_document/{slug} created/updated"))
    }
}

pub type SharedDocumentGet = GetSharedDocument;
pub struct GetSharedDocument(AgentId, Arc<VizierStorage>);

impl GetSharedDocument {
    fn new(agent_id: AgentId, store: Arc<VizierStorage>) -> Self {
        Self(agent_id, store)
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct SharedDocumentGetArgs {
    #[schemars(description = "slug of the document to retrieve")]
    pub slug: String,
}

#[async_trait::async_trait]
impl VizierTool for SharedDocumentGet {
    type Input = SharedDocumentGetArgs;
    type Output = String;

    fn name() -> String {
        "shared_document_get".to_string()
    }

    fn description(&self) -> String {
        "Get a shared document by its slug".into()
    }

    async fn call(&self, args: Self::Input) -> Result<Self::Output, VizierError> {
        let res = self
            .1
            .get_shared_document(args.slug)
            .await
            .map_err(|err| VizierError(err.to_string()))?;

        match res {
            Some(doc) => Ok(doc.content),
            None => Err(VizierError("document not found".into())),
        }
    }
}

pub type SharedDocumentList = ListSharedDocument;
pub struct ListSharedDocument(AgentId, Arc<VizierStorage>);

impl ListSharedDocument {
    fn new(agent_id: AgentId, store: Arc<VizierStorage>) -> Self {
        Self(agent_id, store)
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct SharedDocumentListArgs {
    #[serde(default)]
    #[schemars(description = "offset for pagination")]
    pub offset: usize,

    #[serde(default = "default_limit")]
    #[schemars(description = "limit for pagination")]
    pub limit: usize,
}

fn default_limit() -> usize {
    10
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SharedDocumentListOutput {
    pub slug: String,
    pub title: String,
    pub author_agent_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl From<SharedDocumentSummary> for SharedDocumentListOutput {
    fn from(summary: SharedDocumentSummary) -> Self {
        Self {
            slug: summary.slug,
            title: summary.title,
            author_agent_id: summary.author_agent_id,
            timestamp: summary.timestamp,
        }
    }
}

#[async_trait::async_trait]
impl VizierTool for SharedDocumentList {
    type Input = SharedDocumentListArgs;
    type Output = Vec<SharedDocumentListOutput>;

    fn name() -> String {
        "shared_document_list".to_string()
    }

    fn description(&self) -> String {
        "List all shared documents with pagination".into()
    }

    async fn call(&self, args: Self::Input) -> Result<Self::Output, VizierError> {
        let res = self
            .1
            .list_shared_documents(args.offset, args.limit)
            .await
            .map_err(|err| VizierError(err.to_string()))?;

        Ok(res.into_iter().map(|s| s.into()).collect())
    }
}

