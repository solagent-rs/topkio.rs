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

use topkio::GeminiClient;

#[tokio::main]
async fn main() {
    let gemini_api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
    let client = GeminiClient::new(&gemini_api_key);

    let builder = client.config("gemini-1.5-flash");
    let builder = builder.stream(true);
    let builder = builder.temperature(0.8);
    let config = builder.build();

    // let prompt = "Entertain me";
    let prompt = "1 + 1 = ";

    let _ = client
        .prompt(config, prompt, &mut |res| {
            print!("{}", res);
            Ok(())
        })
        .await;

    println!("\n");
}
