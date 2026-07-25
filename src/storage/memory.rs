use anyhow::Result;

use crate::{
    indexer::VizierIndexer,
    schema::{
        Memory, MemoryGraph, MemoryGraphNode, MemoryQueryParams, MemoryVisibility,
        PaginatedMemory, VizierAttachment,
    },
    storage::VizierStorage,
};

#[async_trait::async_trait]
pub trait MemoryStorage {
    async fn write_memory(
        &self,
        agent_id: String,
        slug: Option<String>,
        title: String,
        content: String,
        visibility: MemoryVisibility,
        shared_to: Vec<String>,
        tags: Vec<String>,
        attachments: Vec<VizierAttachment>,
        indexer: &VizierIndexer,
    ) -> Result<Memory>;

    async fn query_memory(
        &self,
        agent_id: String,
        query: String,
        limit: usize,
        threshold: f64,
        indexer: &VizierIndexer,
    ) -> Result<Vec<Memory>>;

    async fn get_all_agent_memory(&self, agent_id: String) -> Result<Vec<Memory>>;

    async fn get_filtered_memories(
        &self,
        params: MemoryQueryParams,
    ) -> Result<PaginatedMemory>;

    async fn get_memory_detail(&self, agent_id: String, slug: String) -> Result<Option<Memory>>;

    async fn get_related_memories(
        &self,
        agent_id: String,
        slug: String,
    ) -> Result<Vec<Memory>>;

    async fn get_memory_graph(
        &self,
        agent_id: String,
        search: Option<String>,
    ) -> Result<MemoryGraph>;

    async fn has_incoming_links(&self, agent_id: String, slug: String) -> Result<bool>;

    async fn delete_memory(
        &self,
        agent_id: String,
        slug: String,
        indexer: &VizierIndexer,
    ) -> Result<()>;

    async fn increment_read_count(&self, agent_id: String, slug: String) -> Result<()>;
}

#[async_trait::async_trait]
impl MemoryStorage for VizierStorage {
    async fn write_memory(
        &self,
        agent_id: String,
        slug: Option<String>,
        title: String,
        content: String,
        visibility: MemoryVisibility,
        shared_to: Vec<String>,
        tags: Vec<String>,
        attachments: Vec<VizierAttachment>,
        indexer: &VizierIndexer,
    ) -> Result<Memory> {
        self.0
            .write_memory(agent_id, slug, title, content, visibility, shared_to, tags, attachments, indexer)
            .await
    }

    async fn query_memory(
        &self,
        agent_id: String,
        query: String,
        limit: usize,
        threshold: f64,
        indexer: &VizierIndexer,
    ) -> Result<Vec<Memory>> {
        self.0.query_memory(agent_id, query, limit, threshold, indexer).await
    }

    async fn get_all_agent_memory(&self, agent_id: String) -> Result<Vec<Memory>> {
        self.0.get_all_agent_memory(agent_id).await
    }

    async fn get_filtered_memories(
        &self,
        params: MemoryQueryParams,
    ) -> Result<PaginatedMemory> {
        self.0.get_filtered_memories(params).await
    }

    async fn get_memory_detail(&self, agent_id: String, slug: String) -> Result<Option<Memory>> {
        self.0.get_memory_detail(agent_id, slug).await
    }

    async fn get_related_memories(
        &self,
        agent_id: String,
        slug: String,
    ) -> Result<Vec<Memory>> {
        self.0.get_related_memories(agent_id, slug).await
    }

    async fn get_memory_graph(
        &self,
        agent_id: String,
        search: Option<String>,
    ) -> Result<MemoryGraph> {
        self.0.get_memory_graph(agent_id, search).await
    }

    async fn has_incoming_links(&self, agent_id: String, slug: String) -> Result<bool> {
        self.0.has_incoming_links(agent_id, slug).await
    }

    async fn delete_memory(
        &self,
        agent_id: String,
        slug: String,
        indexer: &VizierIndexer,
    ) -> Result<()> {
        self.0.delete_memory(agent_id, slug, indexer).await
    }

    async fn increment_read_count(&self, agent_id: String, slug: String) -> Result<()> {
        self.0.increment_read_count(agent_id, slug).await
    }
}

pub fn compute_initial_slugs(nodes: &[MemoryGraphNode], search: Option<&str>) -> Vec<String> {
    if let Some(q) = search {
        let q = q.trim().to_lowercase();
        if !q.is_empty() {
            return nodes
                .iter()
                .filter(|n| {
                    n.title.to_lowercase().contains(&q)
                        || n.slug.to_lowercase().contains(&q)
                        || n.tags.iter().any(|t| t.to_lowercase().contains(&q))
                })
                .map(|n| n.slug.clone())
                .collect();
        }
    }
    nodes.iter().map(|n| n.slug.clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::MemoryVisibility;

    fn node(slug: &str, tags: &[&str]) -> MemoryGraphNode {
        MemoryGraphNode {
            slug: slug.to_string(),
            title: slug.to_string(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            visibility: MemoryVisibility::Private,
            agent_id: "a".to_string(),
        }
    }

    #[test]
    fn search_returns_only_matches_case_insensitive() {
        let nodes = vec![node("kubernetes-basics", &["devops"]), node("intro", &["misc"])];
        let initial = compute_initial_slugs(&nodes, Some("KUBE"));
        assert_eq!(initial, vec!["kubernetes-basics".to_string()]);
    }

    #[test]
    fn empty_search_returns_all_nodes() {
        let nodes = vec![node("a", &["x"]), node("b", &["x"])];
        let initial = compute_initial_slugs(&nodes, Some(""));
        assert_eq!(initial, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn no_search_returns_all_nodes() {
        let nodes = vec![
            node("alone", &[]),
            node("hub", &["t"]),
            node("other", &["t"]),
        ];
        let initial = compute_initial_slugs(&nodes, None);
        assert_eq!(
            initial,
            vec!["alone".to_string(), "hub".to_string(), "other".to_string()]
        );
    }

    #[test]
    fn no_search_is_not_capped_per_tag() {
        let nodes: Vec<MemoryGraphNode> = (0..10)
            .map(|i| node(&format!("t-{i}"), &["t"]))
            .collect();
        let initial = compute_initial_slugs(&nodes, None);
        assert_eq!(initial.len(), nodes.len());
    }
}
