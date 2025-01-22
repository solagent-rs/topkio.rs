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
pub struct Config {
    pub model: String,
    pub temperature: Option<f32>,
    pub stream: Option<bool>,
}

impl Config {
    pub fn new(model: String) -> Self {
        Config {
            model,
            temperature: None,
            stream: None,
        }
    }
}

// impl Prompt for Config {
//     fn prompt(
//             &self,
//             prompt: &str,
//         ) -> impl std::future::Future<Output = Result<String, ()>> + Send {

//             // self.chat(prompt, vec![])
//     }
// }

// impl Chat for Config {
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
pub struct ConfigBuilder {
    #[serde(flatten)]
    pub agent: Config,
}

impl ConfigBuilder {
    pub fn new(model: String) -> Self {
        ConfigBuilder {
            agent: Config::new(model),
        }
    }
}

impl ConfigBuilder {
    pub fn stream(mut self, stream: bool) -> Self {
        self.agent.stream = Some(stream);
        self
    }

    pub fn temperature(mut self, temparature: f32) -> Self {
        self.agent.temperature = Some(temparature);
        self
    }

    pub fn build(self) -> Config {
        self.agent
    }
}
