use std::{collections::HashMap, fmt::Display};

use rig::{completion::ToolDefinition, tool::Tool};
use schemars::schema_for;
use serde::{Deserialize, Serialize};

use crate::{agent::tools::rhai::RhaiInterpreter, error::VizierError};

pub struct ToolDoc {
    name: String,
    description: String,
    input_schema: String,
    output_schema: String,
}

impl Display for ToolDoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"#{}
            {}

            ## Input Schema
            {}

            ## Output Schema
            {}

            "#,
            self.name, self.description, self.input_schema, self.output_schema
        )
    }
}

pub struct RhaiToolsDocs {
    docs: HashMap<String, ToolDoc>,
}

impl RhaiInterpreter {
    pub async fn generate_docs_tool(&self) -> RhaiToolsDocs {
        let mut docs = HashMap::new();

        for tool in self.programmatic_tools.iter() {
            let definition = tool.get_definition().await;

            let doc = ToolDoc {
                name: definition.name.clone(),
                description: definition.description,
                input_schema: tool.describe_input(),
                output_schema: tool.describe_output(),
            };

            docs.insert(definition.name, doc);
        }

        RhaiToolsDocs { docs }
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct RhaiToolsDocsArgs {
    #[schemars(description = "Optional, if filled will only show documentation for the tool")]
    tool_name: Option<String>,
}

impl RhaiToolsDocs {
    fn description(&self) -> String {
        let tool_list = self
            .docs
            .iter()
            .map(|t| t.0.clone())
            .collect::<Vec<_>>()
            .join(", ");

        let example = String::from("some_tool(#{arg: some_val})");

        format!(
            "Use this tool to get documentation detail of available programmatic tool.\n\
            Calls the underlying programmatic tool with given map/object parameter in rhai_interpreter (e.g., '{}').\n\
            List of available tools: {}",
            example, tool_list
        )
    }
}

impl Tool for RhaiToolsDocs {
    const NAME: &'static str = "rhai_tools_docs";

    type Error = VizierError;
    type Args = RhaiToolsDocsArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let parameters = serde_json::to_value(schema_for!(Self::Args)).unwrap();

        ToolDefinition {
            name: Self::NAME.to_string(),
            description: self.description(),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(match args.tool_name {
            None => {
                let docs = self
                    .docs
                    .iter()
                    .map(|(_, doc)| doc.to_string())
                    .collect::<Vec<_>>()
                    .join("\n\n");

                docs
            }
            Some(tool) => {
                let doc = self.docs.get(&tool).map(|tool| tool.to_string());

                if doc.is_none() {
                    "".into()
                } else {
                    doc.unwrap()
                }
            }
        })
    }
}
