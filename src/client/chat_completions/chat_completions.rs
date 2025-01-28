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
    /// use deepseek_rs::{DeepSeekClient, client::chat_completions::request::{RequestBody, Message}};
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
    use super::*;
    use crate::{
        client::chat_completions::request::{Message, Model, Temperature},
        request::ResponseFormat,
    };

    #[tokio::test]
    #[ignore]
    async fn test_chat_completions() {
        dotenvy::dotenv().ok();
        let client = DeepSeekClient::default().unwrap();
        let request =
            RequestBody::new_messages(vec![Message::new_user_message("Hello".to_string())]);
        let response = client.chat_completions(request).await.unwrap();
        assert!(!response.choices.is_empty());
        assert!(!response.id.is_empty());
    }
    #[tokio::test]
    #[ignore]
    async fn test_chat_completions_reasoning() {
        dotenvy::dotenv().ok();
        let client = DeepSeekClient::default().unwrap();
        let request =
            RequestBody::new_messages(vec![Message::new_user_message("Hello".to_string())])
                .with_model(Model::DeepSeekReasoner);
        let response = client.chat_completions(request).await.unwrap();
        assert!(!response.choices.is_empty());
        assert!(!response.id.is_empty());
    }
    #[tokio::test]
    #[ignore]
    async fn test_chat_completions_with_system_message() {
        dotenvy::dotenv().ok();
        let client = DeepSeekClient::default().unwrap();
        let request = RequestBody::new_messages(vec![
            Message::new_system_message("You are a helpful assistant.".to_string()),
            Message::new_user_message("What is 2+2?".to_string()),
        ]);
        let response = client.chat_completions(request).await.unwrap();
        assert!(!response.choices.is_empty());
    }
    #[tokio::test]
    #[ignore]
    async fn test_chat_completions_with_system_message_json() {
        dotenvy::dotenv().ok();
        let client = DeepSeekClient::default().unwrap();
        let request = RequestBody::new_messages(vec![
            Message::new_system_message("You are a helpful assistant.".to_string()),
            Message::new_user_message("What is 2+2?".to_string()),
            Message::new_user_message(r#"```json { "output" : <value>} "#.to_string()),
        ])
        .with_response_format(ResponseFormat::new(
            crate::request::ResponseFormatType::Json,
        ));
        let response = client.chat_completions(request).await.unwrap();
        assert!(!response.choices.is_empty());
    }
    #[tokio::test]
    #[ignore]
    async fn test_chat_completions_with_reasoner_model() {
        dotenvy::dotenv().ok();
        let client = DeepSeekClient::default().unwrap();
        let request = RequestBody::new_messages(vec![Message::new_user_message(
            "Solve this math problem: 15 * 7".to_string(),
        )])
        .with_model(Model::DeepSeekReasoner);

        let response = client.chat_completions(request).await.unwrap();
        assert!(!response.choices.is_empty());
        // Reasoner model should include reasoning_content
        assert!(response.choices[0].message.reasoning_content.is_some());
    }

    #[tokio::test]
    #[ignore]
    async fn test_chat_completions_with_temperature() {
        dotenvy::dotenv().ok();
        let client = DeepSeekClient::default().unwrap();
        let request = RequestBody::new_messages(vec![Message::new_user_message(
            "Tell me a creative story.".to_string(),
        )])
        .with_temperature(Temperature::new(0.9));

        let response = client.chat_completions(request).await.unwrap();
        assert!(!response.choices.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn test_chat_completions_wrong_api_key() {
        let client = DeepSeekClient::new_with_api_key("wrong_api_key".to_string());
        let request =
            RequestBody::new_messages(vec![Message::new_user_message("Hello".to_string())]);
        let res = client.chat_completions(request).await;
        assert!(matches!(res, Err(RequestErrors::Unauthorized(_))));
    }

    #[tokio::test]
    #[ignore]
    async fn test_chat_completions_empty_messages() {
        dotenvy::dotenv().ok();
        let client = DeepSeekClient::default().unwrap();
        let request = RequestBody::new_messages(vec![]);
        let res = client.chat_completions(request).await;
        assert!(matches!(res, Err(RequestErrors::BadRequest(_))));
    }

    #[tokio::test]
    #[ignore]
    async fn test_chat_completions_conversation() {
        dotenvy::dotenv().ok();
        let client = DeepSeekClient::default().unwrap();

        // First message
        let request = RequestBody::new_messages(vec![Message::new_user_message(
            "What is the capital of France?".to_string(),
        )]);
        let response1 = client.chat_completions(request).await.unwrap();

        // Follow-up message using assistant's response
        let request = RequestBody::new_messages(vec![
            Message::new_user_message("What is the capital of France?".to_string()),
            Message::new_assistant_message(
                response1.choices[0]
                    .message
                    .content
                    .clone()
                    .unwrap_or_default(),
            ),
            Message::new_user_message("What is its population?".to_string()),
        ]);
        let response2 = client.chat_completions(request).await.unwrap();

        assert!(!response1.choices.is_empty());
        assert!(!response2.choices.is_empty());
    }
}
