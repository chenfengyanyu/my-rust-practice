// https://docs.rs/reqwest/0.11.14/reqwest/
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Debug, Serialize, Deserialize)]
struct InfoResponse {
    data: String,
    // headers: HashMap<String, String>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut info = HashMap::new();
    info.insert("apiKey", "sk-qG9REgsuFWMP1my8dqsGT3BlbkFxxxxxxxxx");
    info.insert("sessionId", "8d1cb9b0-d535-43a8-b738-4f61b1608579");
    info.insert("content", "你是谁？");

    let client = reqwest::Client::new();
    let res = client.post("https://api.openai-proxy.com/v1/chat/completions")
    .json(&info)
    .send()
    .await?
    .json::<InfoResponse>()
    .await?;

    println!("{:#?}", res);
    Ok(())
}