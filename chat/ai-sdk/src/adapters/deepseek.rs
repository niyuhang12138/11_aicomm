//! deepseek response
//! {
//!   "id": "a6250b7d-ed69-4e1e-983e-8b501d807403",
//!   "object": "chat.completion",
//!   "created": 1741052005,
//!   "model": "deepseek-chat",
//!   "choices": [
//!     {
//!       "index": 0,
//!       "message": {
//!         "role": "assistant",
//!         "content": "中国最长的河流是长江，全长约6300公里。"
//!       },
//!       "logprobs": null,
//!       "finish_reason": "stop"
//!     }
//!   ],
//!   "usage": {
//!     "prompt_tokens": 48,
//!     "completion_tokens": 13,
//!     "total_tokens": 61,
//!     "prompt_tokens_details": {
//!       "cached_tokens": 0
//!     },
//!     "prompt_cache_hit_tokens": 0,
//!     "prompt_cache_miss_tokens": 48
//!   },
//!   "system_fingerprint": "fp_3a5770e1b4_prod0225"
//! }

use crate::AiService;
use crate::Message;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct DeepSeekAdapter {
    host: String,
    api_key: String,
    model: String,
    client: Client,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeepSeekChatCompletionRequest {
    pub model: String,
    pub messages: Vec<DeepSeekMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepSeekMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepSeekChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<DeepSeekChoice>,
    pub usage: DeepSeekUsage,
    pub system_fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepSeekChoice {
    pub index: u32,
    pub message: DeepSeekMessage,
    pub logprobs: Option<i64>,
    pub finish_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepSeekUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    pub prompt_tokens_details: DeepSeekPromptTokensDetails,
    pub prompt_cache_hit_tokens: u32,
    pub prompt_cache_miss_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepSeekPromptTokensDetails {
    pub cached_tokens: u32,
}

impl DeepSeekAdapter {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            host: "https://api.deepseek.com".to_string(),
            api_key: api_key.into(),
            model: model.into(),
            client: Client::new(),
        }
    }
}

impl AiService for DeepSeekAdapter {
    async fn complete(&self, messages: &[Message]) -> anyhow::Result<String> {
        let request = DeepSeekChatCompletionRequest {
            model: self.model.clone(),
            messages: messages.iter().map(|m| m.into()).collect(),
        };
        let url = format!("{}/chat/completions", self.host);
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;
        let mut data = response.json::<DeepSeekChatCompletionResponse>().await?;
        let content = data
            .choices
            .pop()
            .ok_or(anyhow::anyhow!("No response"))?
            .message
            .content;
        Ok(content)
    }
}

impl From<&Message> for DeepSeekMessage {
    fn from(value: &Message) -> Self {
        Self {
            role: value.role.to_string(),
            content: value.content.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Role;
    use crate::get_deepseek_api_key;

    #[ignore]
    #[tokio::test]
    async fn deepseek_complete_should_work() {
        let api_key = get_deepseek_api_key();
        let adapter = DeepSeekAdapter::new(api_key, "deepseek-chat");
        let messages = vec![Message {
            role: Role::User,
            content: "中国最长的河流是".to_string(),
        }];
        let responnse = adapter.complete(&messages).await.unwrap();
        assert!(responnse.len() > 0)
    }
}
