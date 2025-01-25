//! # DeepSeek Rust Client
//!
//! A Rust client for interacting with the DeepSeek API.
//!
//! ## Usage
//!
//! ```no_run
//! use deepseek::{DeepSeekClient, client::chat_completions::request::{RequestBody, Message}};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = DeepSeekClient::default().unwrap();
//!     let request = RequestBody::new_messages(
//!         vec![Message::new_user_message("Hello".to_string())]
//!     );
//!     client.chat_completions(request).await;
//! }
//! ```

pub mod client;
pub mod errors;

// Re-exports for convenience
pub use client::chat_completions::request;
pub use client::client::DeepSeekClient;
