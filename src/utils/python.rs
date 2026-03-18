use anyhow::Result;

/// Check if Lua is available (Lua is embedded via mlua, so this is always true)
/// This function is kept for compatibility with existing code.
pub fn check_python_version() -> Result<()> {
    log::info!("Lua interpreter available (embedded via mlua)");
    Ok(())
}
