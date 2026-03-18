use std::{fs, path::PathBuf, sync::Arc};

use mlua::Lua;
use rig::{completion::ToolDefinition, tool::Tool};
use schemars::schema_for;
use serde::{Deserialize, Serialize};

use crate::error::VizierError;

mod docs;
mod ptc;

pub use ptc::ProgrammaticToolCall;

pub struct LuaInterpreter {
    workdir: String,
    allowed_modules: Vec<String>,
    programmatic_tools: Vec<Box<dyn ProgrammaticToolCall>>,
}

impl LuaInterpreter {
    pub fn new(workdir: String) -> Self {
        fs::create_dir_all(PathBuf::from(workdir.clone())).unwrap();

        // Lua 5.4 standard libraries that are safe to expose
        let allowed_modules = vec![
            "math".to_string(),
            "string".to_string(),
            "table".to_string(),
            "utf8".to_string(),
            "math".to_string(),
            "io".to_string(),
            "os".to_string(),
        ];

        Self {
            workdir,
            allowed_modules,
            programmatic_tools: Vec::new(),
        }
    }

    /// Register a tool to be callable from Lua scripts (builder pattern)
    pub fn with_tool(mut self, tool: Box<dyn ProgrammaticToolCall>) -> Self {
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
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct LuaInterpreterArgs {
    #[schemars(description = "Lua script to run")]
    pub script: String,
}

impl Tool for LuaInterpreter
where
    Self: Send + Sync,
{
    const NAME: &'static str = "lua_interpreter";
    type Error = VizierError;
    type Args = LuaInterpreterArgs;
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
                "\n\nAvailable tools (callable as functions, with kwargs, docs available in 'lua_tools_docs'): {}",
                tool_descriptions.join(",")
            )
        };

        let description = format!(
            "Run a Lua script in a sandboxed environment.\n\n\
            **only use this tool for calculation and accessing tools**\n\n\
            Allowed standard libraries: {}{}",
            self.allowed_modules.join(", "),
            tools_desc
        );

        ToolDefinition {
            name: Self::NAME.to_string(),
            description,
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        log::info!("lua_interpreter {}", args.script.clone());

        // Create a new Lua instance with limited standard library
        let lua = Lua::new();

        // Sandbox: only allow specific libraries
        let globals = lua.globals();

        // Set up safe standard libraries
        lua.load_from_std_lib(
            mlua::StdLib::TABLE
                | mlua::StdLib::STRING
                | mlua::StdLib::MATH
                | mlua::StdLib::UTF8
                | mlua::StdLib::OS,
        )
        .map_err(|e| VizierError(format!("Failed to load Lua libraries: {}", e)))?;

        // Set working directory in Lua global
        globals
            .set("_WORKDIR", self.workdir.clone())
            .map_err(|e| VizierError(format!("Failed to set workdir: {}", e)))?;

        // Register programmatic tools in the Lua environment
        for tool in &self.programmatic_tools {
            tool.register_in_lua(&lua, &globals)
                .map_err(|e| VizierError(format!("Failed to register tool: {}", e)))?;
        }

        // Execute the Lua script and capture output
        let result: String = lua
            .load(&args.script)
            .eval::<String>()
            .or_else(|_| {
                // If the script doesn't return a value, execute it and return empty
                lua.load(&args.script)
                    .exec()
                    .map(|_| String::new())
                    .map_err(|e| VizierError(format!("Lua execution error: {}", e)))
            })
            .map_err(|e| VizierError(format!("Lua interpreter error: {}", e)))?;

        log::info!("{result}");
        Ok(result)
    }
}
