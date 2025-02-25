pub mod claude {
    use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
    use serde::{Deserialize, Serialize};
    use std::env;

    const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1";
    const DEFAULT_MODEL: &str = "claude-3-7-sonnet-20250219";

    #[derive(Debug, Serialize)]
    pub struct Message {
        pub role: String,
        pub content: String,
    }

    #[derive(Debug, Serialize)]
    pub struct ClaudeRequest {
        pub model: String,
        pub messages: Vec<Message>,
        pub max_tokens: u32,
        pub system: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct ClaudeResponse {
        pub content: Vec<Content>,
        pub role: String,
        pub model: String,
        pub id: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Content {
        pub text: String,
        #[serde(rename = "type")]
        pub content_type: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Model {
        #[serde(rename = "type")]
        pub model_type: String,
        pub id: String,
        pub display_name: String,
        pub created_at: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct ListModelsResponse {
        pub data: Vec<Model>,
        pub has_more: bool,
        pub first_id: String,
        pub last_id: String,
    }

    pub struct ClaudeClient {
        client: reqwest::Client,
        api_key: String,
    }

    impl ClaudeClient {
        pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
            let api_key = env::var("ANTHROPIC_API_KEY")
                .map_err(|_| "ANTHROPIC_API_KEY environment variable not set")?;

            Ok(Self {
                client: reqwest::Client::new(),
                api_key,
            })
        }

        pub async fn send_message(
            &self,
            model: Option<&str>,
            system_prompt: &str,
            user_message: &str,
        ) -> Result<String, Box<dyn std::error::Error>> {
            let model = model.unwrap_or(DEFAULT_MODEL);
            let mut headers = HeaderMap::new();
            headers.insert(
                "x-api-key",
                HeaderValue::from_str(&self.api_key)
                    .map_err(|e| format!("Invalid API key format: {}", e))?,
            );
            headers.insert(
                "anthropic-api-key",
                HeaderValue::from_str(&self.api_key)
                    .map_err(|e| format!("Invalid API key format: {}", e))?,
            );
            headers.insert(
                "anthropic-version",
                HeaderValue::from_static("2023-06-01"),
            );
            headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );

            let request = ClaudeRequest {
                model: model.to_string(),
                messages: vec![
                    Message {
                        role: "user".to_string(),
                        content: user_message.to_string(),
                    },
                ],
                max_tokens: 4000,
                system: Some(system_prompt.to_string()),
            };

            let raw_response = self
                .client
                .post(format!("{}/messages", ANTHROPIC_API_URL))
                .headers(headers)
                .json(&request)
                .send()
                .await?
                .text()
                .await?;
            println!("Raw response: {}", raw_response);
            let response: ClaudeResponse = serde_json::from_str(&raw_response)?;

            Ok(response.content.into_iter()
                .filter(|c| c.content_type == "text")
                .map(|c| c.text)
                .collect::<Vec<_>>()
                .join(""))
        }

        pub async fn list_models(&self) -> Result<Vec<Model>, Box<dyn std::error::Error>> {
            let mut headers = HeaderMap::new();
            headers.insert(
                "x-api-key",
                HeaderValue::from_str(&self.api_key)
                    .map_err(|e| format!("Invalid API key format: {}", e))?,
            );
            headers.insert(
                "anthropic-api-key",
                HeaderValue::from_str(&self.api_key)
                    .map_err(|e| format!("Invalid API key format: {}", e))?,
            );
            headers.insert(
                "anthropic-version",
                HeaderValue::from_static("2023-06-01"),
            );
            headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

            let raw_response = self
                .client
                .get(format!("{}/models", ANTHROPIC_API_URL))
                .headers(headers)
                .send()
                .await?
                .text()
                .await?;
            println!("Raw response: {}", raw_response);
            let response: ListModelsResponse = serde_json::from_str(&raw_response)?;

            Ok(response.data)
        }
    }
}

#[cfg(test)]
mod tests;
