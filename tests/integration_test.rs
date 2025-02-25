use claude_client::claude::ClaudeClient;
use std::env;

#[tokio::test]
async fn test_integration_list_models() {
    // Skip test if no API key is set
    if env::var("ANTHROPIC_API_KEY").is_err() {
        println!("Skipping integration test: ANTHROPIC_API_KEY not set");
        return;
    }

    let client = ClaudeClient::new().expect("Failed to create client");
    let models = client.list_models().await.expect("Failed to list models");

    // Verify we got some models back
    assert!(!models.is_empty());

    // Verify expected model properties
    let model = &models[0];
    assert!(!model.id.is_empty());
    assert!(!model.display_name.is_empty());
    assert_eq!(model.model_type, "model");
}

#[tokio::test]
async fn test_integration_send_message() {
    // Skip test if no API key is set
    if env::var("ANTHROPIC_API_KEY").is_err() {
        println!("Skipping integration test: ANTHROPIC_API_KEY not set");
        return;
    }

    let client = ClaudeClient::new().expect("Failed to create client");
    
    // Test with default model
    let response = client
        .send_message(
            None,
            "You are a helpful assistant.",
            "What is 2+2?",
        )
        .await
        .expect("Failed to send message with default model");

    // Verify we got a non-empty response
    assert!(!response.is_empty());
    
    // Verify response contains a number
    assert!(response.contains("4"));

    // Test with specific model
    let response = client
        .send_message(
            Some("claude-3-opus-20240229"),
            "You are a helpful assistant.",
            "What is 2+2?",
        )
        .await
        .expect("Failed to send message with specific model");

    // Verify we got a non-empty response
    assert!(!response.is_empty());
    
    // Verify response contains a number
    assert!(response.contains("4"));
}
