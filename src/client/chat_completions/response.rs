use serde::{Deserialize, Serialize};

use super::request::Role;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]

pub struct ChatCompletionsResponse {
    pub id: String,
    pub choices: Vec<ChatCompletionsChoices>,
    pub created: i32, // Unix timestamp in seconds
    pub model: String,
    pub object: String, //TODO: add enum
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum FinishReasons {
    // Possible values: [stop, length, content_filter, tool_calls, insufficient_system_resource]
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "content_filter")]
    ContentFilter,
    #[serde(rename = "tool_calls")]
    ToolCalls,
    #[serde(rename = "insufficient_system_resource")]
    InsufficientSystemResource,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ChatCompletionsChoices {
    pub finish_reason: FinishReasons,
    pub index: i32,
    pub message: Message,
    //TODO: add log_prob
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Message {
    pub content: Option<String>,
    pub reasoning_content: Option<String>,
    pub role: Role,
    pub tool_calls: Option<Vec<ToolsCall>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ToolsCall {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "function")]
    pub function_call: FunctionCall,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Usage {
    pub completion_tokens: i32,
    pub prompt_tokens: i32,
    pub prompt_cache_hit_tokens: i32,
    pub prompt_cache_miss_tokens: i32,
    pub total_tokens: i32,
    pub completion_tokens_details: Option<CompletionTokensDetails>,
    pub prompt_tokens_details: Option<PromptTokensDetails>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]

pub struct CompletionTokensDetails {
    pub reasoning_tokens: i32,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]

pub struct PromptTokensDetails {
    pub cached_tokens: i32,
}
