use std::collections::{BTreeSet, HashMap, HashSet};

use anyhow::Result;

use crate::{
    indexer::VizierIndexer,
    schema::{
        Memory, MemoryGraph, MemoryGraphEdge, MemoryGraphNode, MemoryQueryParams,
        MemoryVisibility, PaginatedMemory, VizierAttachment,
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

pub fn compute_initial_slugs(
    nodes: &[MemoryGraphNode],
    edges: &[MemoryGraphEdge],
    search: Option<&str>,
) -> Vec<String> {
    if let Some(q) = search {
        let q = q.trim().to_lowercase();
        if q.is_empty() {
            return compute_curated_initial(nodes, edges);
        }
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
    compute_curated_initial(nodes, edges)
}

fn compute_curated_initial(nodes: &[MemoryGraphNode], edges: &[MemoryGraphEdge]) -> Vec<String> {
    let mut degree: HashMap<&str, usize> = HashMap::new();
    for n in nodes {
        degree.insert(n.slug.as_str(), 0);
    }
    for e in edges {
        if let Some(d) = degree.get_mut(e.source.as_str()) {
            *d += 1;
        }
        if let Some(d) = degree.get_mut(e.target.as_str()) {
            *d += 1;
        }
    }

    let mut initial: Vec<String> = Vec::new();
    let mut seen: HashSet<&str> = HashSet::new();

    for n in nodes {
        if degree.get(n.slug.as_str()).copied().unwrap_or(0) == 0 {
            initial.push(n.slug.clone());
            seen.insert(n.slug.as_str());
        }
    }

    let mut tag_set: BTreeSet<&str> = BTreeSet::new();
    for n in nodes {
        for t in &n.tags {
            tag_set.insert(t.as_str());
        }
    }

    for tag in tag_set {
        let mut candidates: Vec<&MemoryGraphNode> = nodes
            .iter()
            .filter(|n| n.tags.iter().any(|t| t == tag))
            .collect();
        candidates.sort_by(|a, b| {
            let da = degree.get(a.slug.as_str()).copied().unwrap_or(0);
            let db = degree.get(b.slug.as_str()).copied().unwrap_or(0);
            db.cmp(&da).then_with(|| a.slug.cmp(&b.slug))
        });
        let mut picked = 0;
        for n in candidates {
            if picked == 5 {
                break;
            }
            if seen.contains(n.slug.as_str()) {
                continue;
            }
            initial.push(n.slug.clone());
            seen.insert(n.slug.as_str());
            picked += 1;
        }
    }

    initial
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

    fn edge(source: &str, target: &str) -> MemoryGraphEdge {
        MemoryGraphEdge {
            source: source.to_string(),
            target: target.to_string(),
            broken: false,
        }
    }

    #[test]
    fn search_returns_only_matches_case_insensitive() {
        let nodes = vec![node("kubernetes-basics", &["devops"]), node("intro", &["misc"])];
        let initial = compute_initial_slugs(&nodes, &[], Some("KUBE"));
        assert_eq!(initial, vec!["kubernetes-basics".to_string()]);
    }

    #[test]
    fn empty_search_falls_back_to_curated() {
        let nodes = vec![node("a", &["x"]), node("b", &["x"])];
        let initial = compute_initial_slugs(&nodes, &[], Some(""));
        assert!(!initial.is_empty());
    }

    #[test]
    fn isolated_nodes_are_always_included() {
        let nodes = vec![node("alone", &[]), node("hub", &["t"])];
        let edges = vec![edge("hub", "other")];
        let initial = compute_initial_slugs(&nodes, &edges, None);
        assert!(initial.contains(&"alone".to_string()));
    }

    #[test]
    fn top_five_per_tag_with_global_dedup() {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for i in 0..10 {
            let slug = format!("t-{i}");
            nodes.push(node(&slug, &["t"]));
            edges.push(edge("hub", &slug));
        }
        nodes.push(node("hub", &["t"]));
        for i in 0..3 {
            let slug = format!("s-{i}");
            nodes.push(node(&slug, &["s"]));
            edges.push(edge("hub", &slug));
        }
        let initial = compute_initial_slugs(&nodes, &edges, None);
        let seen: std::collections::HashSet<&str> = initial.iter().map(|s| s.as_str()).collect();
        assert_eq!(seen.len(), initial.len(), "no duplicate slugs");
        assert!(seen.contains("hub"));
        assert!(seen.contains("s-0"));
        assert!(seen.contains("s-1"));
        assert!(seen.contains("s-2"));
        let t_count = initial.iter().filter(|s| s.starts_with("t-")).count();
        assert!(t_count <= 5, "no more than 5 picks from tag t, got {t_count}");
    }

    #[test]
    fn deterministic_tiebreak_by_slug() {
        let nodes = vec![
            node("zzz", &["t"]),
            node("aaa", &["t"]),
            node("mmm", &["t"]),
            node("hub", &["t"]),
        ];
        let edges = vec![
            edge("hub", "aaa"),
            edge("hub", "mmm"),
            edge("hub", "zzz"),
        ];
        let initial = compute_initial_slugs(&nodes, &edges, None);
        assert_eq!(
            initial,
            vec!["hub".to_string(), "aaa".to_string(), "mmm".to_string(), "zzz".to_string()]
        );
    }
}
