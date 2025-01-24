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

use futures_util::{stream, StreamExt};

use crate::{
    primitives::{Completion, CompletionRequestBuilder, Message},
    tool::{ToolDyn, ToolSet},
};
use std::cell::OnceCell;

pub struct Agent<C: Completion> {
    pub client: C,
    pub model: String,
    pub temperature: Option<f64>,
    pub stream: Option<bool>,

    static_tools: Vec<String>,
    pub tools: ToolSet,
}

impl<C: Completion> Agent<C> {
    pub fn new(client: C, model: &str) -> Self {
        Agent {
            client,
            model: model.to_string(),
            temperature: None,
            stream: None,
            static_tools: vec![],
            tools: ToolSet::default(),
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
        // TODO: how to handle chat_history
        let static_tools = stream::iter(self.static_tools.iter())
            .filter_map(|toolname| async move {
                if let Some(tool) = self.tools.get(toolname) {
                    Some(tool.definition().await)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .await;

        let req = CompletionRequestBuilder::new()
            .model(self.model.clone())
            .prompt(prompt.to_string())
            .chat_history(chat_history)
            .temperature(self.temperature)
            .stream(self.stream)
            .tools(static_tools)
            .build();

        let cell = OnceCell::new();
        let _ = cell.set(callback);

        self.client.post(req, &self.tools, cell).await
    }
}

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

    pub fn tool(mut self, tool: impl ToolDyn + 'static) -> Self {
        self.agent.static_tools.push(tool.name());
        self.agent.tools.add(tool);
        self
    }

    pub fn build(self) -> Agent<C> {
        self.agent
    }
}
