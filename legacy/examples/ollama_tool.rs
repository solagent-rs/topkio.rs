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
