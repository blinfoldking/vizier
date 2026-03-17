use std::sync::Arc;

use rhai::{Dynamic, Engine, EvalAltResult, Map};
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use tokio::runtime::Handle;

use crate::error::VizierError;

#[async_trait::async_trait]
pub trait ProgrammaticToolCall: Send + Sync {
    /// Returns the tool name for registration in Rhai engine
    fn name(&self) -> &'static str;

    /// Returns a description of the tool and its arguments for LLM discovery
    fn describe(&self) -> String;

    /// Returns a description of input of the tool and its arguments for LLM discovery
    fn describe_input(&self) -> String;

    /// Returns a description of output of the tool and its arguments for LLM discovery
    fn describe_output(&self) -> String;

    /// Returns the tool definition
    async fn get_definition(&self) -> ToolDefinition;

    /// Registers this tool as a callable function in the Rhai engine
    fn register_in_engine(&self, engine: &mut Engine);
}

#[async_trait::async_trait]
impl<T> ProgrammaticToolCall for Arc<T>
where
    T: Tool<Error = VizierError> + Send + Sync + 'static,
    T::Args: for<'de> Deserialize<'de> + schemars::JsonSchema + Send,
    T::Output: Serialize + schemars::JsonSchema,
{
    fn name(&self) -> &'static str {
        T::NAME
    }

    async fn get_definition(&self) -> ToolDefinition {
        let definition = self.definition("".into()).await;
        definition
    }

    fn describe_input(&self) -> String {
        let args_schema = schemars::schema_for!(T::Args);
        let args_schema_json = serde_json::to_string_pretty(&args_schema).unwrap_or_default();
        args_schema_json
    }

    fn describe_output(&self) -> String {
        let output_schema = schemars::schema_for!(T::Output);
        let output_schema_json = serde_json::to_string_pretty(&output_schema).unwrap_or_default();
        output_schema_json
    }

    fn describe(&self) -> String {
        let args_schema = schemars::schema_for!(T::Args);
        let args_schema_json = serde_json::to_string_pretty(&args_schema).unwrap_or_default();
        let output_schema = schemars::schema_for!(T::Output);
        let output_schema_json = serde_json::to_string_pretty(&output_schema).unwrap_or_default();
        format!(
            "Function: {}\nArguments (JSON Schema):\n{}\nOutput (JSON Schema):\n{}",
            T::NAME,
            args_schema_json,
            output_schema_json
        )
    }

    fn register_in_engine(&self, engine: &mut Engine) {
        let tool = self.clone();

        // Register a function that takes a Map (Rhai's object type) as keyword arguments
        engine.register_fn(
            T::NAME,
            move |args: Map| -> Result<Dynamic, Box<EvalAltResult>> {
                let tool = tool.clone();
                let tool_name = T::NAME;

                // Convert Rhai Map to JSON Value
                let json_args = map_to_json(&args);

                // Deserialize into the tool's Args type
                let tool_args: T::Args = match serde_json::from_value(json_args) {
                    Ok(args) => args,
                    Err(e) => {
                        return Err(format!("Invalid arguments for {}: {}", tool_name, e).into());
                    }
                };

                // Clone tool for async block
                let tool_clone = tool.clone();

                // Execute the async tool call using tokio
                let result = if let Ok(handle) = Handle::try_current() {
                    // We're inside a tokio runtime, use block_in_place
                    tokio::task::block_in_place(|| {
                        handle.block_on(async { tool_clone.call(tool_args).await })
                    })
                } else {
                    // No runtime, create a new one
                    tokio::runtime::Runtime::new()
                        .unwrap()
                        .block_on(async { tool_clone.call(tool_args).await })
                };

                // Convert result back to Rhai Dynamic
                match result {
                    Ok(output) => {
                        let json_output = match serde_json::to_value(&output) {
                            Ok(val) => val,
                            Err(e) => {
                                return Err(format!("Failed to serialize output: {}", e).into());
                            }
                        };

                        match json_to_dynamic(json_output) {
                            Ok(val) => Ok(val),
                            Err(e) => Err(format!("Failed to convert to Rhai: {}", e).into()),
                        }
                    }
                    Err(e) => Err(format!("Tool execution failed: {:?}", e).into()),
                }
            },
        );
    }
}

/// Convert Rhai Map to serde_json::Value
fn map_to_json(map: &Map) -> serde_json::Value {
    let mut json_map = serde_json::Map::new();
    for (key, value) in map.iter() {
        json_map.insert(key.to_string(), dynamic_to_json(value.clone()));
    }
    serde_json::Value::Object(json_map)
}

/// Convert Rhai Dynamic to serde_json::Value
fn dynamic_to_json(value: Dynamic) -> serde_json::Value {
    if let Ok(v) = value.clone().as_int() {
        serde_json::Value::Number(v.into())
    } else if let Ok(v) = value.clone().as_float() {
        serde_json::Value::Number(serde_json::Number::from_f64(v).unwrap_or(0.into()))
    } else if let Ok(v) = value.clone().as_bool() {
        serde_json::Value::Bool(v)
    } else if let Ok(v) = value.clone().into_string() {
        serde_json::Value::String(v)
    } else if let Some(v) = value.clone().try_cast::<rhai::Array>() {
        let arr: Vec<serde_json::Value> = v.into_iter().map(|item| dynamic_to_json(item)).collect();
        serde_json::Value::Array(arr)
    } else if let Some(v) = value.clone().try_cast::<Map>() {
        map_to_json(&v)
    } else if value.is::<()>() {
        serde_json::Value::Null
    } else {
        serde_json::Value::String(value.to_string())
    }
}

/// Convert serde_json::Value to Rhai Dynamic
fn json_to_dynamic(value: serde_json::Value) -> Result<Dynamic, Box<EvalAltResult>> {
    match value {
        serde_json::Value::Null => Ok(Dynamic::UNIT),
        serde_json::Value::Bool(b) => Ok(Dynamic::from(b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Dynamic::from(i))
            } else if let Some(f) = n.as_f64() {
                Ok(Dynamic::from(f))
            } else {
                Ok(Dynamic::from(n.to_string()))
            }
        }
        serde_json::Value::String(s) => Ok(Dynamic::from(s)),
        serde_json::Value::Array(arr) => {
            let rhai_arr: Result<Vec<Dynamic>, _> =
                arr.into_iter().map(|v| json_to_dynamic(v)).collect();
            Ok(Dynamic::from(rhai_arr?))
        }
        serde_json::Value::Object(obj) => {
            let mut map = Map::new();
            for (key, value) in obj {
                map.insert(key.into(), json_to_dynamic(value)?);
            }
            Ok(Dynamic::from(map))
        }
    }
}
