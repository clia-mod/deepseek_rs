# DeepSeek Rust Client

A Rust client library for the DeepSeek API.

## Features

- Chat completions API support
- Builder pattern for request configuration
- Type-safe API parameters
- Async/await support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
deepseek = "0.1.0"
```

## Usage

Here's a basic example of how to use the DeepSeek Rust client:

```rust
use deepseek::client::Client;

#[tokio::main]
async fn main() {
    let client = Client::new("your_api_key");

    let response = client.chat_completion("Hello, world!").await.unwrap();
    println!("{}", response);
}
```

## Documentation

For more detailed information, please refer to the [API documentation](https://docs.deepseek.com).

## License

This project is licensed under the MIT License.
