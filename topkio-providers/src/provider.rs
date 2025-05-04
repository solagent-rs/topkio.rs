use std::sync::Arc;
use tracing::info;
use topkio_core::provider::{Provider, ProviderConfig};

use crate::openai::OpenAIProvider;
use crate::gemini::GeminiProvider;
use crate::ollama::OllamaProvider;
use crate::deepseek::DeepSeekProvider;

#[non_exhaustive]
pub enum ProviderKind {
    /// Ollama model, e.g., "llama3.2".
    Ollama(String),
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

impl ProviderKind {
    // 从配置键和模型名称创建 ProviderKind
    pub fn from_key_and_model(key: &str, model: &str) -> Option<Self> {
        match key {
            "ollama" => Some(ProviderKind::Ollama(model.to_string())),
            _ => None,
        }
    }

    // 根据模型名称匹配 ProviderKind
    pub fn matches_model(&self, model: &str) -> bool {
        match self {
            ProviderKind::Ollama(m) => m == model,
        }
    }

    // 获取提供商的标识符（用于日志或调试）
    pub fn provider_name(&self) -> &'static str {
        match self {
            ProviderKind::Ollama(_) => "ollama",
        }
    }
}

// 提供商工厂 trait，用于创建提供商实例
trait ProviderFactory {
    fn create(&self, config: ProviderConfig) -> Box<dyn Provider>;
}

// 提供商注册项
struct ProviderRegistry {
    key: &'static str,
    factory: Box<dyn ProviderFactory>,
    default_models: Vec<&'static str>,
}

impl ProviderRegistry {
    fn new<F: ProviderFactory + 'static>(
        key: &'static str,
        factory: F,
        default_models: Vec<&'static str>,
    ) -> Self {
        ProviderRegistry {
            key,
            factory: Box::new(factory),
            default_models,
        }
    }
}

// 宏定义提供商注册
macro_rules! register_provider {
    ($key:expr, $factory:ty, $models:expr) => {
        ProviderRegistry::new($key, <$factory>::default(), $models)
    };
}

// 为每个提供商实现工厂
struct OpenAIProviderFactory;
impl ProviderFactory for OpenAIProviderFactory {
    fn create(&self, config: ProviderConfig) -> Box<dyn Provider> {
        Box::new(OpenAIProvider::new(config))
    }
}
impl Default for OpenAIProviderFactory {
    fn default() -> Self {
        OpenAIProviderFactory
    }
}

struct GeminiProviderFactory;
impl ProviderFactory for GeminiProviderFactory {
    fn create(&self, config: ProviderConfig) -> Box<dyn Provider> {
        Box::new(GeminiProvider::new(config))
    }
}
impl Default for GeminiProviderFactory {
    fn default() -> Self {
        GeminiProviderFactory
    }
}

struct OllamaProviderFactory;
impl ProviderFactory for OllamaProviderFactory {
    fn create(&self, config: ProviderConfig) -> Box<dyn Provider> {
        Box::new(OllamaProvider::new(config))
    }
}
impl Default for OllamaProviderFactory {
    fn default() -> Self {
        OllamaProviderFactory
    }
}

struct DeepSeekProviderFactory;
impl ProviderFactory for DeepSeekProviderFactory {
    fn create(&self, config: ProviderConfig) -> Box<dyn Provider> {
        Box::new(DeepSeekProvider::new(config))
    }
}
impl Default for DeepSeekProviderFactory {
    fn default() -> Self {
        DeepSeekProviderFactory
    }
}

// 创建提供商
pub fn create_providers(config: topkio_core::provider::ProvidersConfig) -> Vec<(ProviderKind, Arc<Box<dyn Provider>>)> {
    let mut providers = Vec::new();

    // 提供商注册表
    let registries = vec![
        register_provider!(
            "openai",
            OpenAIProviderFactory,
            vec!["gpt-4", "gpt-3.5-turbo"]
        ),
        register_provider!(
            "gemini",
            GeminiProviderFactory,
            vec!["gemini-pro", "gemini-ultra"]
        ),
        register_provider!(
            "ollama",
            OllamaProviderFactory,
            vec!["llama3", "mistral"]
        ),
        register_provider!(
            "deepseek",
            DeepSeekProviderFactory,
            vec!["deepseek-rag", "deepseek-pro"]
        ),
    ];

    // 遍历注册表并检查配置
    for registry in registries {
        let cfg = match registry.key {
            "openai" => config.openai.clone(),
            "gemini" => config.gemini.clone(),
            "ollama" => config.ollama.clone(),
            "deepseek" => config.deepseek.clone(),
            _ => None,
        };

        if let Some(cfg) = cfg {
            for model in registry.default_models {
                if let Some(kind) = ProviderKind::from_key_and_model(registry.key, model) {
                    providers.push((
                        kind,
                        Arc::new(registry.factory.create(cfg.clone())),
                    ));
                    info!("{} provider initialized with model: {}", registry.key, model);
                }
            }
        } else {
            info!("No configuration found for provider '{}', skipped", registry.key);
        }
    }

    providers
}

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
            Arc::try_unwrap(provider).unwrap_or_else(|arc| arc.as_ref().clone())
        })
}