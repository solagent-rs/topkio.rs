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

use crate::primitives::FunctionDeclaration;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, future::Future, pin::Pin};

pub trait Tool: Send + Sync {
    type Args: for<'a> Deserialize<'a> + Send + Sync;
    type Returns: Serialize;

    fn name(&self) -> String;
    fn definition(&self) -> impl Future<Output = FunctionDeclaration> + Send + Sync;
    fn invoke(
        &self,
        args: Self::Args,
    ) -> impl Future<Output = Result<Self::Returns, ()>> + Send + Sync;
}

pub trait ToolDyn: Send + Sync {
    fn name(&self) -> String;

    fn definition(
        &self,
    ) -> Pin<Box<dyn futures_util::Future<Output = FunctionDeclaration> + Send + '_>>;

    fn invoke(
        &self,
        args: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, ()>> + Send + Sync + '_>>;
}

impl<T: Tool> ToolDyn for T {
    fn name(&self) -> String {
        self.name()
    }

    fn definition(
        &self,
    ) -> Pin<Box<dyn futures_util::Future<Output = FunctionDeclaration> + Send + '_>> {
        Box::pin(<Self as Tool>::definition(self))
    }

    fn invoke(
        &self,
        args: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, ()>> + Send + Sync + '_>> {
        Box::pin(async move {
            match serde_json::from_str(&args) {
                Ok(args) => <Self as Tool>::invoke(self, args)
                    .await
                    .map_err(|_e| ())
                    .map(|output| {
                        let x = serde_json::to_string(&output).ok();
                        let y = x.unwrap_or_default();
                        y
                    }),
                Err(_e) => Err(()),
            }
        })
    }
}

pub(crate) enum ToolType {
    Simple(Box<dyn ToolDyn>),
}

impl ToolType {
    pub fn name(&self) -> String {
        match self {
            ToolType::Simple(tool) => tool.name(),
        }
    }

    pub async fn definition(&self) -> FunctionDeclaration {
        match self {
            ToolType::Simple(tool) => tool.definition().await,
        }
    }

    pub async fn invoke(&self, args: String) -> Result<String, ()> {
        match self {
            ToolType::Simple(tool) => tool.invoke(args).await,
        }
    }
}

#[derive(Default)]
pub struct ToolSet {
    pub(crate) tools: HashMap<String, ToolType>,
}

impl ToolSet {
    pub(crate) fn get(&self, toolname: &str) -> Option<&ToolType> {
        self.tools.get(toolname)
    }

    pub fn add(&mut self, tool: impl ToolDyn + 'static) {
        self.tools
            .insert(tool.name(), ToolType::Simple(Box::new(tool)));
    }

    pub async fn invoke(&self, toolname: &str, args: String) -> Result<String, String> {
        if let Some(tool) = self.tools.get(toolname) {
            Ok(tool.invoke(args).await.unwrap_or("".to_string()))
        } else {
            Err("ToolNotFoundError".to_string())
        }
    }
}
