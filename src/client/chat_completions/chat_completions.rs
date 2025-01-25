//! Chat completions API implementation

use super::request::RequestBody;
use crate::{
    client::{chat_completions::response::ChatCompletionsResponse, client::DeepSeekClient},
    errors::request_errors::RequestErrors,
};

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
    pub async fn chat_completions(
        &self,
        request: RequestBody,
    ) -> Result<ChatCompletionsResponse, RequestErrors> {
        let url = format!("{}/chat/completions", self.url);
        let headers = self.default_headers();
        let res = self
            .client
            .post(&url)
            .headers(headers)
            .json(&request)
            .send()
            .await
            .map_err(|e| RequestErrors::from(e))?;
        match res.status() {
            reqwest::StatusCode::OK => {}
            reqwest::StatusCode::BAD_REQUEST => {
                return Err(RequestErrors::BadRequest(
                    res.text().await.map_err(|e| RequestErrors::from(e))?,
                ))
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                return Err(RequestErrors::Unauthorized(
                    res.text().await.map_err(|e| RequestErrors::from(e))?,
                ))
            }
            reqwest::StatusCode::FORBIDDEN => return Err(RequestErrors::Forbidden),
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                return Err(RequestErrors::RateLimitExceeded(
                    res.text().await.map_err(|e| RequestErrors::from(e))?,
                ))
            }
            _ => {
                return Err(RequestErrors::StatusError(
                    res.status(),
                    res.text().await.map_err(|e| RequestErrors::from(e))?,
                ))
            }
        }
        let body: ChatCompletionsResponse = res.json().await.map_err(|e| RequestErrors::from(e))?;
        Ok(body)
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
        client.chat_completions(request).await.unwrap();
    }
    #[tokio::test]
    #[ignore]
    async fn test_chat_completions_wrong_api_key() {
        let client = DeepSeekClient::new_with_api_key("wrong_api_key".to_string());
        let request =
            RequestBody::new_messages(vec![Message::new_user_message("Hello".to_string())]);
        let res = client.chat_completions(request).await;
        assert!(res.is_err());
        if let Err(RequestErrors::Unauthorized(_)) = res {
        } else {
            panic!("Expected Unauthorized error")
        }
    }
}
