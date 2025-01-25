//! Chat completions API implementation

use super::request::RequestBody;
use crate::client::client::DeepSeekClient;

impl DeepSeekClient {
    /// Sends a chat completion request to the DeepSeek API
    ///
    /// # Example
    /// ```no_run
    /// use deepseek::{DeepSeekClient, client::chat_completions::request::{RequestBody, Message}};
    /// # use tokio;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = DeepSeekClient::default().unwrap();
    /// let request = RequestBody::new_messages(
    ///     vec![Message::new_user_message("Hello".to_string())]
    /// );
    /// client.chat_completions(request).await;
    /// # }
    /// ```
    pub async fn chat_completions(&self, request: RequestBody) {
        let url = format!("{}/chat/completions", self.url);
        let headers = self.default_headers();
        let res = self
            .client
            .post(&url)
            .headers(headers)
            .json(&request)
            .send()
            .await
            .unwrap();
        let body = res.text().await.unwrap();
        println!("{}", body);
    }
}

#[cfg(test)]
mod tests {
    use crate::client::chat_completions::request::Message;

    use super::*;
    #[tokio::test]
    #[ignore]
    async fn test_chat_completions() {
        dotenvy::dotenv().ok();
        let client = DeepSeekClient::default().unwrap();
        let request =
            RequestBody::new_messages(vec![Message::new_user_message("Hello".to_string())]);
        client.chat_completions(request).await;
    }
}
