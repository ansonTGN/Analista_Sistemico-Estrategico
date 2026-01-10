// src/openai.rs

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct InputMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct ResponsesRequest {
    pub model: String,
    pub input: Vec<InputMessage>,
    pub temperature: f32,
    pub store: bool,
}

#[derive(Deserialize, Default)]
pub struct ResponsesResponse {
    pub output: Vec<OutputItem>,
}

#[derive(Deserialize, Default)]
pub struct OutputItem {
    #[serde(rename = "type")]
    pub item_type: String, // "message", etc.
    pub content: Option<Vec<ContentPart>>,
}

#[derive(Deserialize, Default)]
pub struct ContentPart {
    #[serde(rename = "type")]
    pub part_type: String, // "output_text"
    pub text: Option<String>,
}

pub fn extract_output_text(resp: &ResponsesResponse) -> Option<String> {
    for item in &resp.output {
        if item.item_type == "message" {
            if let Some(parts) = &item.content {
                for p in parts {
                    if p.part_type == "output_text" {
                        if let Some(t) = &p.text {
                            return Some(t.clone());
                        }
                    }
                }
            }
        }
    }
    None
}

pub async fn call_responses_api(
    client: &Client,
    api_key: &str,
    req: &ResponsesRequest,
) -> Result<ResponsesResponse, String> {
    let resp = client
        .post("https://api.openai.com/v1/responses")
        .bearer_auth(api_key)
        .json(req)
        .send()
        .await
        .map_err(|e| format!("Error conectando con OpenAI: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Error OpenAI ({status}): {body}"));
    }

    resp.json::<ResponsesResponse>()
        .await
        .map_err(|e| format!("Error parseando JSON OpenAI: {e}"))
}
