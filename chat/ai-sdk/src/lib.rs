mod adapters;
mod util;

pub use adapters::*;
use enum_dispatch::enum_dispatch;
pub use util::*;

#[derive(Debug)]
#[enum_dispatch(AiService)]
pub enum AiAdapter {
    DeepSeek(DeepSeekAdapter),
}

#[derive(Debug, Clone)]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait AiService {
    async fn complete(&self, messages: &[Message]) -> anyhow::Result<String>;
    // other functions...
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
            Role::System => write!(f, "system"),
        }
    }
}

impl Message {
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self::new(Role::User, content)
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new(Role::Assistant, content)
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self::new(Role::System, content)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ai_sdk_should_work() {}
}
