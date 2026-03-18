use std::sync::Arc;

use mlua::{Lua, Table, Value as LuaValue};
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use tokio::runtime::Handle;

use crate::error::VizierError;

#[async_trait::async_trait]
pub trait ProgrammaticToolCall: Send + Sync {
    /// Returns the tool name for registration in Lua globals
    fn name(&self) -> &'static str;

    /// Returns a description of the tool and its arguments for LLM discovery
    fn describe(&self) -> String;

    /// Returns a description of input of the tool and its arguments for LLM discovery
    fn describe_input(&self) -> String;

    /// Returns a description of output of the tool and its arguments for LLM discovery
    fn describe_output(&self) -> String;

    /// Returns the tool definition
    async fn get_definition(&self) -> ToolDefinition;

    /// Registers this tool as a callable function in the Lua globals table
    fn register_in_lua(&self, lua: &Lua, globals: &Table) -> mlua::Result<()>;
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

    fn register_in_lua(&self, lua: &Lua, globals: &Table) -> mlua::Result<()> {
        let tool = self.clone();

        // Create a Lua function that calls the tool
        let func = lua.create_function(move |lua, args: mlua::Table| {
            let tool = tool.clone();

            // Convert Lua table to JSON value
            let json_args = lua_table_to_json(&args)?;

            // Deserialize into the tool's Args type
            let tool_args: T::Args = serde_json::from_value(json_args)
                .map_err(|e| mlua::Error::RuntimeError(format!("Invalid arguments: {}", e)))?;

            // Execute the async tool call using tokio
            let result = if let Ok(handle) = Handle::try_current() {
                // We're inside a tokio runtime, use block_in_place
                tokio::task::block_in_place(|| {
                    handle.block_on(async { tool.call(tool_args).await })
                })
            } else {
                // No runtime, create a new one
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(async { tool.call(tool_args).await })
            };

            // Convert result back to Lua
            match result {
                Ok(output) => {
                    let json_output = serde_json::to_value(&output)
                        .map_err(|e| mlua::Error::RuntimeError(format!("Failed to serialize output: {}", e)))?;
                    json_to_lua_value(lua, &json_output)
                }
                Err(e) => Err(mlua::Error::RuntimeError(format!(
                    "Tool execution failed: {:?}",
                    e
                ))),
            }
        })?;

        globals.set(T::NAME, func)?;
        Ok(())
    }
}

/// Convert a Lua table to a JSON value
fn lua_table_to_json(table: &mlua::Table) -> mlua::Result<serde_json::Value> {
    let mut map = serde_json::Map::new();

    // Clone to allow iteration
    let table_clone = table.clone();
    for pair in table_clone.pairs::<LuaValue, LuaValue>() {
        let (key, value) = pair?;
        let key_str = match key {
            LuaValue::String(s) => s.to_string_lossy().to_string(),
            LuaValue::Integer(i) => i.to_string(),
            _ => continue,
        };

        let json_value = lua_value_to_json(value)?;
        map.insert(key_str, json_value);
    }

    Ok(serde_json::Value::Object(map))
}

/// Convert a Lua value to a JSON value
fn lua_value_to_json(value: LuaValue) -> mlua::Result<serde_json::Value> {
    match value {
        LuaValue::Nil => Ok(serde_json::Value::Null),
        LuaValue::Boolean(b) => Ok(serde_json::Value::Bool(b)),
        LuaValue::Integer(i) => Ok(serde_json::Value::Number(i.into())),
        LuaValue::Number(n) => {
            if let Some(num) = serde_json::Number::from_f64(n) {
                Ok(serde_json::Value::Number(num))
            } else {
                Ok(serde_json::Value::Null)
            }
        }
        LuaValue::String(s) => Ok(serde_json::Value::String(
            s.to_string_lossy().to_string(),
        )),
        LuaValue::Table(t) => {
            // Check if it's an array or object by collecting pairs first
            let mut pairs_vec = Vec::new();
            let mut is_array = true;
            let mut max_index = 0usize;

            let t_clone = t.clone();
            for pair in t_clone.pairs::<LuaValue, LuaValue>() {
                let pair_data = pair?;
                let (key, _) = &pair_data;
                match key {
                    LuaValue::Integer(i) if *i > 0 => {
                        max_index = max_index.max(*i as usize);
                    }
                    LuaValue::Integer(_) => {
                        is_array = false;
                    }
                    _ => {
                        is_array = false;
                    }
                }
                pairs_vec.push(pair_data);
            }

            if is_array && max_index > 0 {
                let mut arr = Vec::new();
                for i in 1..=max_index {
                    if let Ok(Some(val)) = t.get::<_, Option<LuaValue>>(i as i64) {
                        arr.push(lua_value_to_json(val)?);
                    } else {
                        arr.push(serde_json::Value::Null);
                    }
                }
                Ok(serde_json::Value::Array(arr))
            } else {
                let mut map = serde_json::Map::new();
                for (key, value) in pairs_vec {
                    let key_str = match key {
                        LuaValue::String(s) => s.to_string_lossy().to_string(),
                        LuaValue::Integer(i) => i.to_string(),
                        _ => continue,
                    };
                    map.insert(key_str, lua_value_to_json(value)?);
                }
                Ok(serde_json::Value::Object(map))
            }
        }
        _ => Err(mlua::Error::RuntimeError(
            "Unsupported Lua type for JSON conversion".to_string(),
        )),
    }
}

/// Convert a JSON value back to a Lua value
pub fn json_to_lua_value<'lua>(lua: &'lua Lua, value: &serde_json::Value) -> mlua::Result<mlua::Value<'lua>> {
    match value {
        serde_json::Value::Null => Ok(LuaValue::Nil),
        serde_json::Value::Bool(b) => Ok(LuaValue::Boolean(*b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(LuaValue::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(LuaValue::Number(f))
            } else {
                Ok(LuaValue::Nil)
            }
        }
        serde_json::Value::String(s) => Ok(LuaValue::String(lua.create_string(s)?)),
        serde_json::Value::Array(arr) => {
            let table = lua.create_table()?;
            for (i, val) in arr.iter().enumerate() {
                table.set(i + 1, json_to_lua_value(lua, val)?)?;
            }
            Ok(LuaValue::Table(table))
        }
        serde_json::Value::Object(obj) => {
            let table = lua.create_table()?;
            for (key, val) in obj.iter() {
                table.set(key.clone(), json_to_lua_value(lua, val)?)?;
            }
            Ok(LuaValue::Table(table))
        }
    }
}
