//! Request types for the chat completions API
//!
//! This module contains all the types needed to construct a chat completion request.

use std::fmt;

use serde::{Deserialize, Serialize};

/// A chat completion request body
///
/// # Example
/// ```
/// use deepseek_rs::client::chat_completions::request::{RequestBody, Message};
///
/// let request = RequestBody::new_messages(
///     vec![Message::new_user_message("Hello".to_string())]
/// );
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RequestBody {
    messages: Vec<Message>,
    model: Model,
    frequency_penalty: Option<FrequencyPenalty>,
    max_tokens: Option<MaxTokens>,
    presence_penalty: Option<PresencePenalty>,
    response_format: Option<ResponseFormat>,
    stop: Option<StopType>,
    stream: Option<bool>,
    stream_options: Option<StreamOptions>,
    temperature: Option<Temperature>,
    top_p: Option<TopP>,
    logprobs: Option<bool>,
    top_logprobs: Option<TopLogProbs>,
}

impl RequestBody {
    /// Creates a new RequestBody with specified messages and model
    ///
    /// # Examples
    /// ```
    /// use deepseek_rs::client::chat_completions::request::{RequestBody, Message, Model};
    ///
    /// let request = RequestBody::new(
    ///     vec![Message::new_user_message("Hello".to_string())],
    ///     Model::DeepseekChat
    /// );
    /// ```
    pub fn new(messages: Vec<Message>, model: Model) -> Self {
        RequestBody {
            messages,
            model,
            ..Default::default()
        }
    }

    /// Creates a new RequestBody with only messages, using default model
    ///
    /// # Examples
    /// ```
    /// use deepseek_rs::client::chat_completions::request::{RequestBody, Message};
    ///
    /// let request = RequestBody::new_messages(
    ///     vec![Message::new_user_message("Hello".to_string())]
    /// );
    /// ```
    pub fn new_messages(messages: Vec<Message>) -> Self {
        RequestBody {
            messages,
            ..Default::default()
        }
    }

    /// Sets the messages for this request
    pub fn with_messages(mut self, messages: Vec<Message>) -> Self {
        self.messages = messages;
        self
    }

    /// Sets the model for this request
    pub fn with_model(mut self, model: Model) -> Self {
        self.model = model;
        self
    }

    /// Sets the frequency penalty (-2.0 to 2.0)
    pub fn with_frequency_penalty(mut self, penalty: FrequencyPenalty) -> Self {
        self.frequency_penalty = Some(penalty);
        self
    }

    /// Sets maximum tokens in the response (1 to 8192)
    pub fn with_max_tokens(mut self, tokens: MaxTokens) -> Self {
        self.max_tokens = Some(tokens);
        self
    }

    /// Sets presence penalty (-2.0 to 2.0)
    pub fn with_presence_penalty(mut self, penalty: PresencePenalty) -> Self {
        self.presence_penalty = Some(penalty);
        self
    }

    /// Sets response format (JSON or Text)
    pub fn with_response_format(mut self, format: ResponseFormat) -> Self {
        self.response_format = Some(format);
        self
    }

    /// Sets stop sequence(s)
    pub fn with_stop(mut self, stop: StopType) -> Self {
        self.stop = Some(stop);
        self
    }

    /// Enables/disables streaming
    pub fn with_stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    /// Sets streaming options
    pub fn with_stream_options(mut self, options: StreamOptions) -> Self {
        self.stream_options = Some(options);
        self
    }

    /// Sets temperature (0.0 to 2.0)
    pub fn with_temperature(mut self, temp: Temperature) -> Self {
        self.temperature = Some(temp);
        self
    }

    /// Sets top_p (0.0 to 1.0)
    pub fn with_top_p(mut self, top_p: TopP) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Enables/disables logprobs
    pub fn with_logprobs(mut self, logprobs: bool) -> Self {
        self.logprobs = Some(logprobs);
        self
    }

    /// Sets number of top logprobs to return (0 to 20)
    pub fn with_top_logprobs(mut self, top_logprobs: TopLogProbs) -> Self {
        self.top_logprobs = Some(top_logprobs);
        self
    }
}

impl Default for RequestBody {
    fn default() -> Self {
        RequestBody {
            messages: Vec::new(),
            model: Model::DeepseekChat,
            frequency_penalty: None,
            max_tokens: None,
            presence_penalty: None,
            response_format: None,
            stop: None,
            stream: None,
            stream_options: None,
            temperature: None,
            top_p: None,
            logprobs: None,
            top_logprobs: None,
        }
    }
}

/// Available models for chat completions
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Model {
    #[serde(rename = "deepseek-chat")]
    DeepseekChat,
    #[serde(rename = "deepseek-reasoner")]
    DeepSeekReasoner,
}

/// Frequency penalty value between -2 and 2
///
/// # Examples
/// ```
/// use deepseek_rs::client::chat_completions::request::FrequencyPenalty;
///
/// let penalty = FrequencyPenalty::new(1);
/// assert_eq!(penalty.to_string(), "1");
///
/// // Values are clamped
/// let max_penalty = FrequencyPenalty::new(3);
/// assert_eq!(max_penalty.to_string(), "2");
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct FrequencyPenalty(i8);

impl FrequencyPenalty {
    pub fn new(penalty: i8) -> Self {
        if penalty > 2 {
            FrequencyPenalty(2)
        } else if penalty < -2 {
            FrequencyPenalty(-2)
        } else {
            FrequencyPenalty(penalty)
        }
    }
}

impl Default for FrequencyPenalty {
    fn default() -> Self {
        FrequencyPenalty(0)
    }
}
impl fmt::Display for FrequencyPenalty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]

pub struct MaxTokens(i16);

impl MaxTokens {
    pub fn new(tokens: i16) -> Self {
        if tokens > 8192 {
            MaxTokens(8192)
        } else if tokens < 1 {
            MaxTokens(1)
        } else {
            MaxTokens(tokens)
        }
    }
}
impl Default for MaxTokens {
    fn default() -> Self {
        MaxTokens(4096)
    }
}

impl fmt::Display for MaxTokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]

pub struct PresencePenalty(i8);

impl PresencePenalty {
    pub fn new(penalty: i8) -> Self {
        if penalty > 2 {
            PresencePenalty(2)
        } else if penalty < -2 {
            PresencePenalty(-2)
        } else {
            PresencePenalty(penalty)
        }
    }
}

impl Default for PresencePenalty {
    fn default() -> Self {
        PresencePenalty(0)
    }
}
impl fmt::Display for PresencePenalty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]

pub struct ResponseFormat {
    pub response_format: ResponseFormatType,
}
impl Default for ResponseFormat {
    fn default() -> Self {
        ResponseFormat {
            response_format: ResponseFormatType::Json,
        }
    }
}
impl ResponseFormat {
    pub fn new(response_format: ResponseFormatType) -> Self {
        ResponseFormat { response_format }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]

pub enum ResponseFormatType {
    #[serde(rename = "json_object")]
    Json,
    #[serde(rename = "text")]
    Text,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum StopType {
    Stop(String),
    StopArray(Vec<String>),
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]

pub struct StreamOptions {
    pub include_usage: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]

pub struct Temperature(f32);

impl Temperature {
    pub fn new(temp: f32) -> Self {
        if temp > 2.0 {
            Temperature(2.0)
        } else if temp < 0.0 {
            Temperature(0.0)
        } else {
            Temperature(temp)
        }
    }
}
impl Default for Temperature {
    fn default() -> Self {
        Temperature(0.7)
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]

pub struct TopP(f32);
impl TopP {
    pub fn new(top_p: f32) -> Self {
        if top_p > 1.0 {
            TopP(1.0)
        } else if top_p < 0.0 {
            TopP(0.0)
        } else {
            TopP(top_p)
        }
    }
}
impl Default for TopP {
    fn default() -> Self {
        TopP(1.0)
    }
}
impl fmt::Display for TopP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]

pub struct TopLogProbs(u8);

impl TopLogProbs {
    pub fn new(top_log_probs: u8) -> Self {
        if top_log_probs > 20 {
            TopLogProbs(20)
        } else {
            TopLogProbs(top_log_probs)
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]

pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "assistant")]
    Assistant,
}

/// A chat message with role and content
///
/// # Examples
/// ```
/// use deepseek_rs::client::chat_completions::request::{Message, Role};
///
/// let user_msg = Message::new_user_message("Hello".to_string());
/// assert!(matches!(user_msg.role, Role::User));
///
/// let system_msg = Message::new_system_message_with_name(
///     "Configure the assistant".to_string(),
///     "sys".to_string()
/// );
/// assert!(matches!(system_msg.role, Role::System));
/// assert_eq!(system_msg.name, Some("sys".to_string()));
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub name: Option<String>,
}
impl Message {
    /// Creates a new message with specified role, content and optional name
    pub fn new(role: Role, content: String, name: Option<String>) -> Self {
        Message {
            role,
            content,
            name,
        }
    }

    /// Creates a new user message
    pub fn new_user_message(content: String) -> Self {
        Message {
            role: Role::User,
            content,
            name: None,
        }
    }

    /// Creates a new system message
    pub fn new_system_message(content: String) -> Self {
        Message {
            role: Role::System,
            content,
            name: None,
        }
    }

    /// Creates a new assistant message
    pub fn new_assistant_message(content: String) -> Self {
        Message {
            role: Role::Assistant,
            content,
            name: None,
        }
    }

    /// Creates a new user message with a name
    pub fn new_user_message_with_name(content: String, name: String) -> Self {
        Message {
            role: Role::User,
            content,
            name: Some(name),
        }
    }

    /// Creates a new system message with a name
    pub fn new_system_message_with_name(content: String, name: String) -> Self {
        Message {
            role: Role::System,
            content,
            name: Some(name),
        }
    }

    /// Creates a new assistant message with a name
    pub fn new_assistant_message_with_name(content: String, name: String) -> Self {
        Message {
            role: Role::Assistant,
            content,
            name: Some(name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_penalty() {
        assert_eq!(FrequencyPenalty::new(1).0, 1);
        assert_eq!(FrequencyPenalty::new(3).0, 2); // Should clamp to max
        assert_eq!(FrequencyPenalty::new(-3).0, -2); // Should clamp to min
        assert_eq!(FrequencyPenalty::default().0, 0);
    }

    #[test]
    fn test_max_tokens() {
        assert_eq!(MaxTokens::new(100).0, 100);
        assert_eq!(MaxTokens::new(9000).0, 8192); // Should clamp to max
        assert_eq!(MaxTokens::new(0).0, 1); // Should clamp to min
        assert_eq!(MaxTokens::default().0, 4096);
    }

    #[test]
    fn test_presence_penalty() {
        assert_eq!(PresencePenalty::new(1).0, 1);
        assert_eq!(PresencePenalty::new(3).0, 2); // Should clamp to max
        assert_eq!(PresencePenalty::new(-3).0, -2); // Should clamp to min
        assert_eq!(PresencePenalty::default().0, 0);
    }

    #[test]
    fn test_response_format() {
        let json_format = ResponseFormat::new(ResponseFormatType::Json);
        let text_format = ResponseFormat::new(ResponseFormatType::Text);
        let default_format = ResponseFormat::default();

        match default_format.response_format {
            ResponseFormatType::Json => assert!(true),
            _ => assert!(false, "Default should be Json"),
        }
        match json_format.response_format {
            ResponseFormatType::Json => assert!(true),
            _ => assert!(false, "Expected Json"),
        }
        match text_format.response_format {
            ResponseFormatType::Text => assert!(true),
            _ => assert!(false, "Expected Text"),
        }
    }

    #[test]
    fn test_temperature() {
        assert_eq!(Temperature::new(1.0).0, 1.0);
        assert_eq!(Temperature::new(3.0).0, 2.0); // Should clamp to max
        assert_eq!(Temperature::new(-1.0).0, 0.0); // Should clamp to min
        assert_eq!(Temperature::default().0, 0.7);
    }

    #[test]
    fn test_top_p() {
        assert_eq!(TopP::new(0.5).0, 0.5);
        assert_eq!(TopP::new(1.5).0, 1.0); // Should clamp to max
        assert_eq!(TopP::new(-0.5).0, 0.0); // Should clamp to min
        assert_eq!(TopP::default().0, 1.0);
    }

    #[test]
    fn test_top_log_probs() {
        assert_eq!(TopLogProbs::new(10).0, 10);
        assert_eq!(TopLogProbs::new(25).0, 20); // Should clamp to max
    }

    #[test]
    fn test_message() {
        let user_msg = Message::new_user_message("Hello".to_string());
        assert!(matches!(user_msg.role, Role::User));
        assert_eq!(user_msg.content, "Hello");
        assert!(user_msg.name.is_none());

        let system_msg =
            Message::new_system_message_with_name("System message".to_string(), "sys".to_string());
        assert!(matches!(system_msg.role, Role::System));
        assert_eq!(system_msg.content, "System message");
        assert_eq!(system_msg.name, Some("sys".to_string()));

        let assistant_msg = Message::new_assistant_message("Response".to_string());
        assert!(matches!(assistant_msg.role, Role::Assistant));
        assert_eq!(assistant_msg.content, "Response");
        assert!(assistant_msg.name.is_none());
    }

    #[test]
    fn test_stop_type() {
        let single_stop = StopType::Stop("stop".to_string());
        let multi_stop = StopType::StopArray(vec!["stop1".to_string(), "stop2".to_string()]);

        match single_stop {
            StopType::Stop(s) => assert_eq!(s, "stop"),
            _ => panic!("Expected Stop variant"),
        }

        match multi_stop {
            StopType::StopArray(arr) => {
                assert_eq!(arr.len(), 2);
                assert_eq!(arr[0], "stop1");
                assert_eq!(arr[1], "stop2");
            }
            _ => panic!("Expected StopArray variant"),
        }
    }

    #[test]
    fn test_request_body_builder() {
        let messages = vec![Message::new_user_message("test".to_string())];

        // Test new
        let req = RequestBody::new(messages.clone(), Model::DeepseekChat);
        assert!(matches!(req.model, Model::DeepseekChat));
        assert_eq!(req.messages.len(), 1);

        // Test new_messages
        let req = RequestBody::new_messages(messages.clone());
        assert!(matches!(req.model, Model::DeepseekChat)); // Default model
        assert_eq!(req.messages.len(), 1);

        // Test builder methods
        let req = RequestBody::new_messages(messages)
            .with_model(Model::DeepSeekReasoner)
            .with_frequency_penalty(FrequencyPenalty::new(1))
            .with_max_tokens(MaxTokens::new(100))
            .with_presence_penalty(PresencePenalty::new(1))
            .with_response_format(ResponseFormat::new(ResponseFormatType::Json))
            .with_stop(StopType::Stop("stop".to_string()))
            .with_stream(true)
            .with_stream_options(StreamOptions {
                include_usage: true,
            })
            .with_temperature(Temperature::new(0.8))
            .with_top_p(TopP::new(0.9))
            .with_logprobs(true)
            .with_top_logprobs(TopLogProbs::new(5));

        // Verify all fields are set correctly
        assert!(matches!(req.model, Model::DeepSeekReasoner));
        assert_eq!(req.frequency_penalty.unwrap().to_string(), "1");
        assert_eq!(req.max_tokens.unwrap().to_string(), "100");
        assert_eq!(req.presence_penalty.unwrap().to_string(), "1");
        assert!(req.response_format.is_some());
        assert!(matches!(req.stop, Some(StopType::Stop(_))));
        assert_eq!(req.stream, Some(true));
        assert!(req.stream_options.unwrap().include_usage);
        assert_eq!(req.temperature.unwrap().to_string(), "0.8");
        assert_eq!(req.top_p.unwrap().to_string(), "0.9");
        assert_eq!(req.logprobs, Some(true));
        assert_eq!(req.top_logprobs.unwrap().0, 5);
    }

    #[test]
    fn test_request_body_default() {
        let req = RequestBody::default();
        assert!(req.messages.is_empty());
        assert!(matches!(req.model, Model::DeepseekChat));
        assert!(req.frequency_penalty.is_none());
        assert!(req.max_tokens.is_none());
        assert!(req.presence_penalty.is_none());
        assert!(req.response_format.is_none());
        assert!(req.stop.is_none());
        assert!(req.stream.is_none());
        assert!(req.stream_options.is_none());
        assert!(req.temperature.is_none());
        assert!(req.top_p.is_none());
        assert!(req.logprobs.is_none());
        assert!(req.top_logprobs.is_none());
    }
}
