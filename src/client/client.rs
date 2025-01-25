use thiserror::Error;

pub struct DeepSeekClient {
    pub(crate) url: String,
    pub(crate) api_key: String,
    pub(crate) client: reqwest::Client,
}
const URL: &'static str = "https://api.deepseek.com";

impl DeepSeekClient {
    pub fn new_with_api_key(api_key: String) -> Self {
        DeepSeekClient {
            url: URL.to_string(),
            api_key,
            client: reqwest::Client::new(),
        }
    }
    pub fn new_with_url_and_api_key(url: String, api_key: String) -> Self {
        DeepSeekClient {
            url,
            api_key,
            client: reqwest::Client::new(),
        }
    }
    pub fn default() -> Result<Self, ClientInitErrors> {
        let api_key = std::env::var("DEEP_SEEK_API_KEY")?;
        Ok(DeepSeekClient {
            url: URL.to_string(),
            api_key,
            client: reqwest::Client::new(),
        })
    }
    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = api_key;
    }
    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = api_key;
        self
    }
    pub(crate) fn default_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", self.api_key)).unwrap(),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers
    }
}
#[derive(Debug, Error)]
pub enum ClientInitErrors {
    #[error("Error getting API key from environment variable")]
    DeepSeekApiKeyNotSet(#[from] std::env::VarError),
    #[error("Unknown error")]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_with_api_key() {
        let client = DeepSeekClient::new_with_api_key("api_key".to_string());
        assert_eq!(client.api_key, "api_key");
        assert_eq!(client.url, URL);
    }
    #[test]
    fn test_new_with_url_and_api_key() {
        let client =
            DeepSeekClient::new_with_url_and_api_key("url".to_string(), "api_key".to_string());
        assert_eq!(client.api_key, "api_key");
        assert_eq!(client.url, "url");
    }
    #[test]
    fn test_default() {
        std::env::set_var("DEEP_SEEK_API_KEY", "api_key");
        let client = DeepSeekClient::default().unwrap();
        assert_eq!(client.api_key, "api_key");
        assert_eq!(client.url, URL);
    }
    #[test]
    #[should_panic]
    fn test_default_panic() {
        std::env::remove_var("DEEP_SEEK_API_KEY");
        let _ = DeepSeekClient::default().unwrap();
    }
}
