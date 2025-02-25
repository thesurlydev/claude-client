# claude-client

A Rust client for the [Claude](https://claude.ai/) API. This client provides a simple interface to interact with Claude's API, including sending messages and listing available models.

## Features

- Send messages to Claude with optional system prompts
- List available Claude models
- Built-in error handling and type safety
- Async/await support using tokio
- Default to latest Claude model (claude-3-7-sonnet-20250219)

## Building and Testing

### Prerequisites

- Rust 1.75 or later
- An Anthropic API key for running tests

### Building

To build the project:

```sh
cargo build
```

### Running Tests

The test suite includes both unit tests and integration tests. The integration tests require a valid Anthropic API key to be set in the environment:

```sh
# Set your API key
export ANTHROPIC_API_KEY="your_api_key_here"

# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration_test
```

If no API key is set, the integration tests will be skipped automatically.

## Usage

First, set your Anthropic API key as an environment variable:

```sh
export ANTHROPIC_API_KEY="your_api_key_here"
```

### Basic Usage

```rust
use claude_client::claude::ClaudeClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let client = ClaudeClient::new()?;

    // Send a message using the default model (claude-3-7-sonnet-20250219)
    let response = client
        .send_message(
            None,
            "You are a helpful assistant.",
            "What is the capital of France?"
        )
        .await?;
    println!("Response: {}", response);

    Ok(())
}
```

### Using a Specific Model

```rust
use claude_client::claude::ClaudeClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClaudeClient::new()?;

    // Send a message using a specific model
    let response = client
        .send_message(
            Some("claude-3-opus-20240229"),
            "You are a helpful assistant.",
            "What is the capital of France?"
        )
        .await?;
    println!("Response: {}", response);

    Ok(())
}
```

### Listing Available Models

```rust
use claude_client::claude::ClaudeClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClaudeClient::new()?;

    // Get a list of available models
    let models = client.list_models().await?;

    // Print model information
    for model in models {
        println!("Model ID: {}", model.id);
        println!("Display Name: {}", model.display_name);
        println!("Created At: {}", model.created_at);
        println!("---");
    }

    Ok(())
}
```

## Error Handling

The client uses Rust's `Result` type for error handling. All errors are wrapped in a `Box<dyn std::error::Error>`. Common errors include:

- Missing or invalid API key
- Network connectivity issues
- Invalid model selection
- API rate limits

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

