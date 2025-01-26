# DeepSeek Rust Client

A Rust client library for the DeepSeek API.

## Features

- Fully async Rust implementation with `tokio`
- Type-safe chat completions API
- Ergonomic builder pattern for requests
- Comprehensive error handling using Rust's `Result` type
- First-class `serde` serialization support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
deepseek_rs = "0.1.2"
```

## Usage

Here's a basic example of how to use the DeepSeek Rust client:

```rust
use deepseek::client::Client;

#[tokio::main]
async fn main() {
    let client = Client::new("your_api_key");
    let request = RequestBody::new_messages(
        vec![Message::new_user_message("Hello".to_string())]
     );
    let response = client.chat_completion(request).await.unwrap();
    println!("{}", response);
}
```

## Examples

### Basic Usage

```rust
use deepseek_rs::{DeepSeekClient, client::chat_completions::request::{Message, RequestBody}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DeepSeekClient::default()?;
    let request = RequestBody::new_messages(vec![
        Message::new_user_message("Hello".to_string())
    ]);
    let response = client.chat_completions(request).await?;
    println!("{}", response.choices[0].message.content.unwrap());
    Ok(())
}
```

### Using the Reasoning Model

```rust
use deepseek_rs::{
    DeepSeekClient,
    client::chat_completions::request::{Message, Model, RequestBody}
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DeepSeekClient::default()?;
    let request = RequestBody::new_messages(vec![
        Message::new_user_message("What is 15 * 7?".to_string())
    ])
    .with_model(Model::DeepSeekReasoner);

    let response = client.chat_completions(request).await?;
    println!("Reasoning: {}", response.choices[0].message.reasoning_content.unwrap());
    println!("Answer: {}", response.choices[0].message.content.unwrap());
    Ok(())
}
```

For more examples, check out the [examples directory](examples/).

## Documentation

For more detailed information, please refer to the [API documentation](https://docs.deepseek.com).

## License

This project is licensed under the MIT License.
