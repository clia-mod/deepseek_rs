use crate::client::client::DeepSeekClient;

use super::request::RequestBody;
impl DeepSeekClient {
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
