use crate::{
    AppError, AppState,
    models::{CreateAgent, UpdateAgent},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

/// List all agents in the chat.
#[utoipa::path(
    get,
    path = "/api/chats/{id}/agents",
    params(
        ("id" = u64, Path, description = "Chat id")
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn list_agent_handler(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let agents = state.list_agents(id as _).await?;
    Ok((StatusCode::OK, Json(agents)))
}

/// Create a new agent in the chat.
#[utoipa::path(
    post,
    path = "/api/chats/{id}/agents",
    params(
        ("id" = u64, Path, description = "Chat id")
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn create_agent_handler(
    Path(id): Path<u64>,
    State(state): State<AppState>,
    Json(input): Json<CreateAgent>,
) -> Result<impl IntoResponse, AppError> {
    let agent = state.create_agent(input, id).await?;
    Ok((StatusCode::CREATED, Json(agent)))
}

/// Update the agent by id.
#[utoipa::path(
    patch,
    path = "/api/chats/{id}/agents/{agent_id}",
    params(
        ("id" = u64, Path, description = "Chat id"),
        ("agent_id" = u64, Path, description = "Agent id")
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn update_agent_handler(
    Path(id): Path<u64>,
    State(state): State<AppState>,
    Json(input): Json<UpdateAgent>,
) -> Result<impl IntoResponse, AppError> {
    let agent = state.update_agent(input, id as _).await?;
    Ok((StatusCode::OK, Json(agent)))
}
