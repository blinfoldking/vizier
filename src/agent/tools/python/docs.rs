use std::{collections::HashMap, fmt::Display};

use rig::{completion::ToolDefinition, tool::Tool};
use schemars::schema_for;
use serde::{Deserialize, Serialize};

use crate::{agent::tools::python::LuaInterpreter, error::VizierError};

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

pub struct LuaToolsDocs {
    docs: HashMap<String, ToolDoc>,
}

impl LuaInterpreter {
    pub async fn generate_docs_tool(&self) -> LuaToolsDocs {
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

        LuaToolsDocs { docs }
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct LuaToolsDocsArgs {
    #[schemars(description = "Optional, if filled will only show documentation for the tool")]
    tool_name: Option<String>,
}

impl LuaToolsDocs {
    fn description(&self) -> String {
        format!(
            r#"use this tool to get documentation detail of available programmatic tools.
            Calls the underlying programmatic tool with given kwargs in lua_interpreter (ie. `some_tool({{arg=some_val}})`)
            list of available tools: {}"#,
            self.docs
                .iter()
                .map(|t| t.0.clone())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Tool for LuaToolsDocs {
    const NAME: &'static str = "lua_tools_docs";

    type Error = VizierError;
    type Args = LuaToolsDocsArgs;
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
