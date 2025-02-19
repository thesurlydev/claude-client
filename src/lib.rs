pub mod claude {
    use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
    use serde::{Deserialize, Serialize};
    use std::env;

    const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";

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
    }

    #[derive(Debug, Deserialize)]
    pub struct ClaudeResponse {
        pub content: Vec<Content>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Content {
        pub text: String,
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
            model: &str,
            system_prompt: &str,
            user_message: &str,
        ) -> Result<String, Box<dyn std::error::Error>> {
            let mut headers = HeaderMap::new();
            headers.insert(
                "x-api-key",
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
                        role: "system".to_string(),
                        content: system_prompt.to_string(),
                    },
                    Message {
                        role: "user".to_string(),
                        content: user_message.to_string(),
                    },
                ],
                max_tokens: 4000,
            };

            let response = self
                .client
                .post(ANTHROPIC_API_URL)
                .headers(headers)
                .json(&request)
                .send()
                .await?
                .json::<ClaudeResponse>()
                .await?;

            Ok(response.content[0].text.clone())
        }
    }
}

#[cfg(test)]
mod tests;
