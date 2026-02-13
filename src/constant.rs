pub const DEFAULT_CONFIG_PATH: &str = "config/.vizier/config.toml"; // relative to $HOME
pub const DEFAULT_CONFIG_TOML: &str = include_str!("../templates/.vizier.template.toml");

pub const BOOT_MD: &str = include_str!("../templates/BOOT.md");
pub const AGENT_MD: &str = include_str!("../templates/AGENT.md");
pub const IDENT_MD: &str = include_str!("../templates/IDENT.md");
pub const USER_MD: &str = include_str!("../templates/USER.md");
