use tracing::info;
use topkio_core::provider::Provider;

use crate::openai::OpenAIProvider;
use crate::gemini::GeminiProvider;
use crate::ollama::OllamaProvider;
use crate::deepseek::DeepSeekProvider;

// 提供商类型枚举，包含模型名称
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderKind {
    OpenAI(String),    // 模型名称，例如 "gpt-4"
    Gemini(String),    // 模型名称，例如 "gemini-pro"
    Ollama(String),    // 模型名称，例如 "llama3"
    DeepSeek(String),  // 模型名称，例如 "deepseek-rag"
}

impl ProviderKind {
    // 从配置键和模型名称创建 ProviderKind
    pub fn from_key_and_model(key: &str, model: &str) -> Option<Self> {
        match key {
            "openai" => Some(ProviderKind::OpenAI(model.to_string())),
            "gemini" => Some(ProviderKind::Gemini(model.to_string())),
            "ollama" => Some(ProviderKind::Ollama(model.to_string())),
            "deepseek" => Some(ProviderKind::DeepSeek(model.to_string())),
            _ => None,
        }
    }

    // 根据模型名称匹配 ProviderKind
    pub fn matches_model(&self, model: &str) -> bool {
        match self {
            ProviderKind::OpenAI(m) => model.starts_with("gpt") && m == model,
            ProviderKind::Gemini(m) => model.starts_with("gemini") && m == model,
            ProviderKind::Ollama(m) => (model.contains("llama") || model.contains("mistral")) && m == model,
            ProviderKind::DeepSeek(m) => model.contains("deepseek") && m == model,
        }
    }
}

pub fn create_providers(config: topkio_core::config::ProvidersConfig) -> Vec<(ProviderKind, Box<dyn Provider>)> {
    let mut providers = Vec::new();

    // 遍历配置中的所有提供商
    for (key, cfg) in config.providers {
        match key.as_str() {
            "openai" => {
                // 假设配置中可能有模型名称，实际可扩展
                let model = "gpt-4"; // 示例模型，实际应从配置或请求中获取
                providers.push((
                    ProviderKind::OpenAI(model.to_string()),
                    Box::new(OpenAIProvider::new(cfg.clone())) as Box<dyn Provider>,
                ));
                info!("OpenAI provider initialized with model: {}", model);
            }
            "gemini" => {
                let model = "gemini-pro";
                providers.push((
                    ProviderKind::Gemini(model.to_string()),
                    Box::new(GeminiProvider::new(cfg.clone())),
                ));
                info!("Gemini provider initialized with model: {}", model);
            }
            "ollama" => {
                let model = "llama3";
                providers.push((
                    ProviderKind::Ollama(model.to_string()),
                    Box::new(OllamaProvider::new(cfg.clone())),
                ));
                info!("Ollama provider initialized with model: {}", model);
            }
            "deepseek" => {
                let model = "deepseek-rag";
                providers.push((
                    ProviderKind::DeepSeek(model.to_string()),
                    Box::new(DeepSeekProvider::new(cfg.clone())),
                ));
                info!("DeepSeek provider initialized with model: {}", model);
            }
            _ => {
                info!("Unknown provider '{}' ignored", key);
            }
        }
    }

    providers
}

pub fn select_provider(
    providers: &[(ProviderKind, Box<dyn Provider>)],
    model: &str,
) -> Option<Box<dyn Provider>> {
    providers
        .iter()
        .find(|(kind, _)| kind.matches_model(model))
        .map(|(_, provider)| provider)
}