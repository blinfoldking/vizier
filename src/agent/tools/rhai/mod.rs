use std::{fs, path::PathBuf, sync::Arc};

use rhai::{Array, Dynamic, Engine, EvalAltResult, Map, Scope};
use rig::{completion::ToolDefinition, tool::Tool};
use schemars::schema_for;
use serde::{Deserialize, Serialize};

use crate::error::VizierError;

mod docs;
mod ptc;

pub use ptc::ProgrammaticToolCall;

pub struct RhaiInterpreter {
    workdir: String,
    programmatic_tools: Vec<Box<dyn ProgrammaticToolCall + Send + Sync>>,
}

impl RhaiInterpreter {
    pub fn new(workdir: String) -> Self {
        fs::create_dir_all(PathBuf::from(workdir.clone())).unwrap();

        Self {
            workdir,
            programmatic_tools: Vec::new(),
        }
    }

    fn create_engine() -> Engine {
        let mut engine = Engine::new();

        // Enable necessary features
        engine.set_max_expr_depths(64, 32);

        engine
    }

    /// Register a tool to be callable from Rhai scripts (builder pattern)
    pub fn with_tool(mut self, tool: Box<dyn ProgrammaticToolCall + Send + Sync>) -> Self {
        self.programmatic_tools.push(tool);
        self
    }

    /// Register a tool using Arc wrapper (builder pattern)
    pub fn tool<T>(mut self, tool: T) -> Self
    where
        T: Tool<Error = VizierError> + Send + Sync + 'static,
        T::Args: for<'de> Deserialize<'de> + schemars::JsonSchema + Send,
        T::Output: Serialize + schemars::JsonSchema,
    {
        self.programmatic_tools.push(Box::new(Arc::new(tool)));
        self
    }

    /// Register programmatic tools in the engine
    fn register_tools(&self, engine: &mut Engine) {
        for tool in &self.programmatic_tools {
            tool.register_in_engine(engine);
        }
    }
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct RhaiInterpreterArgs {
    #[schemars(description = "Rhai script to run")]
    pub script: String,
}

impl Tool for RhaiInterpreter {
    const NAME: &'static str = "rhai_interpreter";
    type Error = VizierError;
    type Args = RhaiInterpreterArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let parameters = serde_json::to_value(schema_for!(Self::Args)).unwrap();

        // Build available tools description
        let tools_desc = if self.programmatic_tools.is_empty() {
            String::new()
        } else {
            let mut tool_descriptions = Vec::new();
            for tool in &self.programmatic_tools {
                tool_descriptions.push(tool.name());
            }
            format!(
                "\n\nAvailable tools (Calls the underlying programmatic tool with given map/object parameter in rhai_interpreter, ie: some_tool(#{{arg: some_val}})): {}",
                tool_descriptions.join(", ")
            )
        };

        let description = format!(
            "Run a Rhai script in a sandboxed environment.\n\n\
            **Use this tool for calculations and accessing tools**\n\
            {tools_desc}"
        );

        ToolDefinition {
            name: Self::NAME.to_string(),
            description,
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        log::info!("rhai_interpreter {}", args.script.clone());

        // Create a new engine instance with tools registered
        let mut engine = Self::create_engine();
        self.register_tools(&mut engine);

        // Execute the script
        let result: Result<String, Box<EvalAltResult>> = (|| {
            let mut scope = Scope::new();

            // Execute the script
            let result = engine.eval_with_scope::<Dynamic>(&mut scope, &args.script)?;

            // Capture output - for now return the result as string
            let output = if result.is::<()>() {
                String::new()
            } else {
                result.to_string()
            };

            Ok(output)
        })();

        println!("{:?}", result);

        match result {
            Ok(output) => Ok(output),
            Err(err) => Err(VizierError(format!("Rhai execution error: {}", err))),
        }
    }
}
