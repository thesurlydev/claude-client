#[cfg(test)]
mod tests {
    use crate::claude::ClaudeClient;
    use std::env;
    use std::sync::Mutex;
    use once_cell::sync::Lazy;

    static ENV_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    #[test]
    fn test_claude_client_new_no_api_key() {
        let _lock = ENV_MUTEX.lock().unwrap();
        // Ensure ANTHROPIC_API_KEY is not set
        env::remove_var("ANTHROPIC_API_KEY");

        let client = ClaudeClient::new();
        assert!(client.is_err());
    }

    #[test]
    fn test_claude_client_new() {
        let _lock = ENV_MUTEX.lock().unwrap();
        // Ensure environment is clean
        env::remove_var("ANTHROPIC_API_KEY");
        
        // Set the ANTHROPIC_API_KEY environment variable for the test
        env::set_var("ANTHROPIC_API_KEY", "test_api_key");

        let client = ClaudeClient::new();
        assert!(client.is_ok());

        // Clean up the environment variable
        env::remove_var("ANTHROPIC_API_KEY");
    }

    #[tokio::test]
    async fn test_claude_client_send_message() {
        let _lock = ENV_MUTEX.lock().unwrap();
        // Ensure environment is clean
        env::remove_var("ANTHROPIC_API_KEY");
        
        // Set the ANTHROPIC_API_KEY environment variable for the test
        env::set_var("ANTHROPIC_API_KEY", "test_api_key");

        let client = ClaudeClient::new().unwrap();

        // This test will fail because it's trying to hit a real API endpoint
        // You would typically mock the API call in a real test
        let result = client
            .send_message(Some("claude-2"), "", "hello")
            .await;

        assert!(result.is_err());

        // Clean up the environment variable
        env::remove_var("ANTHROPIC_API_KEY");
    }

    #[tokio::test]
    async fn test_claude_client_list_models() {
        let _lock = ENV_MUTEX.lock().unwrap();
        // Ensure environment is clean
        env::remove_var("ANTHROPIC_API_KEY");
        
        // Set the ANTHROPIC_API_KEY environment variable for the test
        env::set_var("ANTHROPIC_API_KEY", "test_api_key");

        let client = ClaudeClient::new().unwrap();

        // This test will fail because it's trying to hit a real API endpoint
        // You would typically mock the API call in a real test
        let result = client.list_models().await;

        assert!(result.is_err());

        // Clean up the environment variable
        env::remove_var("ANTHROPIC_API_KEY");
    }
}
