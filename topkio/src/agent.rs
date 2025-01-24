// Copyright 2025 zTgx
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::primitives::{Completion, CompletionRequestBuilder, FunctionDeclaration, Message};
use std::cell::OnceCell;

#[derive(Clone)]
pub struct Agent<C: Completion> {
    pub client: C,
    pub model: String,
    pub temperature: Option<f64>,
    pub stream: Option<bool>,

    pub static_tools: Vec<FunctionDeclaration>,
}

impl<C: Completion> Agent<C> {
    pub fn new(client: C, model: &str) -> Self {
        Agent {
            client,
            model: model.to_string(),
            temperature: None,
            stream: None,
            static_tools: vec![],
        }
    }
}

// TODO: return Result<(), PromptError>
impl<C: Completion> Agent<C> {
    pub async fn prompt<F>(&self, prompt: &str, callback: F) -> Result<(), ()>
    where
        F: Fn(&str) + Send + 'static,
    {
        self.chat_impl(prompt, vec![], callback).await
    }

    pub async fn chat<F>(
        &self,
        prompt: &str,
        chat_history: Vec<Message>,
        callback: F,
    ) -> Result<(), ()>
    where
        F: Fn(&str) + Send + 'static,
    {
        self.chat_impl(prompt, chat_history, callback).await
    }
}

impl<C: Completion> Agent<C> {
    async fn chat_impl<F>(
        &self,
        prompt: &str,
        chat_history: Vec<Message>,
        callback: F,
    ) -> Result<(), ()>
    where
        F: Fn(&str) + Send + 'static,
    {
        let req = CompletionRequestBuilder::new()
            .model(self.model.clone())
            .prompt(prompt.to_string())
            .chat_history(chat_history)
            .temperature(self.temperature)
            .stream(self.stream)
            .build();

        let cell = OnceCell::new();
        let _ = cell.set(callback);

        self.client.post(req, cell).await
    }
}

#[derive(Clone)]
pub struct AgentBuilder<C: Completion> {
    pub agent: Agent<C>,
}

impl<C: Completion> AgentBuilder<C> {
    pub fn new(client: C, model: &str) -> Self {
        AgentBuilder {
            agent: Agent::new(client, model),
        }
    }
}

impl<C: Completion> AgentBuilder<C> {
    pub fn stream(mut self, stream: bool) -> Self {
        self.agent.stream = Some(stream);
        self
    }

    pub fn temperature(mut self, temparature: f64) -> Self {
        self.agent.temperature = Some(temparature);
        self
    }

    pub fn tool(mut self, tool: FunctionDeclaration) -> Self {
        self.agent.static_tools.push(tool);

        self
    }

    pub fn build(self) -> Agent<C> {
        self.agent
    }
}
