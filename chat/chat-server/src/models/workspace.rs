use crate::{AppError, AppState};
use chat_core::Workspace;
use sqlx::query_as;

impl AppState {
    pub async fn create_workspace(&self, name: &str, user_id: u16) -> Result<Workspace, AppError> {
        let ws = query_as(
        "INSERT INTO workspaces (name, owner_id) VALUES ($1, $2) RETURNING id, name, owner_id, created_at"
      )
      .bind(name)
      .bind(user_id as i64)
      .fetch_one(&self.pool)
      .await?;

        Ok(ws)
    }

    pub async fn find_workspace_by_name(&self, name: &str) -> Result<Option<Workspace>, AppError> {
        let ws = query_as("SELECT id, name, owner_id, created_at FROM workspaces WHERE name = $1")
            .bind(name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(ws)
    }

    #[allow(dead_code)]
    pub async fn find_workspace_by_id(&self, id: u64) -> Result<Option<Workspace>, AppError> {
        let ws = query_as(
            "SELECT id, name, owner_id, created_at FROM workspaces WHERE id = $1 ORDER BY id",
        )
        .bind(id as i64)
        .fetch_optional(&self.pool)
        .await?;

        Ok(ws)
    }

    pub async fn update_workspace_owner(
        &self,
        ws_id: u64,
        owner_id: u64,
    ) -> Result<Workspace, AppError> {
        let ws = query_as(
            "UPDATE workspaces SET owner_id = $1 WHERE id = $2 and (SELECT ws_id FROM users WHERE id = $1) = $2 RETURNING id, name, owner_id, created_at"
          )
          .bind(owner_id as i64)
          .bind(ws_id as i64)
          .fetch_one(&self.pool)
          .await?;

        Ok(ws)
    }
}

#[cfg(test)]
mod tests {

    use crate::models::CreateUser;

    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn workspace_should_create_and_set_owned() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let input = CreateUser::new("test", "alice1", "alice1@gmail.com", "alice123");
        let user = state.create_user(&input).await?;
        let ws = state.find_workspace_by_name("test").await?.unwrap();
        assert_eq!(ws.name, "test");
        assert_eq!(user.ws_id, ws.id);
        assert_eq!(ws.owner_id, user.id);
        Ok(())
    }

    #[tokio::test]
    async fn workspace_should_find_by_name() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let ws = state.find_workspace_by_name("acme").await?;
        assert_eq!(ws.unwrap().name, "acme");

        Ok(())
    }

    #[tokio::test]
    async fn workspace_should_fetch_all_chat_users() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let users = state.fetch_chat_user_all(1).await?;
        assert_eq!(users.len(), 6);

        Ok(())
    }
}
