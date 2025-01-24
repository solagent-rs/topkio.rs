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

mod agent;
mod constants;
mod gemini;
mod openai;
mod primitives;
mod tool;
mod utils;

pub use agent::*;
pub use gemini::Client as GeminiClient;
pub use openai::Client as OpenAIClient;

use primitives::CompletionRequest;
use std::cell::OnceCell;

pub trait Completion {
    fn post<F>(
        &self,
        req: CompletionRequest,
        callback: OnceCell<F>,
    ) -> impl std::future::Future<Output = Result<(), ()>> + Send
    where
        F: Fn(&str) + Send + 'static;
}
