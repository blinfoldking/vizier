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
                tool_descriptions.push(format!("\n### {}\n{}", tool.name(), tool.describe()));
            }
            format!(
                "\n\n## Available PTC (Programmatic Tool Calls)\n\n\
                These tools can be called from within Rhai scripts using the function name with a map/object parameter.\n\
                Example: `web_search(#{{query: \"some query\", page: 1}})`\n\n{}",
                tool_descriptions.join("\n")
            )
        };

        let std_lib_desc = r#"Available std library functions:
- String: trim(s), trim_start(s), trim_end(s), to_lowercase(s), to_uppercase(s), replace(s, from, to), contains(s, pattern), starts_with(s, pattern), ends_with(s, pattern), split(s, delimiter), join(arr, separator), substring(s, start, end)
- Array: len(arr), push(arr, val), pop(arr), get(arr, idx), contains(arr, val), sort(arr), reverse(arr), first(arr), last(arr), is_empty(arr), clear(arr)
- Map: len(map), get(map, key), set(map, key, val), contains(map, key), keys(map), values(map), remove(map, key), is_empty(map), clear(map)
- Math: abs(n), floor(n), ceil(n), round(n), sqrt(n), pow(base, exp), max(a, b), min(a, b), sin(n), cos(n), tan(n), asin(n), acos(n), atan(n), atan2(y, x), exp(n), ln(n), log(n, base), log10(n), log2(n), signum(n), trunc(n), fract(n), cbrt(n), hypot(x, y)
- Type conversions: to_int(s), to_float(s), to_string(n), to_hex(n), to_oct(n), to_bin(n), parse_int(s, radix), type_of(val), is_number(val)
- DateTime: now(), sleep(seconds)
- Utility: print(msg), println(msg), is_empty(s), range(end), range(start, end), range(start, end, step)"#;

        let description = format!(
            "Run a Rhai script in a sandboxed environment.\n\n\
            **Use this tool for calculations and accessing tools**\n\n\
            {std_lib_desc}\n\
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
