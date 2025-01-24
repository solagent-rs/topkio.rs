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

use serde::{Deserialize, Serialize};
use serde_json::json;
use topkio::{AgentBuilder, FunctionDeclaration, OpenAIClient, Tool};

#[derive(Deserialize)]
struct OperationArgs {
    x: i32,
    y: i32,
}

#[derive(Deserialize, Serialize)]
struct Adder;
impl Tool for Adder {
    type Args = OperationArgs;
    type Returns = i32;

    fn name(&self) -> String {
        "add".to_string()
    }

    async fn definition(&self) -> FunctionDeclaration {
        FunctionDeclaration {
            name: "add".to_string(),
            description: "Add x and y together".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "x": {
                        "type": "number",
                        "description": "The first number to add"
                    },
                    "y": {
                        "type": "number",
                        "description": "The second number to add"
                    }
                }
            }),
        }
    }

    async fn invoke(&self, args: Self::Args) -> Result<Self::Returns, ()> {
        let result = args.x + args.y;
        Ok(result)
    }
}

#[tokio::main]
async fn main() {
    let client = OpenAIClient::with_ollama("http://localhost:11434/v1");
    let agent = AgentBuilder::new(client, "llama3.2")
        .stream(true)
        .temperature(0.8)
        .tool(Adder)
        .build();

    let prompt = "x=1 add y=3";

    let f = |res: &str| {
        print!("{}", res);
    };

    let _ = agent.prompt(prompt, f).await;

    println!("\n");
}
