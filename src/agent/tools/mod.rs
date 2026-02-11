use rig::tool::server::{ToolServer, ToolServerHandle};

use crate::{
    agent::tools::{
        brave_search::{BraveSearch, NewsOnlySearch, WebOnlySearch},
        workspace::{
            AgentDocument, IdentDocument, ReadPrimaryDocument, UserDocument,
            WritePrimaryDocument,
        },
    },
    config::ToolsConfig,
};

mod brave_search;
mod workspace;

#[derive(Clone)]
pub struct VizierTools {
    pub handle: ToolServerHandle,
    pub turn_depth: u32,
    pub workspace: String,
}

impl VizierTools {
    pub fn new(workspace: String, config: ToolsConfig) -> Self {
        let mut tool_server_builder = ToolServer::new()
            .tool(ReadPrimaryDocument::<AgentDocument>::new(
                workspace.clone(),
            ))
            .tool(ReadPrimaryDocument::<IdentDocument>::new(
                workspace.clone(),
            ))
            .tool(ReadPrimaryDocument::<UserDocument>::new(workspace.clone()))
            .tool(WritePrimaryDocument::<AgentDocument>::new(
                workspace.clone(),
            ))
            .tool(WritePrimaryDocument::<IdentDocument>::new(
                workspace.clone(),
            ))
            .tool(WritePrimaryDocument::<UserDocument>::new(
                workspace.clone(),
            ));

        if let Some(brave_search) = config.brave_search {
            tool_server_builder =
                tool_server_builder.tool(BraveSearch::<WebOnlySearch>::new(&brave_search));
            // .tool(BraveSearch::<NewsOnlySearch>::new(&brave_search));
        }

        let tool_server = tool_server_builder.run();

        Self {
            workspace,
            turn_depth: config.turn_depth,
            handle: tool_server,
        }
    }
}
