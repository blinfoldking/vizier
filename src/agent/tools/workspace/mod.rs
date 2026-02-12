use std::marker::PhantomData;

use rig::{completion::ToolDefinition, tool::Tool};
use schemars::schema_for;
use serde::{Deserialize, Serialize};

use crate::error::{VizierError, error};

pub trait PrimaryDocument {
    const NAME: &'static str;
    const READ_NAME: &'static str;
    const WRITE_NAME: &'static str;
}

pub struct AgentDocument;

impl PrimaryDocument for AgentDocument {
    const NAME: &'static str = "AGENT.md";
    const READ_NAME: &'static str = "READ_AGENT_MD_FILE";
    const WRITE_NAME: &'static str = "WRITE_AGENT_MD_FILE";
}

pub struct IdentDocument;

impl PrimaryDocument for IdentDocument {
    const NAME: &'static str = "IDENT.md";
    const READ_NAME: &'static str = "READ_IDENT_MD_FILE";
    const WRITE_NAME: &'static str = "WRITE_IDENT_MD_FILE";
}

pub struct UserDocument;

impl PrimaryDocument for UserDocument {
    const NAME: &'static str = "USER.md";
    const READ_NAME: &'static str = "READ_USER_MD_FILE";
    const WRITE_NAME: &'static str = "WRITE_USER_MD_FILE";
}

pub struct ContextDocument;

impl PrimaryDocument for ContextDocument {
    const NAME: &'static str = "CONTEXT.md";
    const READ_NAME: &'static str = "READ_CONTEXT_MD_FILE";
    const WRITE_NAME: &'static str = "WRITE_CONTEXT_MD_FILE";
}

pub struct ReadPrimaryDocument<T: PrimaryDocument> {
    workspace: String,
    _phantom_data: PhantomData<T>,
}

impl<T: PrimaryDocument> ReadPrimaryDocument<T> {
    pub fn new(workspace: String) -> Self {
        Self {
            workspace,
            _phantom_data: PhantomData,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ReadPrimaryDocumentArgs {
    #[schemars(description = "the filename")]
    filename: String,
}

impl<T: PrimaryDocument> Tool for ReadPrimaryDocument<T>
where
    Self: Sync + Send,
{
    const NAME: &'static str = T::READ_NAME;
    type Error = VizierError;
    type Args = ReadPrimaryDocumentArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let parameters = serde_json::to_value(schema_for!(Self::Args)).unwrap();

        ToolDefinition {
            name: self.name(),
            description: format!("read {} file", T::NAME),
            parameters,
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        log::info!("read {}/{}", self.workspace, T::NAME);

        let path = std::path::PathBuf::from(format!("{}/{}", self.workspace, T::NAME));
        match std::fs::read_to_string(path) {
            Ok(s) => Ok(s),
            Err(err) => error("read file", err),
        }
    }
}

pub struct WritePrimaryDocument<T: PrimaryDocument> {
    _phantom_data: PhantomData<T>,
    workspace: String,
}

impl<T: PrimaryDocument> WritePrimaryDocument<T> {
    pub fn new(workspace: String) -> Self {
        Self {
            _phantom_data: PhantomData,
            workspace,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct WritePrimaryDocumentArgs {
    #[schemars(description = "New content of the file")]
    content: String,
}

impl<T: PrimaryDocument> Tool for WritePrimaryDocument<T>
where
    Self: Sync + Send,
{
    const NAME: &'static str = T::WRITE_NAME;
    type Error = VizierError;
    type Args = WritePrimaryDocumentArgs;
    type Output = ();

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let parameters = serde_json::to_value(schema_for!(Self::Args)).unwrap();

        ToolDefinition {
            name: Self::NAME.to_string(),
            description: format!("write over the content {} file, **not append**", T::NAME),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        log::info!("write {}", T::NAME);

        let path = std::path::PathBuf::from(format!("{}/{}", self.workspace, T::NAME));

        match std::fs::write(path, args.content) {
            Ok(_) => Ok(()),
            Err(err) => error("write file", err),
        }
    }
}
