use futures::TryFutureExt;
use rig::{completion::ToolDefinition, tool::Tool};
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    error::VizierError,
    shell::{ShellProvider, VizierShell},
};

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ShellExecArgs {
    #[schemars(description = "shell command to execute")]
    pub commands: String,
}

pub struct ShellExec(pub Arc<VizierShell>);

impl Tool for ShellExec {
    const NAME: &'static str = "shell_exec";
    type Error = VizierError;
    type Args = ShellExecArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let parameters = serde_json::to_value(schema_for!(Self::Args)).unwrap();

        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "run a a CLI command on a workspace directory".into(),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(self
            .0
            .exec(args.commands)
            .map_err(|err| VizierError(err.to_string()))
            .await?)
    }
}
