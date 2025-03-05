use ai_sdk::{get_deepseek_api_key, AiAdapter, AiService, DeepSeekAdapter};
use chat_core::{AdapterType, Agent, AgentContext, AgentDecision, AgentType, ChatAgent};

#[allow(unused)]
#[derive(Debug)]
pub enum AgentVariant {
    Replay(ReplyAgent),
    Tap(TapAgent),
    Proxy(ProxyAgent),
}

#[allow(unused)]
#[derive(Debug)]
pub struct ProxyAgent {
    pub name: String,
    pub adapter: AiAdapter,
    pub prompt: String,
    pub args: serde_json::Value,
}

#[allow(unused)]
#[derive(Debug)]
pub struct ReplyAgent {
    pub name: String,
    pub adapter: AiAdapter,
    pub prompt: String,
    pub args: serde_json::Value,
}

#[allow(unused)]
#[derive(Debug)]
pub struct TapAgent {
    pub name: String,
    pub adapter: AiAdapter,
    pub prompt: String,
    pub args: serde_json::Value,
}

impl Agent for ProxyAgent {
    async fn process(
        &self,
        msg: &str,
        _ctx: &AgentContext,
    ) -> Result<chat_core::AgentDecision, chat_core::AgentError> {
        let prompt = format!("{}: {}", self.prompt, msg);
        let messages = vec![ai_sdk::Message::user(prompt)];
        let response = self.adapter.complete(&messages).await?;
        let decision = AgentDecision::Modify(response);
        Ok(decision)
    }
}

impl Agent for ReplyAgent {
    async fn process(
        &self,
        msg: &str,
        _ctx: &AgentContext,
    ) -> Result<chat_core::AgentDecision, chat_core::AgentError> {
        let messages = vec![ai_sdk::Message::user(msg)];
        let response = self.adapter.complete(&messages).await?;
        let decision = AgentDecision::Reply(response);
        Ok(decision)
    }
}

impl Agent for TapAgent {
    async fn process(
        &self,
        _msg: &str,
        _ctx: &AgentContext,
    ) -> Result<chat_core::AgentDecision, chat_core::AgentError> {
        Ok(AgentDecision::None)
    }
}

impl Agent for AgentVariant {
    async fn process(
        &self,
        msg: &str,
        ctx: &AgentContext,
    ) -> Result<AgentDecision, chat_core::AgentError> {
        match self {
            AgentVariant::Replay(agent) => agent.process(msg, ctx).await,
            AgentVariant::Tap(agent) => agent.process(msg, ctx).await,
            AgentVariant::Proxy(agent) => agent.process(msg, ctx).await,
        }
    }
}

impl From<ChatAgent> for AgentVariant {
    fn from(value: ChatAgent) -> Self {
        let adapter = match value.adapter {
            AdapterType::Deepseek => {
                let api_key = get_deepseek_api_key();
                DeepSeekAdapter::new(api_key, value.model).into()
            }
        };
        match value.r#type {
            AgentType::Reply => AgentVariant::Replay(ReplyAgent {
                name: value.name,
                adapter: adapter,
                prompt: value.prompt,
                args: value.args,
            }),
            AgentType::Tap => AgentVariant::Tap(TapAgent {
                name: value.name,
                adapter: adapter,
                prompt: value.prompt,
                args: value.args,
            }),
            AgentType::Proxy => AgentVariant::Proxy(ProxyAgent {
                name: value.name,
                adapter: adapter,
                prompt: value.prompt,
                args: value.args,
            }),
        }
    }
}

impl From<ProxyAgent> for AgentVariant {
    fn from(value: ProxyAgent) -> Self {
        AgentVariant::Proxy(value)
    }
}

impl From<ReplyAgent> for AgentVariant {
    fn from(value: ReplyAgent) -> Self {
        AgentVariant::Replay(value)
    }
}

impl From<TapAgent> for AgentVariant {
    fn from(value: TapAgent) -> Self {
        AgentVariant::Tap(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{models::CreateAgent, AppState};
    use std::collections::HashMap;

    #[ignore]
    #[tokio::test]
    async fn agent_variant_should_work() -> anyhow::Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let input = CreateAgent::new(
            AdapterType::Deepseek,
            "deepseek-chat",
            "test",
            AgentType::Proxy,
            "You are a helpful assistant",
            HashMap::<String, String>::new(),
        );
        let agent = state
            .create_agent(input, 1)
            .await
            .expect("create chat failed");
        let agent: AgentVariant = agent.into();

        let decision = agent.process("Hello", &AgentContext::new()).await?;
        if let AgentDecision::Modify(_) = decision {
            Ok(())
        } else {
            panic!("Expected Modify decision")
        }
    }
}
