use std::sync::Arc;
use tracing::info;
use topkio_core::provider::{Provider, ProviderConfig};

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
            ProviderKind::OpenAI(m) => m == model,
            ProviderKind::Gemini(m) => m == model,
            ProviderKind::Ollama(m) => m == model,
            ProviderKind::DeepSeek(m) => m == model,
        }
    }

    // 获取提供商的标识符（用于日志或调试）
    pub fn provider_name(&self) -> &'static str {
        match self {
            ProviderKind::OpenAI(_) => "openai",
            ProviderKind::Gemini(_) => "gemini",
            ProviderKind::Ollama(_) => "ollama",
            ProviderKind::DeepSeek(_) => "deepseek",
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
pub fn create_providers(config: topkio_core::config::ProvidersConfig) -> Vec<(ProviderKind, Arc<Box<dyn Provider>>)> {
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