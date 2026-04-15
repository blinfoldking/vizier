use anyhow::Result;

use crate::{schema::{SharedDocument, SharedDocumentSummary}, storage::VizierStorage};

#[async_trait::async_trait]
pub trait SharedDocumentStorage {
    async fn write_shared_document(
        &self,
        author_agent_id: String,
        slug: Option<String>,
        title: String,
        content: String,
    ) -> Result<()>;

    async fn query_shared_documents(
        &self,
        query: String,
        limit: usize,
        threshold: f64,
    ) -> Result<Vec<SharedDocument>>;

    async fn get_shared_document(&self, slug: String) -> Result<Option<SharedDocument>>;

    async fn list_shared_documents(
        &self,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<SharedDocumentSummary>>;

    async fn delete_shared_document(
        &self,
        author_agent_id: String,
        slug: String,
    ) -> Result<()>;
}

#[async_trait::async_trait]
impl SharedDocumentStorage for VizierStorage {
    async fn write_shared_document(
        &self,
        author_agent_id: String,
        slug: Option<String>,
        title: String,
        content: String,
    ) -> Result<()> {
        self.0.write_shared_document(author_agent_id, slug, title, content).await
    }

    async fn query_shared_documents(
        &self,
        query: String,
        limit: usize,
        threshold: f64,
    ) -> Result<Vec<SharedDocument>> {
        self.0.query_shared_documents(query, limit, threshold).await
    }

    async fn get_shared_document(&self, slug: String) -> Result<Option<SharedDocument>> {
        self.0.get_shared_document(slug).await
    }

    async fn list_shared_documents(
        &self,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<SharedDocumentSummary>> {
        self.0.list_shared_documents(offset, limit).await
    }

    async fn delete_shared_document(
        &self,
        author_agent_id: String,
        slug: String,
    ) -> Result<()> {
        self.0.delete_shared_document(author_agent_id, slug).await
    }
}