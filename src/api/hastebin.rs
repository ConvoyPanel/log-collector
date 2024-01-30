use std::error::Error;

pub async fn upload(payload: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://paste.frocdn.com/documents")
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()
        .await?
        .text()
        .await?;

    let response_json: serde_json::Value = serde_json::from_str(&res)?;

    let key = response_json["key"].as_str().unwrap();
    Ok(format!("https://paste.frocdn.com/{}", key))
}