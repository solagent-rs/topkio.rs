use std::sync::Arc;
use tracing::info;
use topkio_core::provider::Provider;
use once_cell::sync::Lazy;

// use crate::openai::OpenAIProvider;
// use crate::gemini::GeminiProvider;
use crate::ollama::OllamaProvider;
// use crate::deepseek::DeepSeekProvider;

#[non_exhaustive]
pub enum ProviderKind {
    /// Ollama model, e.g., "llama3.2".
    Ollama {
        model_name: String,
        config: ModelConfig, // 携带模型专属配置
    },
    
    // /// OpenAI model, e.g., "gpt-4", "gpt-3.5-turbo".
    // OpenAI(String),
    // /// Gemini model, e.g., "gemini-1.0".
    // Gemini(String),
    // /// Anthropic model, e.g., "claude-3".
    // Anthropic(String),
    // /// Cohere model, e.g., "cohere-1.0".
    // Cohere(String),
    // /// Perplexity model, e.g., "perplexity-1.0".
    // Perplexity(String),
}

// 全局 Provider 注册表
pub static PROVIDER_REGISTRY: Lazy<HashMap<String, Vec<ProviderKind>>> = Lazy::new(|| {
    let config = load_config().expect("Failed to load config");
    let mut registry = HashMap::new();

    for (provider_name, provider_cfg) in config.providers {
        let models = provider_cfg.models
            .into_iter()
            .map(|(model_name, model_config)| {
                match provider_name.as_str() {
                    "ollama" => ProviderKind::Ollama {
                        model_name,
                        config: model_config,
                    },
                    "openai" => ProviderKind::Openai {
                        model_name,
                        config: model_config,
                    },
                    _ => panic!("Unknown provider: {}", provider_name),
                }
            })
            .collect();

        registry.insert(provider_name, models);
    }

    registry
});

// 选择提供商
pub fn select_provider(
    providers: &[(ProviderKind, Arc<Box<dyn Provider>>)],
    model: &str,
) -> Option<Box<dyn Provider>> {
    providers
        .iter()
        .find(|(kind, _)| kind.matches_model(model))
        .map(|(_, provider)| {
            // 克隆 Arc，获取 Box<dyn Provider>
            let provider = Arc::clone(provider);
            Arc::try_unwrap(provider).unwrap_or_else(|arc| arc.as_ref().clone() )
        })
}