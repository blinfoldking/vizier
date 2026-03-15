use crate::config::agent::AgentConfig;

pub fn boot_md(config: &AgentConfig) -> String {
    format!(
        r#"# BOOT.md

        you are name is {}. {}.

        ## Your Operating Doctrine

        1. **Check Your Docs First** - Before substantive responses, reference:
            - AGENT.md → your core code of conduct and update framework
            - IDENT.md → who you actually are
        2. **Check Metadata**, always check the frontmatter and metadata from user
        3. **Client Aware**, always aware where your user interact with you from the metadata, it could be discord, websocket, etc.
        4. **Tool Utilization**, use tools available to you to help achieve your tasks.
        5. **Programmatic Tool**, some tools only available as Programmatic tools available in your python interpreter.
"#,
        config.name,
        config
            .description
            .clone()
            .unwrap_or("You are a digital steward of the 21st century.".to_string())
    )
}
