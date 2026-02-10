use rig::tool::server::{ToolServer, ToolServerHandle};

use crate::{
    agent::tools::brave_search::{BraveSearch, NewsOnlySearch, WebOnlySearch},
    config::ToolsConfig,
};

mod brave_search;

#[derive(Clone)]
pub struct VizierTools {
    pub handle: ToolServerHandle,
    pub turn_depth: u32,
}

impl VizierTools {
    pub fn new(config: ToolsConfig) -> Self {
        let mut tool_server_builder = ToolServer::new();
        if let Some(brave_search) = config.brave_search {
            tool_server_builder = tool_server_builder
                .tool(BraveSearch::<WebOnlySearch>::new(&brave_search))
                .tool(BraveSearch::<NewsOnlySearch>::new(&brave_search));
        }

        let tool_server = tool_server_builder.run();

        Self {
            turn_depth: config.turn_depth,
            handle: tool_server,
        }
    }
}
