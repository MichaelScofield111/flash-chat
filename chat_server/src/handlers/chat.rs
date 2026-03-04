use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    AppError, AppState,
    model::{Chat, User},
};

pub(crate) async fn list_chat_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let chat = Chat::fetch_all(user.ws_id as _, &state.pool).await?;
    Ok((StatusCode::OK, Json(chat)))
}

pub(crate) async fn create_chat_handler() -> impl IntoResponse {
    "create chat"
}

pub(crate) async fn update_chat_handler() -> impl IntoResponse {
    "update chat"
}

pub(crate) async fn delete_chat_handler() -> impl IntoResponse {
    "delete chat"
}
