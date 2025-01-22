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

use crate::primitives::Message;
use serde::{Deserialize, Serialize};

pub trait Prompt {
    fn prompt(&self, prompt: &str) -> impl std::future::Future<Output = Result<String, ()>> + Send;
}

pub trait Chat {
    fn chat(
        &self,
        prompt: &str,
        chat_history: Vec<Message>,
    ) -> impl std::future::Future<Output = Result<String, ()>> + Send;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Agent {
    pub model: String,
    pub temperature: Option<f32>,
    pub stream: Option<bool>,
}

impl Agent {
    pub fn new(model: String) -> Self {
        Agent {
            model,
            temperature: None,
            stream: None,
        }
    }
}

// impl Prompt for Agent {
//     fn prompt(
//             &self,
//             prompt: &str,
//         ) -> impl std::future::Future<Output = Result<String, ()>> + Send {

//             // self.chat(prompt, vec![])
//     }
// }

// impl Chat for Agent {
//     fn chat(
//             &self,
//             prompt: &str,
//             chat_history: Vec<Message>,
//         ) -> impl std::future::Future<Output = Result<String, ()>> + Send {

//             todo!()
//     }
// }

/// Invoke in a chained manner through the Builder pattern.
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentBuilder {
    #[serde(flatten)]
    pub agent: Agent,
}

impl AgentBuilder {
    pub fn new(model: String) -> Self {
        AgentBuilder {
            agent: Agent::new(model),
        }
    }
}

impl AgentBuilder {
    pub fn stream(mut self, stream: bool) -> Self {
        self.agent.stream = Some(stream);
        self
    }

    pub fn temperature(mut self, temparature: f32) -> Self {
        self.agent.temperature = Some(temparature);
        self
    }

    pub fn build(self) -> Agent {
        self.agent
    }
}
