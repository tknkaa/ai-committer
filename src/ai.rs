use reqwest::{Client, Error};
use serde_json::json;

pub async fn ask_ai(api_key: &str, prompt: &str) -> Result<String, Error> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

    let client = Client::new();
    let body = json!({
        "contents": [{
            "parts": [{"text": prompt}]
        }]
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await?
        .text()
        .await?;

    let parsed_response = parse_response(&response);
    Ok(parsed_response)
}

fn parse_response(response: &str) -> String {
    let v: serde_json::Value = serde_json::from_str(response).unwrap_or_default();

    v["candidates"]
        .as_array()
        .and_then(|candidates| candidates.get(0))
        .and_then(|candidate| candidate["content"]["parts"].as_array())
        .and_then(|parts| parts.get(0))
        .and_then(|part| part["text"].as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "No response from AI".to_string())
}
