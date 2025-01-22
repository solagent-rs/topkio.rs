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

use topkio::OpenAIClient;

#[tokio::main]
async fn main() {
    let client = OpenAIClient::with_ollama("http://localhost:11434/v1");

    let agent_builder = client.agent("llama3.2");
    let builder = agent_builder.stream(true);
    let builder = builder.temperature(0.8);

    // let prompt = "Entertain me";
    let prompt = "1 + 1 = ";

    let _ = client
        .prompt(builder, prompt, &mut |res| {
            print!("{}", res);
            Ok(())
        })
        .await;

    println!("\n");
}
