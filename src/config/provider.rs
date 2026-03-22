use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ProviderVariant {
    deepseek,
    openrouter,
    ollama,
    gemini,
    openai,
    anthropic,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProviderConfig {
    pub anthropic: Option<AnthropicProviderConfig>,
    pub openai: Option<OpenAIProviderConfig>,
    pub gemini: Option<GeminiProviderConfig>,
    pub deepseek: Option<DeepseekProviderConfig>,
    pub openrouter: Option<OpenRouterProviderConfig>,
    pub ollama: Option<OllamaProviderConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnthropicProviderConfig {
    pub api_key: String,
}

impl Default for AnthropicProviderConfig {
    fn default() -> Self {
        Self {
            api_key: "${ANTROPHIC_API_KEY}".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIProviderConfig {
    pub base_url: Option<String>,
    pub api_key: String,
}

impl Default for OpenAIProviderConfig {
    fn default() -> Self {
        Self {
            base_url: None,
            api_key: "${OPENAI_API_KEY}".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeminiProviderConfig {
    pub api_key: String,
}

impl Default for GeminiProviderConfig {
    fn default() -> Self {
        Self {
            api_key: "${GEMINI_API_KEY}".into(),
        }
    }
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
