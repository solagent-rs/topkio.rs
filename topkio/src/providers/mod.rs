use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::Arc;

use crate::models::{CompletionRequest, CompletionResponse};

#[async_trait]
pub trait Provider: Send + Sync {
    async fn create_completion(&self, request: CompletionRequest) -> Result<CompletionResponse, String>;
}

pub type ProviderMap = Arc<DashMap<String, Arc<dyn Provider>>>;

pub fn init_providers(config: Vec<crate::config::ProviderConfig>) -> ProviderMap {
    let providers: ProviderMap = Arc::new(DashMap::new());
    
    for provider_config in config {
        let provider: Arc<dyn Provider> = match provider_config.name.as_str() {
            "openai" => Arc::new(providers::openai::OpenAIProvider::new(provider_config)),
            "anthropic" => Arc::new(providers::anthropic::AnthropicProvider::new(provider_config)),
            _ => continue,
        };
        
        for model in provider_config.models {
            providers.insert(model, provider.clone());
        }
    }
    
    providers
}

pub mod openai;