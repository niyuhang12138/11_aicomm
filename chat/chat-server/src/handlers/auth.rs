use crate::{
    models::{CreateUser, SigninUser},
    AppError, AppState, ErrorOutput,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthOutput {
    token: String,
}

#[utoipa::path(
    post,
    path = "/api/signup",
    responses(
        (status = 200, description = "User created", body = AuthOutput)
    )
)]
pub(crate) async fn signup_handler(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.create_user(&input).await?;
    let token = state.ek.sign(user)?;
    let body = Json(AuthOutput { token });
    Ok((StatusCode::OK, body))
}

#[utoipa::path(
    post,
    path = "/api/signin",
    responses(
        (status = 200, description = "User Login", body = AuthOutput)
    )
)]
pub(crate) async fn signin_handler(
    State(state): State<AppState>,
    Json(input): Json<SigninUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.verify_user(&input).await?;

    match user {
        Some(user) => {
            let token = state.ek.sign(user)?;
            Ok((StatusCode::OK, Json(AuthOutput { token })).into_response())
        }
        None => {
            let body = Json(ErrorOutput::new("Invalid email or password"));
            Ok((StatusCode::FORBIDDEN, body).into_response())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn signup_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let input = CreateUser::new("none", "nyh", "nyh@qq.com", "nyh1111");
        let ret = signup_handler(State(state), Json(input))
            .await?
            .into_response();
        assert_eq!(ret.status(), StatusCode::OK);
        let body = ret.into_body().collect().await?.to_bytes();
        let ret: AuthOutput = serde_json::from_slice(&body)?;
        assert_ne!(ret.token, "");
        Ok(())
    }

    #[tokio::test]
    async fn signup_already_exists_user_should_409() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let input = CreateUser::new("none", "alice", "alice@qq.com", "alice123");
        signup_handler(State(state.clone()), Json(input.clone())).await?;

        let ret = signup_handler(State(state), Json(input))
            .await
            .into_response();

        assert_eq!(ret.status(), StatusCode::CONFLICT);
        let body = ret.into_body().collect().await?.to_bytes();
        let ret: ErrorOutput = serde_json::from_slice(&body)?;
        assert_eq!(ret.error, "email already exists: alice@qq.com");
        Ok(())
    }

    #[tokio::test]
    async fn signin_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let name = "Alice";
        let email = "alice@qq.com";
        let password = "alice123";
        let user = CreateUser::new("none", name, email, password);
        state.create_user(&user).await?;
        let input = SigninUser::new(email, password);
        let ret = signin_handler(State(state), Json(input))
            .await?
            .into_response();
        assert_eq!(ret.status(), StatusCode::OK);
        let body = ret.into_body().collect().await?.to_bytes();
        let ret: AuthOutput = serde_json::from_slice(&body)?;
        assert_ne!(ret.token, "");
        Ok(())
    }

    #[tokio::test]
    async fn signin_invalid_email_should_403() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let email = "alice@qq.com";
        let password = "alice123";
        let input = SigninUser::new(email, password);
        let ret = signin_handler(State(state), Json(input))
            .await
            .into_response();

        assert_eq!(ret.status(), StatusCode::FORBIDDEN);
        let body = ret.into_body().collect().await?.to_bytes();
        let ret: ErrorOutput = serde_json::from_slice(&body)?;
        assert_eq!(ret.error, "Invalid email or password");
        Ok(())
    }
}
