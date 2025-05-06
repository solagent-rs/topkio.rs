use std::sync::Arc;
use topkio_core::config::Config;
use topkio_core::error::TopkioError;
use topkio_core::provider::Provider;
use topkio_providers::provider::{create_providers, select_provider, ProviderKind};

#[derive(Clone)]
pub struct AppState {
    config: Config,
    providers: Vec<(ProviderKind, Arc<Box<dyn Provider>>)>,
}

impl AppState {
    pub fn new(config: Config) -> Result<Self, TopkioError> {
        let providers = create_providers(config.providers.clone());
        Ok(AppState { config, providers })
    }

    pub fn select_provider(&self, model: &str) -> Option<Box<dyn Provider>> {
        select_provider(&self.providers, model)
    }
}