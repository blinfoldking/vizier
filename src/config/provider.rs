use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ProviderVariant {
    deepseek,
    openrouter,
    ollama,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProviderConfig {
    pub ollama: Option<OllamaProviderConfig>,
    pub deepseek: Option<DeepseekProviderConfig>,
    pub openrouter: Option<OpenRouterProviderConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OllamaProviderConfig {
    pub base_url: String,
}

impl Default for OllamaProviderConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeepseekProviderConfig {
    pub api_key: String,
}

impl Default for DeepseekProviderConfig {
    fn default() -> Self {
        Self {
            api_key: "${DEEPSEEK_API_KEY}".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenRouterProviderConfig {
    pub api_key: String,
}

impl Default for OpenRouterProviderConfig {
    fn default() -> Self {
        Self {
            api_key: "${OPENROUTER_API_KEY}".into(),
        }
    }
}
