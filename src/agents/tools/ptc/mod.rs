use std::sync::{Arc, Mutex};

use mlua::{Lua, LuaSerdeExt};
use serde::{Deserialize, Serialize};
use tokio::runtime::Handle;

use crate::{
    agents::tools::{VizierTool, VizierTools},
    error::VizierError,
};

pub struct ProgramaticSandbox {
    pub tools: Arc<VizierTools>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ProgramaticSandboxArgs {
    #[schemars(description = "script to run")]
    pub script: String,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ProgramaticSandboxOutput {
    #[schemars(description = "console_output")]
    pub console_outputs: String,

    #[schemars(description = "return value if any")]
    pub return_value: serde_json::Value,
}

#[async_trait::async_trait]
impl VizierTool for ProgramaticSandbox {
    type Input = ProgramaticSandboxArgs;
    type Output = ProgramaticSandboxOutput;

    fn name() -> String {
        "programmatic_sandbox".to_string()
    }

    fn description(&self) -> String {
        r#"run a lua script in a sandbox.
can used to call multiple tools at once by using tool_call(function_name, args). 
ie: tool_call("web_search", { query = "some query", page = 1 })

and you can use print to see the result of the tool call
"#
        .into()
    }

    async fn call(&self, args: Self::Input) -> Result<Self::Output, VizierError> {
        let lua = Lua::new();
        let globals = lua.globals();

        let _ = lua.load_std_libs(
            mlua::StdLib::TABLE
                | mlua::StdLib::STRING
                | mlua::StdLib::MATH
                | mlua::StdLib::UTF8
                | mlua::StdLib::OS,
        );

        let tools = self.tools.clone();
        let tool_call = lua
            .create_function(
                move |lua: &Lua, (function_name, args): (String, mlua::Value)| {
                    let params = serde_json::to_string(&args)
                        .map_err(|err| mlua::Error::runtime(err.to_string()))?;

                    // Execute the async tool call using tokio
                    let result = if let Ok(handle) = Handle::try_current() {
                        // We're inside a tokio runtime, use block_in_place
                        tokio::task::block_in_place(|| {
                            handle.block_on(async { tools.call(function_name, params).await })
                        })
                    } else {
                        // No runtime, create a new one
                        tokio::runtime::Runtime::new()
                            .unwrap()
                            .block_on(async { tools.call(function_name, params).await })
                    }
                    .map_err(|err| mlua::Error::runtime(err.to_string()))?;

                    let value = serde_json::from_str::<serde_json::Value>(&result)
                        .map_err(|err| mlua::Error::runtime(err.to_string()))?;

                    let ret = lua
                        .to_value(&value)
                        .map_err(|err| mlua::Error::runtime(err.to_string()))?;

                    Ok(ret)
                },
            )
            .map_err(|e| VizierError(e.to_string()))?;

        globals
            .set("tool_call", tool_call)
            .map_err(|e| VizierError(e.to_string()))?;

        let print_buffer = Arc::new(Mutex::new(Vec::<String>::new()));
        let print_buffer_clone = print_buffer.clone();
        let print_fn = lua
            .create_function(move |_: &Lua, args: mlua::Value| {
                let line = serde_json::to_string(&args).unwrap();
                print_buffer_clone.lock().unwrap().push(line);
                Ok(())
            })
            .map_err(|e| VizierError(e.to_string()))?;

        globals
            .set("print", print_fn)
            .map_err(|e| VizierError(e.to_string()))?;

        let lua_val = lua
            .load(&args.script)
            .eval::<mlua::Value>()
            .map_err(|e| VizierError(e.to_string()))?;

        let return_value =
            serde_json::to_value(&lua_val).map_err(|e| VizierError(e.to_string()))?;
        let console_outputs = print_buffer.clone().lock().unwrap().join("\n");

        Ok(ProgramaticSandboxOutput {
            console_outputs,
            return_value,
        })
    }
}

