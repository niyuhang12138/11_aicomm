mod middlewares;
mod util;

use chrono::{DateTime, Utc};
use jwt_simple::reexports::serde_json;
pub use middlewares::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use thiserror::Error;
pub use util::*;
use utoipa::ToSchema;

#[allow(async_fn_in_trait)]
pub trait Agent: std::fmt::Debug {
    async fn process(&self, msg: &str, ctx: &AgentContext) -> Result<AgentDecision, AgentError>;
}

#[derive(Debug, Clone)]
pub struct AgentContext {}

impl AgentContext {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone)]
pub enum AgentDecision {
    Modify(String),
    Reply(String),
    Delete,
    None,
}

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("{0}")]
    AnyError(#[from] anyhow::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, ToSchema)]
pub struct User {
    pub id: i64,
    pub ws_id: i64,
    #[sqlx(default)]
    pub ws_name: String,
    pub fullname: String,
    pub email: String,
    #[sqlx(default)]
    #[serde(skip)]
    pub password_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, ToSchema)]
pub struct Workspace {
    pub id: i64,
    pub name: String,
    pub owner_id: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, ToSchema)]
pub struct ChatUser {
    pub id: i64,
    pub fullname: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize, PartialEq, sqlx::Type, ToSchema)]
#[sqlx(type_name = "chat_type", rename_all = "snake_case")]
pub enum ChatType {
    #[default]
    Single,
    Group,
    PrivateChannel,
    PublicChannel,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, ToSchema)]
pub struct Chat {
    pub id: i64,
    pub ws_id: i64,
    pub name: Option<String>,
    pub r#type: ChatType,
    pub members: Vec<i64>,
    pub agents: Vec<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, ToSchema)]
pub struct Message {
    pub id: i64,
    pub chat_id: i64,
    pub sender_id: i64,
    pub content: String,
    #[sqlx(default)]
    pub modified_content: Option<String>,
    pub files: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, ToSchema, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "agent_type", rename_all = "snake_case")]
pub enum AgentType {
    #[serde(alias = "proxy", alias = "Proxy")]
    #[default]
    Proxy,
    #[serde(alias = "replay", alias = "Reply")]
    Reply,
    #[serde(alias = "tap", alias = "Tap")]
    Tap,
}

#[derive(Debug, Clone, Default, ToSchema, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "adapter_type", rename_all = "snake_case")]
pub enum AdapterType {
    #[default]
    #[serde(alias = "deepseek", alias = "DeepSeek")]
    Deepseek,
}

#[derive(Debug, Clone, FromRow, ToSchema, Serialize, Deserialize, PartialEq)]
pub struct ChatAgent {
    pub id: i64,
    pub chat_id: i64,
    pub name: String,
    pub r#type: AgentType,
    pub adapter: AdapterType,
    pub model: String,
    pub prompt: String,
    pub args: serde_json::Value, // TODO: change to custom type
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: i64, fullname: &str, email: &str) -> Self {
        Self {
            id,
            ws_id: 0,
            ws_name: "".to_string(),
            fullname: fullname.to_string(),
            email: email.to_string(),
            password_hash: None,
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn core_test() {}
}
