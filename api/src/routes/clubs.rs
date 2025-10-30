use axum::{
    Json, Router, debug_handler, extract::{Path, State}, routing::{get, post}
};

use crate::{AppState, dto::club_dto::{ClubResponse, CreateClubRequest}};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_club))
        .route("/", get(get_all_clubs))
        .route("/{id}", get(get_club))
}

// POST /club
#[debug_handler]
async fn create_club(
    State(state): State<AppState>,
    Json(payload): Json<CreateClubRequest>,
) -> Result<Json<ClubResponse>, axum::http::StatusCode> {
    let club = state.club_service
        .register_club(payload.name)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ClubResponse::from_domain(&club)))
}

// GET /members/:id
async fn get_club(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ClubResponse>, axum::http::StatusCode> {
    let club = state.club_service
        .find_club_by_id(id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    match club {
        Some(c) => Ok(Json(ClubResponse::from_domain(&c))),
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
}

async fn get_all_clubs(
    State(state): State<AppState>,
) -> Result<Json<Vec<ClubResponse>>, axum::http::StatusCode> {
    let clubs = state.club_service
        .list_all_clubs()
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let response: Vec<ClubResponse> = clubs.iter().map(|c| ClubResponse::from_domain(c)).collect();

    Ok(Json(response))
}
