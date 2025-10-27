use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};

use crate::{AppState, dto::member_dto::{CreateMemberRequest, MemberResponse}};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_member))
        .route("/:id", get(get_member))
}

// POST /members
async fn create_member(
    State(state): State<AppState>,
    Json(payload): Json<CreateMemberRequest>,
) -> Result<Json<(MemberResponse)>, axum::http::StatusCode> {
    let member = state.member_service
        .register_member(payload.name, payload.email)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(MemberResponse::from_domain(&member)))
}

// GET /members/:id
async fn get_member(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<MemberResponse>, axum::http::StatusCode> {
    let member = state.member_service
        .find_member_by_id(id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    match member {
        Some(m) => Ok(Json(MemberResponse::from_domain(&m))),
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
}
